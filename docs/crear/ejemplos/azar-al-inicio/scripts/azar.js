import { scene } from "wprs";

const formas = ["circulo", "cuadrado", "anillo"];

export function init() {
    const elegida = formas[Math.floor(Math.random() * formas.length)];
    for (const id of formas) {
        scene.layer(id).visible = id === elegida;
    }
    scene.layer(elegida).rotation = Math.random() * 360;
    console.log("forma elegida:", elegida);
}
