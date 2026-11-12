import { scene, audio, engine } from "wprs";

const pulso = scene.layer("pulso");
const destello = scene.layer("destello");
let tamano = 1;
let brillo = 0;

export function update() {
    const meta = 1 + audio.bass * 0.8;
    const rapidez = meta > tamano ? 30 : 4;
    tamano += (meta - tamano) * (1 - Math.exp(-rapidez * engine.dt));
    pulso.scale = tamano;
    pulso.opacity = 0.4 + audio.level * 0.6;

    brillo = audio.beat ? 0.25 : Math.max(0, brillo - engine.dt);
    destello.opacity = brillo;
}
