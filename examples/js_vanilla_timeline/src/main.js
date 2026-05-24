import init, { Timeline, Tween, MotionPath, ColorTween, initAnimato } from "@animato/core";

await init();
initAnimato();

const box = document.querySelector("[data-box]");
const path = new MotionPath("M 0 0 C 120 80 220 -80 340 0", 1.2);
const fade = new Tween(0, 1, 0.45);
const color = new ColorTween("#16a34a", "#0ea5e9", 1.2, "oklch");
const timeline = new Timeline();

timeline.addMotionPath("path", path, "start");
timeline.addTween("fade", fade, "start");
timeline.play();

let last = performance.now();
function frame(now) {
  const dt = (now - last) / 1000;
  last = now;
  timeline.update(dt);
  color.update(dt);

  box.style.opacity = String(fade.value());
  box.style.background = color.valueHex();
  box.style.transform = `translate(${path.x()}px, ${path.y()}px) rotate(${path.rotationDeg()}deg)`;

  if (!timeline.isComplete()) requestAnimationFrame(frame);
}

requestAnimationFrame(frame);
