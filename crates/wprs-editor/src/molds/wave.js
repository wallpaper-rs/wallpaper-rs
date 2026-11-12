import { engine } from "wprs";

export const wave = (value, { params }) => Math.sin(engine.time * params.speed) * params.size;
