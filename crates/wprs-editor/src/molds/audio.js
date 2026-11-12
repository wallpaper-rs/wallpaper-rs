import { audio } from "wprs";

let smooth = 0;

export function update() {
    smooth += (audio.level - smooth) * 0.2;
}

export const level = () => smooth;
export const bass = () => audio.bass;
export const beat = () => audio.beat;
