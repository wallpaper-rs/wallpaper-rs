pragma Singleton
import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function round(p) {
        return p.map(v => Math.round(v * 10) / 10);
    }

    function inside(poly, p) {
        let hit = false;
        for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
            const a = poly[i], b = poly[j];
            if ((a[1] > p[1]) !== (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0])
                hit = !hit;
        }
        return hit;
    }

    function under(page, p) {
        const found = (page.view.gizmos ?? []).filter(g => inside(g.outline, p)).pop();
        return found ? found.id : null;
    }

    function along(corners, p) {
        const a = corners[0], b = corners[1];
        const d = [b[0] - a[0], b[1] - a[1]];
        const t = ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / Math.max(1e-6, d[0] * d[0] + d[1] * d[1]);
        return Math.round(Math.max(0, Math.min(1, t)) * 100) / 100;
    }

    function walls(w, c) {
        const loc = p => round(w.swarm.local(p));
        const dist = (a, b) => Math.round(Math.hypot(a[0] - b[0], a[1] - b[1]) * 10) / 10;
        switch (c.kind) {
        case "plane":
            return [{
                    "at": c.point,
                    "tip": App.tr("Drag to move the plane"),
                    "act": to => w.patch(c, v => v.point = loc(to))
                }, {
                    "at": c.tip,
                    "tip": App.tr("Drag to turn the side particles stay on"),
                    "act": to => w.patch(c, v => {
                                const a = loc(c.point), b = loc(to);
                                v.normal = round([b[0] - a[0], b[1] - a[1]]);
                            })
                }];
        case "circle":
            return [{
                    "at": c.center,
                    "act": to => w.patch(c, v => v.center = loc(to))
                }, {
                    "at": c.edge,
                    "tip": App.tr("Drag to change the radius"),
                    "act": to => w.patch(c, v => v.radius = Math.max(0.1, dist(loc(to), loc(c.center))))
                }];
        case "box":
            return [["from", 0], ["to", 2]].map(pair => ({
                            "at": c.corners[pair[1]],
                            "act": to => w.patch(c, v => v[pair[0]] = loc(to))
                        }));
        case "segment":
            return ["from", "to"].map(key => ({
                            "at": c[key],
                            "act": to => w.patch(c, v => v[key] = loc(to))
                        }));
        case "layer":
            return [{
                    "at": c.grip,
                    "drop": true,
                    "tip": App.tr("Drag onto another layer to make particles collide with that layer"),
                    "act": (to, done) => {
                        const id = done ? under(w.swarm.page, to) : null;
                        if (id && id !== c.value.bounds.layer.id)
                            w.patch(c, v => v.id = id);
                    }
                }];
        case "mask":
            return [{
                    "at": c.grip,
                    "tip": App.tr("Drag along the top edge to set the opacity from which the picture is solid"),
                    "act": to => w.patch(c, v => v.threshold = along(c.corners, to))
                }];
        default:
            return [];
        }
    }

    function spot(k, e, ref, to, write) {
        if (typeof ref === "string" && /^p\d$/.test(ref))
            k.swarm.edit("point", {
                    "point": Number(ref.slice(1)),
                    "at": k.swarm.local(to)
                });
        else if (Array.isArray(ref))
            k.patch(e, v => write(v, round(k.swarm.local(to))));
    }

    function keeps(k, e) {
        if (e.kind === "keep_distance")
            return [{
                    "at": e.at,
                    "label": App.tr("target"),
                    "act": to => spot(k, e, e.target, to, (v, p) => v.target = p)
                }, {
                    "at": e.ring,
                    "tip": App.tr("Drag to change the distance kept"),
                    "act": to => k.patch(e, v => v.distance = Math.round(Math.hypot(to[0] - e.at[0], to[1] - e.at[1]) / Math.max(1e-6, e.radius / Math.max(1e-6, e.distance)) * 10) / 10)
                }];
        return [{
                "at": e.from,
                "label": App.tr("from"),
                "act": to => spot(k, e, e.from_ref, to, (v, p) => v.from = p)
            }, {
                "at": e.to,
                "label": App.tr("to"),
                "act": to => spot(k, e, e.to_ref, to, (v, p) => v.to = p)
            }];
    }
}
