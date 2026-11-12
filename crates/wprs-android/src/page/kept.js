(allowed, post) => {
  const mark = Symbol.for("wprs.kept");
  if (window[mark]) return;
  window[mark] = true;
  let pending = false;
  const later = typeof setTimeout === "function" ? setTimeout : (run) => run();
  const wrote = () => {
    if (pending) return;
    pending = true;
    later(() => {
      pending = false;
      post({ wrote: true });
    }, 2000);
  };
  const watch = (made, names) => {
    if (typeof made !== "function") return;
    for (const name of names) {
      const real = made.prototype[name];
      if (typeof real !== "function") continue;
      made.prototype[name] = function (...args) {
        wrote();
        return real.apply(this, args);
      };
    }
  };
  watch(window.IDBObjectStore, ["put", "add"]);
  watch(window.IDBCursor, ["update"]);
  watch(window.Cache, ["put", "add", "addAll"]);
  const EVERY = 10000;
  let polling = false;
  const poll = () => {
    if (polling || typeof setInterval !== "function") return;
    polling = true;
    setInterval(() => post({ wrote: true }), EVERY);
  };
  for (const name of ["Worker", "SharedWorker"]) {
    const Real = window[name];
    if (typeof Real !== "function") continue;
    const Wrapped = new Proxy(Real, {
      construct: (target, args, derived) => {
        poll();
        return Reflect.construct(target, args, derived === Wrapped ? target : derived);
      },
    });
    window[name] = Wrapped;
  }
  let store = null;
  try {
    store = window.localStorage;
  } catch (e) {}
  if (!store) return;
  const weight = (key, value) => (value === null ? 0 : (String(key).length + String(value).length) * 2);
  const measure = () => {
    let total = 0;
    for (let i = 0; i < store.length; i += 1) total += weight(store.key(i), store.getItem(store.key(i)));
    return total;
  };
  let total = measure();
  let stale = false;
  let reporting = false;
  post({ kept: total });
  const report = () => {
    if (reporting) return;
    reporting = true;
    later(() => {
      reporting = false;
      post({ kept: total });
    }, 2000);
  };
  window.addEventListener("storage", () => {
    stale = true;
  });
  const proto = Storage.prototype;
  const set = proto.setItem;
  const remove = proto.removeItem;
  const clear = proto.clear;
  proto.setItem = function (key, value) {
    if (this !== store) return set.call(this, key, value);
    const grown = weight(key, value) - weight(key, store.getItem(String(key)));
    if (stale || total + grown > allowed) {
      total = measure();
      stale = false;
    }
    if (grown > 0 && total + grown > allowed) {
      throw new DOMException("the page keeps more data than it declared", "QuotaExceededError");
    }
    set.call(this, key, value);
    total += grown;
    report();
  };
  proto.removeItem = function (key) {
    if (this !== store) return remove.call(this, key);
    total -= weight(key, store.getItem(String(key)));
    remove.call(this, key);
    report();
  };
  proto.clear = function () {
    if (this !== store) return clear.call(this);
    total = 0;
    clear.call(this);
    report();
  };
  const own = new Set(Object.getOwnPropertyNames(proto));
  const named = (key) => typeof key === "string" && !own.has(key);
  const shown = new Proxy(store, {
    get: (target, key) => {
      const value = Reflect.get(target, key, target);
      return typeof value === "function" ? value.bind(target) : value;
    },
    set: (target, key, value) => {
      if (!named(key)) return Reflect.set(target, key, value, target);
      target.setItem(key, value);
      return true;
    },
    deleteProperty: (target, key) => {
      if (named(key)) target.removeItem(key);
      return true;
    },
  });
  try {
    Object.defineProperty(window, "localStorage", { get: () => shown, configurable: true, enumerable: true });
  } catch (e) {}
}
