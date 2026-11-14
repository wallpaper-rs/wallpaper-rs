import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: bar

    required property var canvas
    readonly property var page: canvas.page
    readonly property alias spans: spans
    readonly property alias guides: guides
    readonly property alias seeAs: seeAs
    readonly property alias capture: capture

    spacing: 0

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.barraLienzoAlto
        color: Look.nocheAlta

        RowLayout {
            anchors.fill: parent
            anchors.rightMargin: Look.e3
            spacing: Look.e2

            Flickable {
                id: strip

                Layout.fillWidth: true
                Layout.fillHeight: true
                contentWidth: Math.max(width, row.implicitWidth + Look.e3 * 2)
                contentHeight: height
                flickableDirection: Flickable.HorizontalFlick
                boundsBehavior: Flickable.StopAtBounds
                clip: true
                QQC2.ScrollBar.horizontal: QQC2.ScrollBar {
                    policy: strip.contentWidth > strip.width ? QQC2.ScrollBar.AlwaysOn : QQC2.ScrollBar.AlwaysOff
                }

                WheelHandler {
                    enabled: strip.contentWidth > strip.width
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    onWheel: event => {
                        const step = event.angleDelta.x !== 0 ? event.angleDelta.x : event.angleDelta.y;
                        strip.contentX = Math.max(0, Math.min(strip.contentWidth - strip.width, strip.contentX - step));
                    }
                }

                RowLayout {
                    id: row

                    x: Look.e3
                    width: strip.contentWidth - Look.e3 * 2
                    height: strip.height
                    spacing: Look.e2

                    SceneZoomMenu {
                        canvas: bar.canvas
                    }

                    QQC2.ToolButton {
                        icon.name: "cursor-click"
                        text: App.tr("Test the pointer")
                        checkable: true
                        checked: bar.canvas.testing
                        QQC2.ToolTip.text: App.tr("Test the pointer and clicks")
                        QQC2.ToolTip.visible: hovered
                        onToggled: {
                            bar.canvas.testing = checked;
                            if (!checked)
                                bar.page.editor.touch(-1, -1, 0);
                        }
                    }

                    QQC2.ComboBox {
                        id: guides

                        Layout.minimumWidth: Look.altoCampo * 3
                        Accessible.name: App.tr("Guide")
                        model: bar.canvas.aspects.map(aspect => aspect || App.tr("No guide"))
                        displayText: App.tr("Guide: %1", currentText)
                    }

                    SceneSpanPick {
                        id: spans

                        page: bar.page
                    }

                    Item {
                        Layout.fillWidth: true
                    }
                }
            }

            SceneSeeAs {
                id: seeAs

                page: bar.page
            }

            SceneCapture {
                id: capture

                page: bar.page
                display: bar.width < Look.panelChico * 2 ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
            }
        }
    }

    SceneChosenBar {
        Layout.fillWidth: true
        page: bar.page
    }

    SceneTransitionBar {
        Layout.fillWidth: true
        page: bar.page
    }

    Rectangle {
        Layout.fillWidth: true
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }
}
