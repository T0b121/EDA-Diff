/*
KiCad S-expression navigation helpers.

These functions isolate generic tree access from the KiCad PCB/schematic
converters so format parsing stays readable and does not spread lexpr details.
*/

use lexpr::Value;

use crate::adapter::error::AdapterError;
use crate::model::common::Point;

pub fn parse(source: &str) -> Result<Value, AdapterError> {
    lexpr::from_str(source).map_err(|error| {
        AdapterError::InvalidSyntax(format!("Invalid KiCad S-expression: {error}"))
    })
}

pub fn head(value: &Value) -> Option<&str> {
    value.list_iter()?.next()?.as_symbol()
}

pub fn child<'a>(value: &'a Value, name: &str) -> Option<&'a Value> {
    value
        .list_iter()?
        .skip(1)
        .find(|entry| head(entry) == Some(name))
}

pub fn children<'a>(value: &'a Value, name: &'a str) -> impl Iterator<Item = &'a Value> {
    value
        .list_iter()
        .into_iter()
        .flatten()
        .skip(1)
        .filter(move |entry| head(entry) == Some(name))
}

pub fn argument(value: &Value, index: usize) -> Option<&Value> {
    value.list_iter()?.nth(index + 1)
}

pub fn text(value: &Value) -> Option<&str> {
    value.as_str().or_else(|| value.as_symbol())
}

pub fn number(value: &Value) -> Option<f64> {
    value.as_f64()
}

pub fn child_text<'a>(value: &'a Value, name: &str) -> Option<&'a str> {
    text(argument(child(value, name)?, 0)?)
}

pub fn child_number(value: &Value, name: &str) -> Option<f64> {
    number(argument(child(value, name)?, 0)?)
}

pub fn point(value: &Value, name: &str) -> Option<Point> {
    let node = child(value, name)?;
    Some(Point::new(
        number(argument(node, 0)?)?,
        number(argument(node, 1)?)?,
    ))
}

pub fn required_point(value: &Value, name: &str) -> Result<Point, AdapterError> {
    point(value, name)
        .ok_or_else(|| AdapterError::MissingField(format!("Missing KiCad '{name}' coordinates")))
}

pub fn arguments_text(value: &Value) -> Vec<String> {
    value
        .list_iter()
        .into_iter()
        .flatten()
        .skip(1)
        .filter_map(text)
        .map(str::to_owned)
        .collect()
}

pub fn child_rotation(value: &Value, name: &str) -> f64 {
    child(value, name)
        .and_then(|node| argument(node, 2))
        .and_then(number)
        .unwrap_or(0.0)
}

pub fn child_xy_points(value: &Value, name: &str) -> Vec<Point> {
    child(value, name)
        .map(|node| {
            children(node, "xy")
                .filter_map(|xy| {
                    Some(Point::new(
                        number(argument(xy, 0)?)?,
                        number(argument(xy, 1)?)?,
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}
