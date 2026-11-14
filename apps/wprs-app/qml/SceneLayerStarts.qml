import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: starts

    required property var owner
    readonly property var page: owner.page
    readonly property var formats: [["clock", "HH:mm"], ["clock", "HH:mm:ss"], ["clock", "hh:mm a"], ["date", "EEEE"], ["date", "d MMMM yyyy"], ["date", "dd/MM/yyyy"], ["date", "EEEE d MMMM yyyy"]]
    readonly property var moves: [["spin", false, App.tr("Spin clockwise")], ["spin", true, App.tr("Spin counterclockwise")], ["drift", false, App.tr("Drift to the right and back")], ["drift", true, App.tr("Drift to the left and back")]]
    property var samples: ({})

    function sample(format) {
        const now = Date.now() / 1000 - new Date().getTimezoneOffset() * 60;
        const reply = JSON.parse(page.editor.rig("", JSON.stringify({
                        "clock_sample": {
                            "pattern": format[1],
                            "dates": format[0] === "date",
                            "wall": now,
                            "lang": Qt.locale().name.substring(0, 2)
                        }
                    })) || "{}");
        return reply.text ?? "";
    }

    function addClock(format) {
        page.run("layer", {
                "start": {
                    "clock": {
                        "format": format[1],
                        "date": format[0] === "date",
                        "into": owner.current?.type === "group" ? owner.current.id : undefined
                    }
                }
            });
    }

    function keepMoving(move) {
        page.run("layer", {
                "start": {
                    "forever": {
                        "layer": owner.current.id,
                        "motion": move[0],
                        "reverse": move[1]
                    }
                }
            });
    }

    function refresh() {
        const next = {};
        formats.forEach(format => next[format[1]] = sample(format));
        samples = next;
    }
}
