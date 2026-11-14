import QtQuick
import QtQuick.Layouts
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: bar

    required property var page
    readonly property var tool: page.settings ? page.settings.passage : null

    visible: tool !== null && tool.open
    implicitHeight: visible ? Look.altoHerr + Look.e2 * 2 : 0
    color: Look.nocheAlta

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e2

        QQC2.Label {
            text: bar.tool && bar.tool.way === "out" ? App.tr("Transition when it leaves") : App.tr("Transition when it comes in")
            font.weight: Font.DemiBold
            elide: Text.ElideRight
            Layout.fillWidth: true
            Layout.minimumWidth: 0
        }

        QQC2.ToolButton {
            icon.name: bar.tool && bar.tool.playing ? "media-playback-pause" : "media-playback-start"
            text: App.tr("Try the transition")
            display: bar.width < Look.e8 * 10 ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            Accessible.name: text
            QQC2.ToolTip.visible: hovered && display === QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            enabled: bar.tool !== null && bar.tool.current !== null
            onClicked: bar.tool.playing = !bar.tool.playing
        }

        QQC2.ToolButton {
            id: before

            text: App.tr("Previous wallpaper: %1", bar.tool ? bar.tool.beforeName : "") + " ▾"
            Accessible.name: App.tr("Previous wallpaper: %1", bar.tool ? bar.tool.beforeName : "")
            onClicked: menu.popup(before, 0, before.height)

            QQC2.Menu {
                id: menu

                QQC2.MenuItem {
                    text: App.tr("Plain color")
                    onTriggered: bar.tool.before = ""
                }

                QQC2.MenuItem {
                    text: App.tr("Choose a picture…")
                    onTriggered: files.open()
                }
            }
        }

        QQC2.Button {
            text: App.tr("Done")
            onClicked: bar.tool.close()
        }
    }

    FileDialog {
        id: files

        nameFilters: [App.tr("Pictures") + " (*.png *.jpg *.jpeg *.webp)"]
        onAccepted: bar.tool.before = App.path(selectedFile)
    }
}
