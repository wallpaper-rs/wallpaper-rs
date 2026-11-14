import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.ItemDelegate {
    id: tile

    required property int index
    required property string uid
    required property string title
    required property string kind
    required property string color
    required property string thumb
    required property bool favorite
    required property int rating
    required property string info
    required property bool missing
    required property string broken
    required property string revoked
    required property bool selected
    required property string tags
    required property bool mature
    readonly property bool veiled: mature && !App.unveiled
    required property bool flashing

    readonly property bool current: GridView.isCurrentItem && GridView.view.activeFocus

    signal picked(int modifiers)
    signal opened

    highlighted: selected
    focusPolicy: Qt.NoFocus
    Keys.onSpacePressed: event => GridView.view.choose(event.modifiers)

    Rectangle {
        anchors.fill: parent
        z: 1
        visible: tile.current
        color: "transparent"
        border.width: 2
        border.color: Kirigami.Theme.focusColor
        radius: Kirigami.Units.smallSpacing
    }

    QQC2.ToolTip {
        id: tip

        visible: tile.hovered
        delay: Kirigami.Units.toolTipDelay
        text: [tile.title, tile.info, tile.tags.split(", ").map(Labels.tag).join(", "), tile.missing ? App.tr("The file is missing") : "", tile.broken, tile.revoked ? Labels.value(tile.revoked) : "", tile.veiled ? App.tr("Mature content: open the details to show it") : "", tile.flashing ? App.tr("Flashes or shows fast patterns; it is disabled, and playlists skip it, until wallpapers that flash are allowed") : ""].filter(t => t).join("\n")

        contentItem: QQC2.Label {
            text: tip.text
            textFormat: Text.PlainText
            wrapMode: Text.Wrap
            color: tip.palette.toolTipText
        }
    }

    contentItem: ColumnLayout {
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: tile.color || Kirigami.Theme.alternateBackgroundColor

            Image {
                anchors.fill: parent
                source: tile.thumb
                asynchronous: true
                fillMode: Image.PreserveAspectFit
                sourceSize.width: tile.veiled ? 12 : width * Screen.devicePixelRatio
                sourceSize.height: tile.veiled ? 12 : height * Screen.devicePixelRatio
            }

            Row {
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.margins: Kirigami.Units.smallSpacing

                Repeater {
                    model: [tile.favorite && "emblem-favorite", tile.kind === "video" && "media-playback-start", tile.kind === "package" && "package-x-generic", tile.kind === "scene" && "view-media-visualization", (tile.missing || tile.broken) && "emblem-error", tile.revoked && "emblem-unavailable", tile.veiled && "view-private", tile.flashing && "dialog-warning"].filter(icon => icon)

                    delegate: Kirigami.Icon {
                        required property string modelData

                        width: Kirigami.Units.iconSizes.small
                        height: width
                        source: modelData
                    }
                }
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            text: tile.title
            textFormat: Text.PlainText
            elide: Text.ElideRight
            horizontalAlignment: Text.AlignHCenter
            color: tile.missing ? Kirigami.Theme.disabledTextColor : Kirigami.Theme.textColor
        }
    }

    MouseArea {
        anchors.fill: parent
        acceptedButtons: Qt.LeftButton
        onClicked: mouse => tile.picked(mouse.modifiers)
        onDoubleClicked: tile.opened()
    }

    QQC2.CheckBox {
        id: mark

        anchors.left: parent.left
        anchors.top: parent.top
        visible: tile.hovered || tile.selected || mark.activeFocus
        focusPolicy: Qt.NoFocus
        checked: tile.selected
        onToggled: tile.picked(Qt.ControlModifier)
        QQC2.ToolTip.text: App.tr("Choose this picture; choose several to add them to a playlist or rotate them")
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
    }
}
