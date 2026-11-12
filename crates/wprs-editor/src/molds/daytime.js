import { engine } from "wprs";

export const daylight = () => 0.5 - 0.5 * Math.cos(engine.daytime * Math.PI * 2);
export const hour = () => engine.daytime * 24;
