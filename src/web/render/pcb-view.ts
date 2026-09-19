/*
Interactive controls for the SVG PCB comparison viewport.

This module owns browser-only pan, zoom, fit, and revision visibility behavior.
It operates on rendered SVG output and does not know any native EDA format.
*/

interface ViewBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export function connectPcbViewControls(container: HTMLElement): void {
  const fit = document.querySelector<HTMLButtonElement>("#pcb-fit");
  const zoomIn = document.querySelector<HTMLButtonElement>("#pcb-zoom-in");
  const zoomOut = document.querySelector<HTMLButtonElement>("#pcb-zoom-out");
  const before = document.querySelector<HTMLInputElement>("#pcb-show-before");
  const after = document.querySelector<HTMLInputElement>("#pcb-show-after");

  if (!fit || !zoomIn || !zoomOut || !before || !after) return;

  let dragging = false;
  let lastX = 0;
  let lastY = 0;

  fit.addEventListener("click", () => fitBoard(container));
  zoomIn.addEventListener("click", () => zoom(container, 0.8));
  zoomOut.addEventListener("click", () => zoom(container, 1.25));

  before.addEventListener("change", () => {
    container.classList.toggle("hide-before", !before.checked);
  });

  after.addEventListener("change", () => {
    container.classList.toggle("hide-after", !after.checked);
  });

  container.addEventListener("wheel", (event) => {
    const svg = currentSvg(container);
    if (!svg) return;

    event.preventDefault();
    const factor = event.deltaY < 0 ? 0.85 : 1.18;
    zoomAt(svg, factor, event.clientX, event.clientY);
  }, { passive: false });

  container.addEventListener("pointerdown", (event) => {
    if (!currentSvg(container)) return;
    dragging = true;
    lastX = event.clientX;
    lastY = event.clientY;
    container.setPointerCapture(event.pointerId);
    container.classList.add("is-panning");
  });

  container.addEventListener("pointermove", (event) => {
    if (!dragging) return;
    const svg = currentSvg(container);
    if (!svg) return;

    const box = readViewBox(svg);
    const rect = svg.getBoundingClientRect();
    const dx = (event.clientX - lastX) * box.width / rect.width;
    const dy = (event.clientY - lastY) * box.height / rect.height;
    setViewBox(svg, {
      x: box.x - dx,
      y: box.y - dy,
      width: box.width,
      height: box.height
    });
    lastX = event.clientX;
    lastY = event.clientY;
  });

  const stopPan = (event: PointerEvent): void => {
    if (!dragging) return;
    dragging = false;
    container.releasePointerCapture(event.pointerId);
    container.classList.remove("is-panning");
  };

  container.addEventListener("pointerup", stopPan);
  container.addEventListener("pointercancel", stopPan);
}

export function resetPcbViewControls(container: HTMLElement): void {
  const before = document.querySelector<HTMLInputElement>("#pcb-show-before");
  const after = document.querySelector<HTMLInputElement>("#pcb-show-after");

  if (before) before.checked = true;
  if (after) after.checked = true;
  container.classList.remove("hide-before", "hide-after", "is-panning");
  fitBoard(container);
}

function fitBoard(container: HTMLElement): void {
  const svg = currentSvg(container);
  const fit = svg?.dataset.fitViewBox;
  if (!svg || !fit) return;
  svg.setAttribute("viewBox", fit);
}

function zoom(container: HTMLElement, factor: number): void {
  const svg = currentSvg(container);
  if (!svg) return;

  const rect = svg.getBoundingClientRect();
  zoomAt(svg, factor, rect.left + rect.width / 2, rect.top + rect.height / 2);
}

function zoomAt(
  svg: SVGSVGElement,
  factor: number,
  clientX: number,
  clientY: number
): void {
  const box = readViewBox(svg);
  const rect = svg.getBoundingClientRect();
  const px = box.x + (clientX - rect.left) / rect.width * box.width;
  const py = box.y + (clientY - rect.top) / rect.height * box.height;
  const width = Math.max(box.width * factor, 0.01);
  const height = Math.max(box.height * factor, 0.01);

  setViewBox(svg, {
    x: px - (px - box.x) * factor,
    y: py - (py - box.y) * factor,
    width,
    height
  });
}

function currentSvg(container: HTMLElement): SVGSVGElement | null {
  return container.querySelector<SVGSVGElement>("svg.pcb-canvas");
}

function readViewBox(svg: SVGSVGElement): ViewBox {
  const [x, y, width, height] = svg
    .getAttribute("viewBox")!
    .split(/\s+/)
    .map(Number);
  return { x, y, width, height };
}

function setViewBox(svg: SVGSVGElement, box: ViewBox): void {
  svg.setAttribute("viewBox", `${box.x} ${box.y} ${box.width} ${box.height}`);
}
