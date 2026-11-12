import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: mark

    required property Item lane
    required property var owner
    required property var event
    readonly property real span: owner.line.length
    readonly property real moved: drag.active ? drag.translation.x : 0

    function timeAt(x) {
        return Math.round(Math.max(0, Math.min(1, x / lane.width)) * span * 100) / 100;
    }

    x: lane.width * Math.min(1, event.t / span) - width / 2 + moved
    y: 2
    width: Kirigami.Units.smallSpacing * 2
    height: lane.height - 4
    radius: width / 2
    color: drag.active || menu.opened ? Kirigami.Theme.highlightColor : Kirigami.Theme.neutralTextColor

    QQC2.Label {
        anchors.left: parent.right
        anchors.leftMargin: 2
        text: mark.event.name
        font: Kirigami.Theme.smallFont
    }

    QQC2.ToolTip.text: App.tr("%1 at %2 s", event.name, event.t)
    QQC2.ToolTip.visible: hover.hovered && !drag.active

    HoverHandler {
        id: hover
    }

    TapHandler {
        onTapped: {
            mark.owner.line.seek(mark.event.t, false);
            menu.open();
        }
    }

    DragHandler {
        id: drag

        yAxis.enabled: false
        target: null
        onActiveChanged: {
            if (active)
                return;
            const at = mark.timeAt(mark.lane.width * Math.min(1, mark.event.t / mark.span) + translation.x);
            if (Math.abs(at - mark.event.t) >= 0.005)
                mark.owner.edit({
                    "name": mark.event.name,
                    "at": mark.event.t,
                    "time": at
                });
        }
    }

    QQC2.Popup {
        id: menu

        y: parent.height
        padding: Kirigami.Units.smallSpacing

        RowLayout {
            QQC2.TextField {
                id: renamed

                Layout.preferredWidth: Kirigami.Units.gridUnit * 8
                text: mark.event.name
                placeholderText: App.tr("Name of the event")
                onAccepted: rename.clicked()
            }

            QQC2.Button {
                id: rename

                text: App.tr("Rename")
                enabled: renamed.text.trim() !== "" && renamed.text.trim() !== mark.event.name
                onClicked: {
                    mark.owner.edit({
                        "name": mark.event.name,
                        "at": mark.event.t,
                        "to": renamed.text.trim()
                    });
                    menu.close();
                }
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                onClicked: {
                    mark.owner.edit({
                        "name": mark.event.name,
                        "at": mark.event.t,
                        "remove": true
                    });
                    menu.close();
                }
            }
        }
    }
}
