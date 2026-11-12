var window = globalThis;
var posted = [];
window.wprsHost = { post: (m) => posted.push(JSON.parse(m).sounding) };
window.requestAnimationFrame = () => 0;
var document = { addEventListener: () => {}, querySelectorAll: () => [] };
class Target {
  constructor() {
    this.listeners = {};
  }
  addEventListener(kind, f) {
    (this.listeners[kind] = this.listeners[kind] || []).push(f);
  }
  fire(kind) {
    (this.listeners[kind] || []).forEach((f) => f({ target: this }));
  }
}
globalThis.HTMLMediaElement = class extends Target {
  constructor() {
    super();
    this.paused = true;
    this.ended = false;
    this.muted = false;
    this.volume = 1;
  }
  play() {
    this.paused = false;
    this.fire("playing");
    return Promise.resolve();
  }
  pause() {
    this.paused = true;
    this.fire("pause");
  }
};
globalThis.AudioContext = class extends Target {
  constructor() {
    super();
    this.state = "running";
  }
  suspend() {
    this.state = "suspended";
    this.fire("statechange");
  }
};
var wprs = { fps: 60, volume: 1, onSettings: () => {} };
