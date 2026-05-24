import init, { ColorTween, ScrollSmoother, initAnimato } from "@animato/core";
import { Component, ElementRef, OnDestroy, OnInit } from "@angular/core";

@Component({
  selector: "animato-color-demo",
  template: `<div class="swatch"></div>`,
})
export class AppComponent implements OnInit, OnDestroy {
  private frame = 0;
  private tween?: ColorTween;
  private smoother = new ScrollSmoother();

  constructor(private host: ElementRef<HTMLElement>) {}

  async ngOnInit() {
    await init();
    initAnimato();
    this.tween = new ColorTween("#16a34a", "#0ea5e9", 1.2, "oklch");
    this.frame = requestAnimationFrame(this.tick);
  }

  ngOnDestroy() {
    cancelAnimationFrame(this.frame);
  }

  private tick = () => {
    if (!this.tween) return;
    this.tween.update(1 / 60);
    this.smoother.update(1 / 60);
    this.host.nativeElement.style.background = this.tween.valueHex();
    if (!this.tween.isComplete()) this.frame = requestAnimationFrame(this.tick);
  };
}
