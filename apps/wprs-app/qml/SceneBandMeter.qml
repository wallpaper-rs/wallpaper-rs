import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: meter

    required property var page
    property var source: ({
            "audio": "level"
        })
    property int of: 64
    property bool live: visible
    readonly property var reading: JSON.parse(page.editor.levels || "{}")
    readonly property var reply: JSON.parse(page.editor.rig("", JSON.stringify({
                "meter": {
                    "source": source,
                    "reading": reading,
                    "of": of
                }
            })) || "{}") ?? ({})
    readonly property var bands: reply.bands ?? []

    onLiveChanged: page.editor.hear(live)
    Component.onCompleted: page.editor.hear(live)
    Component.onDestruction: {
        if (live)
            page.editor.hear(false);
    }

    Row {
        anchors.fill: parent

        Repeater {
            model: meter.of

            delegate: Item {
                id: slot

                required property int index
                readonly property real level: Math.min(1, Math.max(0, meter.bands[index] ?? 0))
                readonly property bool marked: meter.reply.marked ? index >= meter.reply.marked[0] && index <= meter.reply.marked[1] : false

                width: meter.width / meter.of
                height: meter.height

                Rectangle {
                    anchors.fill: parent
                    visible: slot.marked
                    color: "transparent"
                    border.color: Kirigami.Theme.highlightColor
                    border.width: 1
                }

                Rectangle {
                    anchors.bottom: parent.bottom
                    anchors.horizontalCenter: parent.horizontalCenter
                    width: Math.max(1, parent.width - 2)
                    height: parent.height * slot.level
                    color: slot.marked ? Kirigami.Theme.highlightColor : Kirigami.Theme.disabledTextColor
                    opacity: 0.8
                }
            }
        }
    }
}
