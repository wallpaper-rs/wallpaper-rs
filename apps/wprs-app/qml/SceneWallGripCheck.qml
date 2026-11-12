import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function walls() {
        return JSON.parse(page.editor.rig("flakes", JSON.stringify({
                        "particles": {
                            "time": 0
                        }
                    })) || "null").colliders;
    }

    function patch(c, change) {
        const v = JSON.parse(JSON.stringify(c.value));
        change(v.bounds[c.kind]);
        page.run("particles", {
                "collider": {
                    "system": "flakes",
                    "index": c.index,
                    "collider": v
                }
            });
    }

    function grips(c) {
        return SceneSwarmGrips.walls({
                "swarm": {
                    "page": page
                },
                "patch": check.patch
            }, c);
    }

    function exercise() {
        const steps = page.view.history.done.length;
        page.run("add", {
            "pointer": "/scene/layers/-",
            "value": {
                "id": "wallcheck",
                "type": "shape",
                "shape": {
                    "rect": {
                        "size": [200, 200]
                    }
                },
                "transform": {
                    "position": [300, 300]
                }
            }
        });
        page.run("particles", {
            "collider": {
                "system": "flakes",
                "collider": {
                    "bounds": {
                        "layer": {
                            "id": "sky"
                        }
                    }
                }
            }
        });
        page.run("particles", {
            "collider": {
                "system": "flakes",
                "collider": {
                    "bounds": {
                        "mask": {
                            "texture": "gone",
                            "threshold": 0.5
                        }
                    }
                }
            }
        });
        const all = walls();
        const layer = all.find(c => c.kind === "layer"), mask = all.find(c => c.kind === "mask");
        if (!layer || !mask || !Array.isArray(layer.grip) || !Array.isArray(mask.grip))
            throw new Error("collider grips: a layer and a mask collider show no handle on the canvas: " + JSON.stringify(all));
        const drop = grips(layer)[0];
        drop.act([310, 310], false);
        if (walls().find(c => c.kind === "layer").value.bounds.layer.id !== "sky")
            throw new Error("collider grips: the layer collider changed while the handle was still being dragged");
        drop.act([310, 310], true);
        if (walls().find(c => c.kind === "layer").value.bounds.layer.id !== "wallcheck")
            throw new Error("collider grips: dropping the handle on a layer did not point the collider at it");
        const edge = grips(mask)[0];
        edge.act([mask.corners[0][0] + (mask.corners[1][0] - mask.corners[0][0]) * 0.8, mask.corners[0][1] - 40], true);
        const moved = walls().find(c => c.kind === "mask");
        if (moved.value.bounds.mask.threshold !== 0.8 || Math.abs(moved.grip[0] - (mask.corners[0][0] + (mask.corners[1][0] - mask.corners[0][0]) * 0.8)) > 0.5)
            throw new Error("collider grips: dragging the mask handle did not set the threshold: " + JSON.stringify(moved.value));
        while (page.view.history.done.length > steps)
            page.run("undo");
    }
}
