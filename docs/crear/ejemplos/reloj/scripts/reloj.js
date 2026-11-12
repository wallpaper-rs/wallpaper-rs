import { scene, time } from "wprs";

export function update() {
    scene.layer("hora").text = time.format("HH:mm");
    scene.layer("fecha").text = time.format("EEEE d 'de' MMMM");
}
