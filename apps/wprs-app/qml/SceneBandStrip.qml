import QtQuick
import org.kde.kirigami as Kirigami

Item {
    id: strip

    required property var picker

    Row {
        anchors.fill: parent

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
    }

    SceneBandMeter {
        anchors.fill: parent
        page: picker.page
        source: picker.reply?.source ?? ({
                "audio": "level"
            })
        of: picker.of
        live: picker.visible
    }

    MouseArea {
        id: drag

        property int anchorAt: -1

        function bandAt(x) {
            return Math.max(0, Math.min(picker.of - 1, Math.floor(x / strip.width * picker.of)));
        }

        anchors.fill: parent
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
