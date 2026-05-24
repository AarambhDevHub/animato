<script setup>
import init, { MotionPath, TweenBatch, initAnimato } from "@aarambhdevhub/animato-core";
import { ref, onMounted } from "vue";

const transform = ref("translate(0px, 0px)");
const batchValue = ref(0);

onMounted(async () => {
  await init();
  initAnimato();
  const motion = MotionPath.line(0, 0, 300, 64, 1);
  const batch = new TweenBatch();
  batch.push(0, 1, 1, "easeOutCubic");

  function frame() {
    motion.update(1 / 60);
    batch.tick(1 / 60);
    batchValue.value = batch.value(0);
    transform.value = `translate(${motion.x()}px, ${motion.y()}px)`;
    if (!motion.isComplete()) requestAnimationFrame(frame);
  }

  frame();
});
</script>

<template>
  <div class="card" :style="{ transform, opacity: batchValue }" />
</template>
