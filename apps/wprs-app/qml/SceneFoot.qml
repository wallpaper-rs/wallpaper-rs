import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: foot

    required property var page
    readonly property bool keyed: (page.view.tracks ?? []).length > 0 || page.mesh.clip !== ""
    property bool unfolded: false
    readonly property bool folded: !keyed && !unfolded
    readonly property real wanted: folded ? Look.barraLienzoAlto : Look.lienzoPieAlto

    color: Look.noche

    Rectangle {
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.topMargin: Look.linea
        spacing: 0

        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: Look.barraLienzoAlto - Look.linea
            Layout.leftMargin: Look.e3
            Layout.rightMargin: Look.e3
            spacing: Look.e2

            QQC2.ToolButton {
                visible: !foot.keyed
                icon.name: foot.folded ? "caret-right" : "caret-down"
                text: App.tr("Timeline")
                checkable: true
                checked: foot.unfolded
                onToggled: foot.unfolded = checked
            }

            QQC2.Label {
                visible: foot.keyed
                text: App.tr("Timeline")
                font.weight: Font.DemiBold
            }

            QQC2.Label {
                Layout.fillWidth: true
                text: App.tr("Lasts %1 s · repeats", Number((foot.page.view.length ?? 10).toFixed(2)))
                color: Look.bruma
                elide: Text.ElideRight
            }

            QQC2.ToolButton {
                id: change

                icon.name: "clock"
                text: App.tr("Change…")
                onClicked: lines.popup(change, 0, change.height)

                SceneTimelineMenu {
                    id: lines

                    timeline: ({
                            "page": foot.page
                        })
                }
            }
        }

        SceneTimes {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: !foot.folded
            page: foot.page
        }
    }
}
