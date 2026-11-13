import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.ToolButton {
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

    icon.name: "chronometer"
    text: App.tr("Ready-made starts")
    display: QQC2.AbstractButton.IconOnly
    QQC2.ToolTip.text: App.tr("Add a clock or a date, or make the selected layer turn or drift forever")
    QQC2.ToolTip.visible: hovered
    onClicked: menu.popup()

    QQC2.Menu {
        id: menu

        title: App.tr("Ready-made starts")
        onAboutToShow: {
            const next = {};
            starts.formats.forEach(format => next[format[1]] = starts.sample(format));
            starts.samples = next;
        }

        QQC2.Menu {
            title: App.tr("Add a clock or a date")
            icon.name: "clock"

            SceneRepeater {
                id: clocks

                rows: starts.formats

                delegate: QQC2.MenuItem {
                    required property int index
                    readonly property var modelData: clocks.rows[index]

                    text: modelData[1] + " → " + (starts.samples[modelData[1]] ?? "")
                    onTriggered: starts.addClock(modelData)
                }
            }
        }

        QQC2.Menu {
            title: App.tr("Keep the selected layer moving")
            enabled: starts.owner.current !== null

            SceneRepeater {
                id: motions

                rows: starts.moves

                delegate: QQC2.MenuItem {
                    required property int index
                    readonly property var modelData: motions.rows[index]

                    text: modelData[2]
                    onTriggered: starts.keepMoving(modelData)
                }
            }
        }
    }
}
