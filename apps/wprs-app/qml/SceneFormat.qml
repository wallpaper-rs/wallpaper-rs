import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: format

    required property var host
    readonly property bool date: host.field.key === "date"
    readonly property string pattern: String(host.value ?? "")
    readonly property var words: ({
            "HH": App.tr("Hour, 00 to 23"),
            "H": App.tr("Hour, 0 to 23"),
            "hh": App.tr("Hour, 01 to 12"),
            "h": App.tr("Hour, 1 to 12"),
            "mm": App.tr("Minutes, 00 to 59"),
            "m": App.tr("Minutes, 0 to 59"),
            "ss": App.tr("Seconds, 00 to 59"),
            "s": App.tr("Seconds, 0 to 59"),
            "a": App.tr("AM or PM"),
            "yyyy": App.tr("Year, four digits"),
            "yy": App.tr("Year, two digits"),
            "MMMM": App.tr("Name of the month"),
            "MMM": App.tr("Name of the month, short"),
            "MM": App.tr("Month, 01 to 12"),
            "M": App.tr("Month, 1 to 12"),
            "dd": App.tr("Day of the month, 01 to 31"),
            "d": App.tr("Day of the month, 1 to 31"),
            "EEEE": App.tr("Name of the day"),
            "EEE": App.tr("Name of the day, short"),
            "D": App.tr("Day of the year"),
            "w": App.tr("Week of the year")
        })
    readonly property var names: ({
            "day": App.tr("Day: 0 at midnight, almost 1 the next midnight"),
            "hour": App.tr("Hour, 0 to 23"),
            "minute": App.tr("Minute, 0 to 59"),
            "second": App.tr("Second, 0 to 59"),
            "weekday": App.tr("Weekday, 1 is Monday"),
            "month": App.tr("Month, 1 to 12")
        })
    property var help: ({})
    property var shown: ({})
    property bool open

    function wall() {
        return Date.now() / 1000 - new Date().getTimezoneOffset() * 60;
    }

    function ask(query) {
        return JSON.parse(host.page.editor.rig("", JSON.stringify(query)) || "{}");
    }

    function refresh() {
        const now = wall();
        const lang = Qt.locale().name.substring(0, 2);
        const number = !date && names[pattern] !== undefined;
        if (open || number)
            help = ask({
                    "clock_help": {
                        "wall": now,
                        "lang": lang
                    }
                });
        shown = number ? {
            "text": String((help.numbers ?? []).find(n => n.name === pattern)?.sample ?? "")
        } : ask({
                "clock_sample": {
                    "pattern": pattern,
                    "dates": date,
                    "wall": now,
                    "lang": lang
                }
            });
    }

    function append(token) {
        host.commit(pattern + token);
    }

    onPatternChanged: refresh()
    onOpenChanged: refresh()
    Component.onCompleted: refresh()

    Timer {
        interval: 1000
        repeat: true
        running: format.visible
        onTriggered: format.refresh()
    }

    QQC2.TextField {
        Layout.fillWidth: true
        text: format.pattern
        font.family: "monospace"
        onEditingFinished: {
            if (text !== format.pattern)
                format.host.commit(text);
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: format.pattern !== ""
        text: App.tr("Now it reads: %1", format.shown.text ?? "")
        font.bold: true
        wrapMode: Text.Wrap
    }

    QQC2.Button {
        flat: true
        icon.name: format.open ? "arrow-up" : "help-contents"
        text: format.open ? App.tr("Hide the help") : App.tr("Help with the format")
        onClicked: format.open = !format.open
    }

    ColumnLayout {
        Layout.fillWidth: true
        visible: format.open

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            font: Kirigami.Theme.smallFont
            text: format.date ? App.tr("Click a piece to add it. Text between single quotes stays as written, and two quotes in a row write one quote.") : App.tr("Click a piece to add it. Any other letter stays as written.")
        }

        Flow {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            SceneRepeater {
                id: tokens

                rows: (format.help.tokens ?? []).filter(row => format.date || !row.dates)

                delegate: QQC2.Button {
                    required property int index
                    readonly property var modelData: tokens.rows[index]

                    text: modelData.token + " → " + modelData.sample
                    QQC2.ToolTip.text: format.words[modelData.token] ?? ""
                    QQC2.ToolTip.visible: hovered
                    onClicked: format.append(modelData.token)
                }
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: !format.date
            wrapMode: Text.Wrap
            font: Kirigami.Theme.smallFont
            text: App.tr("For a number, write one of these names instead of a format")
        }

        Flow {
            Layout.fillWidth: true
            visible: !format.date
            spacing: Kirigami.Units.smallSpacing

            SceneRepeater {
                id: numbers

                rows: format.help.numbers ?? []

                delegate: QQC2.Button {
                    required property int index
                    readonly property var modelData: numbers.rows[index]

                    text: modelData.name + " → " + modelData.sample
                    QQC2.ToolTip.text: format.names[modelData.name] ?? ""
                    QQC2.ToolTip.visible: hovered
                    onClicked: format.host.commit(modelData.name)
                }
            }
        }
    }
}
