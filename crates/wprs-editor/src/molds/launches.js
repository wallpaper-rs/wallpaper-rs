import { storage } from "wprs";

let runs = 0;

export function init() {
    runs = (storage.get("runs") ?? 0) + 1;
    storage.set("runs", runs);
}

export const launches = () => runs;
