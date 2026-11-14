import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: chosenBar

    required property var page
    readonly property var chosen: (page.view.gizmos ?? []).find(g => g.pointer === page.editor.selected) ?? null
    readonly property bool free: chosen !== null && !chosen.locked
    readonly property var node: chosen ? (page.view.layers ?? []).find(item => item.pointer === chosen.pointer) ?? null : null

    function fit(how) {
        page.run("layer", {
                "fit": {
                    "layer": chosen.id,
                    "fit": how,
                    "at": page.editor.time,
                    "replace": page.overwrite
                }
            });
    }

    visible: (page.view.layers ?? []).length > 0
    implicitHeight: visible ? Look.barraLienzoAlto : 0
    color: Look.noche

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        spacing: Look.e2

        QQC2.Label {
            Layout.maximumWidth: Look.paso
            text: chosenBar.chosen ? chosenBar.chosen.id : App.tr("Canvas")
            font.weight: Font.DemiBold
            elide: Text.ElideRight
        }

        QQC2.ToolButton {
            id: fitting

            visible: chosenBar.chosen !== null
            enabled: chosenBar.free
            icon.name: "caret-down"
            text: App.tr("Fit to the canvas")
            onClicked: fits.popup(fitting, 0, fitting.height)

            QQC2.Menu {
                id: fits

                QQC2.MenuItem {
                    text: App.tr("Cover the canvas")
                    onTriggered: chosenBar.fit("cover")
                }

                QQC2.MenuItem {
                    text: App.tr("Fit inside the canvas")
                    onTriggered: chosenBar.fit("contain")
                }

                QQC2.MenuItem {
                    text: App.tr("Stretch to the canvas")
                    onTriggered: chosenBar.fit("stretch")
                }
            }
        }

        SceneSplitTool {
            page: chosenBar.page
            display: QQC2.AbstractButton.TextBesideIcon
            text: App.tr("Before and after")
        }

        SceneCanvasClock {
            page: chosenBar.page
            display: QQC2.AbstractButton.TextBesideIcon
        }

        SceneChosenMore {
            bar: chosenBar
        }

        Item {
            Layout.fillWidth: true
        }
    }
}
