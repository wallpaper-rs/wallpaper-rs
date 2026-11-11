import { setInterval } from "wprs";

let count = 0;

export function init() {
    setInterval(() => count += 1, 1000);
}

export const seconds = () => count;
