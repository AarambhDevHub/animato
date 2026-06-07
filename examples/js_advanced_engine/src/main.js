import init, {
  AnimationGroup,
  AnimationRecorder,
  Mat4,
  Quaternion,
  QuaternionTween,
  Spring,
  StaggerPattern,
  Tween,
  Waveform,
  initAnimato,
} from "@aarambhdevhub/animato-core";

await init();
initAnimato();

const fling = Spring.fromVelocity(0, 720, 320);
fling.setUnderdamped(180, 0.65);
fling.update(1 / 60);

const wave = Waveform.sine(1, 24, 0);
const waveTrack = wave.toKeyframes(1, 30);
waveTrack.update(0.5);

const start = Quaternion.identity();
const end = Quaternion.fromAxisAngle(0, 1, 0, 180);
const rotate = new QuaternionTween(start, end, 1);
rotate.update(0.5);

const matrix = Mat4.fromTranslationRotationScale(
  new Float32Array([10, 20, 30]),
  rotate.value(),
  new Float32Array([1, 1, 1]),
);

const pattern = StaggerPattern.grid(4, 3, "center", 0.08);
const fade = new Tween(0, 1, 0.3);
const group = new AnimationGroup();
group.addTween("fade", fade, "start");
group.addQuaternionTween("rotate", rotate, "@0.08");
group.play();
group.update(0.5);

const recorder = new AnimationRecorder();
recorder.start();
recorder.record("fling", 0, 0);
recorder.record("fling", 0.5, fling.position());
const replay = AnimationRecorder.importJson(recorder.exportJson()).replay("fling", 0.25);

document.querySelector("[data-output]").textContent = JSON.stringify({
  spring: fling.position(),
  energy: fling.energy(),
  overshoots: fling.overshootCount(),
  wave: wave.sample(0.25),
  keyframe: waveTrack.value(),
  stagger: pattern.delay(5, 12),
  quaternion: Array.from(rotate.toArray()),
  matrix: Array.from(matrix.toArray()).slice(12, 15),
  groupProgress: group.progress(),
  replay,
});
