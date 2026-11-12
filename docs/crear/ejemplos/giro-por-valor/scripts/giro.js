import { engine } from "wprs";

export const gira = (grados, { params }) => ((grados ?? 0) + params.velocidad * engine.dt) % 360;
