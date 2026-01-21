import init, * as wasm from "../wasm/pkg/wasm_cube";
import "./style.css";

async function main() {
  await init();

  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const viewer = new wasm.Viewer(canvas);
  const primitiveSelect = document.getElementById("primitive") as HTMLSelectElement | null;
  const renderModeSelect = document.getElementById("renderMode") as HTMLSelectElement | null;
  const viewModeSelect = document.getElementById("viewMode") as HTMLSelectElement | null;

  const resize = () => {
    // Match drawing buffer to CSS size for crisp rendering.
    const rect = canvas.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    const w = Math.max(1, Math.floor(rect.width * dpr));
    const h = Math.max(1, Math.floor(rect.height * dpr));
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w;
      canvas.height = h;
      viewer.resize(w, h);
      viewer.fit_to_view();
      viewer.draw();
    }
  };

  resize();
  window.addEventListener("resize", resize);

  if (primitiveSelect) {
    const applyPrimitive = () => {
      viewer.set_primitive(primitiveSelect.value);
      viewer.fit_to_view();
      viewer.draw();
    };
    primitiveSelect.addEventListener("change", applyPrimitive);
    applyPrimitive();
  }

  if (renderModeSelect) {
    const applyRenderMode = () => {
      viewer.set_render_mode(renderModeSelect.value);
      viewer.draw();
    };
    renderModeSelect.addEventListener("change", applyRenderMode);
    applyRenderMode();
  }

  if (viewModeSelect) {
    const applyViewMode = () => {
      viewer.set_view_mode(viewModeSelect.value);
      viewer.draw();
    };
    viewModeSelect.addEventListener("change", applyViewMode);
    applyViewMode();
  }

  let isDragging = false;
  let lastX = 0;
  let lastY = 0;
  let dragButton = 0; // 0 left rotate, 1 middle pan, 2 right pan

  const ROTATE_SPEED = 0.01;
  const PAN_SPEED = 0.002;

  canvas.addEventListener("contextmenu", (e) => e.preventDefault());

  canvas.addEventListener("mousedown", (e) => {
    isDragging = true;
    dragButton = e.button;
    lastX = e.clientX;
    lastY = e.clientY;
  });

  window.addEventListener("mouseup", () => {
    isDragging = false;
  });

  window.addEventListener("mousemove", (e) => {
    if (!isDragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    lastX = e.clientX;
    lastY = e.clientY;

    if (dragButton === 0) {
      viewer.rotate(dx * ROTATE_SPEED, -dy * ROTATE_SPEED);
    } else {
      viewer.pan(-dx * PAN_SPEED, dy * PAN_SPEED);
    }
    viewer.draw();
  });

  canvas.addEventListener(
    "wheel",
    (e) => {
      e.preventDefault();
      // smooth exponential zoom
      const factor = Math.exp(e.deltaY * 0.001);
      viewer.zoom(factor);
      viewer.draw();
    },
    { passive: false }
  );
}

main();
