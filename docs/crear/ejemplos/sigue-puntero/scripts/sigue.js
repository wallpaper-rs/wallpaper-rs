import { scene, input, engine } from "wprs";

const seguidores = [
    { capa: scene.layer("cabeza"), prisa: 8 },
    { capa: scene.layer("medio"), prisa: 4 },
    { capa: scene.layer("cola"), prisa: 2 },
];
const [cx, cy] = engine.canvas.map((v) => v / 2);
const lugar = seguidores.map(() => [cx, cy]);

export function update() {
    const [px, py] = input.cursor.scene;
    seguidores.forEach(({ capa, prisa }, i) => {
        const paso = 1 - Math.exp(-prisa * engine.dt);
        lugar[i][0] += (px - lugar[i][0]) * paso;
        lugar[i][1] += (py - lugar[i][1]) * paso;
        capa.position = lugar[i];
    });
}
