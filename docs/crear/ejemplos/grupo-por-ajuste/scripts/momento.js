import { scene, props, engine, setInterval } from "wprs";

function elegido() {
    const ajuste = props.get("momento");
    if (ajuste !== "auto") return ajuste;
    return engine.daytime >= 0.25 && engine.daytime < 0.75 ? "dia" : "noche";
}

function mostrar() {
    const momento = elegido();
    for (const grupo of ["dia", "noche"]) {
        scene.layer(grupo).visible = grupo === momento;
    }
}

export function properties() {
    mostrar();
}

setInterval(mostrar, 60000);
