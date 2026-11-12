(() => {
  const native = window.requestAnimationFrame.bind(window);
  let fps = wprs.fps || 60;
  let last = 0;
  let pending = false;
  let next = 1;
  let queue = new Map();
  const run = (now) => {
    const wait = 1000 / fps - 2 - (now - last);
    if (wait > 0) {
      setTimeout(() => native(run), wait);
      return;
    }
    pending = false;
    last = now;
    const due = queue;
    queue = new Map();
    due.forEach((cb) => {
      try {
        cb(now);
      } catch (e) {
        setTimeout(() => { throw e; });
      }
    });
  };
  window.requestAnimationFrame = (cb) => {
    queue.set(next, cb);
    if (!pending) {
      pending = true;
      native(run);
    }
    return next++;
  };
  window.cancelAnimationFrame = (id) => queue.delete(id);
  const contexts = [];
  const live = new Set();
  const watched = new WeakSet();
  let said = null;
  const say = () => {
    const now = live.size > 0 || contexts.some((c) => c.state === "running");
    if (now === said || !window.wprsHost) return;
    said = now;
    window.wprsHost.post(JSON.stringify({ sounding: now }));
  };
  const heard = (event) => {
    const e = event.target;
    if (!e.paused && !e.ended && !e.muted && e.volume > 0) live.add(e);
    else live.delete(e);
    say();
  };
  const watch = (element) => {
    if (!(element instanceof HTMLMediaElement) || watched.has(element)) return;
    watched.add(element);
    ["playing", "pause", "ended", "emptied", "volumechange"].forEach((kind) =>
      element.addEventListener(kind, heard),
    );
  };
  const play = HTMLMediaElement.prototype.play;
  HTMLMediaElement.prototype.play = function (...args) {
    watch(this);
    return play.apply(this, args);
  };
  const Real = window.AudioContext;
  if (Real) {
    window.AudioContext = function (...args) {
      const made = new Real(...args);
      contexts.push(made);
      made.addEventListener("statechange", say);
      if (wprs.volume <= 0) made.suspend();
      say();
      return made;
    };
    window.AudioContext.prototype = Real.prototype;
  }
  const silenced = new WeakSet();
  const quiet = (element) => {
    if (wprs.volume <= 0 && !element.muted) {
      element.muted = true;
      silenced.add(element);
    }
  };
  document.addEventListener(
    "play",
    (event) => {
      quiet(event.target);
      watch(event.target);
    },
    true,
  );
  say();
  wprs.onSettings((settings) => {
    fps = settings.fps || 60;
    const muted = settings.volume <= 0;
    contexts.forEach((c) => (muted ? c.suspend() : c.resume()));
    document.querySelectorAll("audio, video").forEach((element) => {
      if (muted) quiet(element);
      else if (silenced.has(element)) {
        element.muted = false;
        silenced.delete(element);
      }
    });
  });
})();
