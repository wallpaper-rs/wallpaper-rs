import { scene, engine, console } from "wprs";

const estados = {
    verde: { dura: 4, siguiente: "ambar" },
    ambar: { dura: 1.5, siguiente: "roja" },
    roja: { dura: 4, siguiente: "verde" },
};
let actual = "";
let desde = 0;

function entrar(nombre) {
    actual = nombre;
    desde = engine.time;
    for (const luz of Object.keys(estados)) {
        scene.layer(luz).opacity = luz === nombre ? 1 : 0.12;
    }
    console.log("estado:", nombre);
}

export function init() {
    entrar("roja");
}

export function update() {
    if (engine.time - desde >= estados[actual].dura) {
        entrar(estados[actual].siguiente);
    }
}
