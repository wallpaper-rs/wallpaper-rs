import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: picker

    required property var page
    property var base: ({})
    property int of: 16
    property int from: -1
    property int to: -1
    property string level: ""
    property string reduce: "mean"
    property string channel: "average"
    readonly property var table: ask({
        "of": picker.of
    })
    readonly property var request: level !== "" ? {
        "pick": level
    } : (from < 0 ? null : {
        "pick": from === to ? from : [from, to],
        "of": of,
        "reduce": reduce,
        "channel": channel
    })
    readonly property var reply: request ? ask(Object.assign({
            "base": base
        }, request)) : null
    readonly property var names: [["bass", App.tr("Bass")], ["mid", App.tr("Mids")], ["treble", App.tr("Treble")], ["level", App.tr("Level")], ["beat", App.tr("Beat")]]

    signal chosen(var source)

    function ask(args) {
        return JSON.parse(page.editor.rig("", JSON.stringify({
                    "bands": args
                })) || "null") ?? ({});
    }

    function pitch(hz) {
        return hz >= 1000 ? (hz / 1000).toFixed(1) + " kHz" : Math.round(hz) + " Hz";
    }

    function mark(index, extend) {
        level = "";
        from = extend && from >= 0 ? Math.min(from, index) : index;
        to = extend && to >= 0 ? Math.max(to, index) : index;
    }

    function summary() {
        if (level !== "")
            return names.find(n => n[0] === level)[1];
        if (!reply || reply.error)
            return reply?.error ?? App.tr("Click a band, or drag over several");
        const range = App.tr("%1 to %2", pitch(reply.hertz[0]), pitch(reply.hertz[1]));
        return from === to ? App.tr("Band %1: %2", from, range) : App.tr("Bands %1 to %2: %3", from, to, range);
    }

    title: App.tr("Follow the sound")
    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
    padding: Kirigami.Units.largeSpacing
    onAccepted: {
        if (reply?.source)
            chosen(reply.source);
    }
    onOpened: {
        level = "";
        from = to = -1;
    }

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        RowLayout {
            Repeater {
                model: picker.names

                delegate: QQC2.Button {
                    required property var modelData

                    text: modelData[1]
                    checkable: true
                    checked: picker.level === modelData[0]
                    onClicked: {
                        picker.level = modelData[0];
                        picker.from = picker.to = -1;
                    }
                }
            }
        }

        Row {
            id: strip

            Layout.fillWidth: true
            Layout.preferredWidth: Kirigami.Units.gridUnit * 30
            Layout.preferredHeight: Kirigami.Units.gridUnit * 3

            Repeater {
                model: picker.of

                delegate: Rectangle {
                    required property int index
                    readonly property bool inside: picker.level === "" && index >= picker.from && index <= picker.to

                    width: strip.width / picker.of
                    height: strip.height
                    color: inside ? Kirigami.Theme.highlightColor : Kirigami.Theme.alternateBackgroundColor
                    border.color: Kirigami.Theme.backgroundColor
                    border.width: 1
                }
            }

            MouseArea {
                id: drag

                property int anchorAt: -1

                function bandAt(x) {
                    return Math.max(0, Math.min(picker.of - 1, Math.floor(x / strip.width * picker.of)));
                }

                x: 0
                y: 0
                width: strip.width
                height: strip.height
                hoverEnabled: true
                onPressed: mouse => {
                    anchorAt = bandAt(mouse.x);
                    picker.mark(anchorAt, false);
                }
                onPositionChanged: mouse => {
                    if (pressed) {
                        picker.from = Math.min(anchorAt, bandAt(mouse.x));
                        picker.to = Math.max(anchorAt, bandAt(mouse.x));
                    }
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.Label {
                text: picker.pitch(picker.table.bands?.[0]?.from ?? 40)
            }

            Item {
                Layout.fillWidth: true
            }

            QQC2.Label {
                text: picker.pitch(picker.table.bands?.[picker.of - 1]?.to ?? 16000)
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            font.bold: true
            text: picker.summary()
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.ComboBox {
                Kirigami.FormData.label: App.tr("Bands")
                model: [App.tr("16 bands"), App.tr("32 bands"), App.tr("64 bands")]
                currentIndex: [16, 32, 64].indexOf(picker.of)
                onActivated: index => {
                    picker.of = [16, 32, 64][index];
                    picker.from = picker.to = -1;
                }
            }

            QQC2.ComboBox {
                Kirigami.FormData.label: App.tr("Several bands count as")
                enabled: picker.from !== picker.to && picker.level === ""
                model: [App.tr("Their average"), App.tr("The loudest")]
                currentIndex: picker.reduce === "max" ? 1 : 0
                onActivated: index => picker.reduce = index === 1 ? "max" : "mean"
            }

            QQC2.ComboBox {
                Kirigami.FormData.label: App.tr("Channel")
                enabled: picker.level === ""
                model: [App.tr("Both"), App.tr("Left"), App.tr("Right")]
                currentIndex: ["average", "left", "right"].indexOf(picker.channel)
                onActivated: index => picker.channel = ["average", "left", "right"][index]
            }
        }
    }
}
