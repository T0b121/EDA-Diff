/*
EDA format adapter module.

All vendor-specific import/export implementations are exposed from here through
one shared contract so domain logic does not depend on native file formats.
*/

pub mod error;
pub mod format_adapter;
pub mod kicad;
