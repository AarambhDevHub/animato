import init, { Tween, RafDriver, initAnimato } from "@animato/core";
import { useEffect, useRef, useState } from "react";

export function App() {
  const [x, setX] = useState(0);
  const tweenRef = useRef(null);

  useEffect(() => {
    let frame = 0;
    let cancelled = false;

    async function start() {
      await init();
      initAnimato();
      const tween = new Tween(0, 320, 0.9);
      tween.setEasing("easeOutCubic");
      const driver = new RafDriver();
      driver.addTween(tween);
      tweenRef.current = tween;

      function tick(now) {
        if (cancelled) return;
        driver.tick(now);
        setX(tween.value());
        if (!tween.isComplete()) frame = requestAnimationFrame(tick);
      }

      frame = requestAnimationFrame(tick);
    }

    start();
    return () => {
      cancelled = true;
      cancelAnimationFrame(frame);
    };
  }, []);

  return <div className="ball" style={{ transform: `translateX(${x}px)` }} />;
}
