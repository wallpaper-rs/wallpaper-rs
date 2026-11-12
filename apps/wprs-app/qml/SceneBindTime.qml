import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: menu

    required property var page
    required property var field
    readonly property bool words: ["text", "multiline"].includes(field.widget)
    readonly property bool numeric: ["number", "integer"].includes(field.widget)
    readonly property var formats: [["clock", "HH:mm"], ["clock", "HH:mm:ss"], ["clock", "hh:mm a"], ["date", "EEEE"], ["date", "d MMMM yyyy"], ["date", "dd/MM/yyyy"], ["date", "EEEE d MMMM yyyy"], ["date", "EEE d MMM, HH:mm"]]
    readonly property var parts: ({
            "day": App.tr("Time of the day, from 0 to 1"),
            "hour": App.tr("Hour"),
            "minute": App.tr("Minute"),
            "second": App.tr("Second"),
            "weekday": App.tr("Day of the week"),
            "month": App.tr("Month")
        })
    property var samples: ({})

    signal picked(var source)

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

    title: words ? App.tr("Show the time or the date") : App.tr("Follow the time of the day")
    enabled: words || numeric
    onAboutToShow: {
        const next = {};
        formats.forEach(format => next[format[1]] = sample(format));
        samples = next;
    }

    SceneRepeater {
        id: texts

        rows: menu.words ? menu.formats : []

        delegate: QQC2.MenuItem {
            required property int index
            readonly property var modelData: texts.rows[index]

            text: modelData[1] + " → " + (menu.samples[modelData[1]] ?? "")
            onTriggered: {
                const source = {};
                source[modelData[0]] = modelData[1];
                menu.picked(source);
            }
        }
    }

    SceneRepeater {
        id: numbers

        rows: menu.numeric ? Object.keys(menu.parts) : []

        delegate: QQC2.MenuItem {
            required property int index
            readonly property string modelData: numbers.rows[index]

            text: menu.parts[modelData]
            onTriggered: menu.picked({
                    "clock": modelData
                })
        }
    }
}
