<script>
  import init, { Spring2D, initAnimato } from "@aarambhdevhub/animato-core";

  let x = 0;
  let y = 0;
  let spring;

  async function boot() {
    await init();
    initAnimato();
    spring = new Spring2D(0, 0, 180, 80);
    spring.setTarget(180, 80);
    requestAnimationFrame(tick);
  }

  function tick() {
    spring.update(1 / 60);
    const values = spring.toArray();
    x = values[0];
    y = values[1];
    if (!spring.isSettled()) requestAnimationFrame(tick);
  }

  boot();
</script>

<div class="panel" style:transform={`translate(${x}px, ${y}px)`}></div>
