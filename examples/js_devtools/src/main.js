import init, {
  DevToolsState,
  EasingCurveEditor,
  PerformanceMonitor,
  RafDriver,
  SpringVisualizer,
  TimelineInspector,
  Tween,
  initAnimato,
} from "@aarambhdevhub/animato-core";

await init();
initAnimato();

const tween = new Tween(0, 1, 1);
const driver = new RafDriver();
driver.addTween(tween);
driver.tick(1000);
driver.tick(1250);

const inspector = new TimelineInspector();
inspector.captureRafDriver(driver);

const easing = new EasingCurveEditor("easeOutCubic");
easing.setSampleCount(8);

const spring = new SpringVisualizer();
spring.setPreset("snappy");
spring.simulate(1, 1 / 60, 90);

const perf = new PerformanceMonitor(8);
perf.recordFrame(1 / 60);

const state = new DevToolsState();
state.toggle();

document.querySelector("[data-output]").textContent = JSON.stringify({
  snapshots: inspector.snapshotCount(),
  kind: inspector.snapshotKind(0),
  progress: inspector.snapshotProgress(0),
  easingSamples: easing.samplePoints().length,
  springFrames: spring.frameCount(),
  fps: perf.fps(),
  open: state.isOpen(),
});
