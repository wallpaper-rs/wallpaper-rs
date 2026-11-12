import { scene } from "wprs";

let copies = [];

export function init({ params }) {
    for (let i = 0; i < params.count; i++) {
        copies.push(scene.spawn(params.template, { position: [160 + i * params.gap, 540] }));
    }
}

export function destroy() {
    copies.forEach((copy) => scene.destroy(copy.id));
    copies = [];
}
