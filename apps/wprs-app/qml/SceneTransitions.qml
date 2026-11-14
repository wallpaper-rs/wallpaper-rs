import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: section

    required property var tool

    function seconds(ms) {
        return App.tr("%1 s", Number(ms / 1000).toLocaleString(Qt.locale(), "f", 1));
    }

    spacing: 0

    Rectangle {
        Layout.fillWidth: true
        Layout.topMargin: Look.e3
        implicitHeight: Look.linea
        color: Look.nocheLinea
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        Layout.topMargin: Look.e3
        level: 4
        text: App.tr("Transitions")
    }

    Text {
        Layout.fillWidth: true
        Layout.bottomMargin: Look.e2
        wrapMode: Text.WordWrap
        text: App.tr("Without one of its own, whoever uses it sees the one chosen in their settings.")
        color: Look.bruma
        font.pixelSize: Look.tPie
    }

    Repeater {
        model: [
            {
                "way": "in",
                "icon": "sign-in",
                "text": App.tr("When it comes in")
            },
            {
                "way": "out",
                "icon": "sign-out",
                "text": App.tr("When it leaves")
            }
        ]

        delegate: QQC2.ItemDelegate {
            id: row

            required property var modelData
            readonly property var made: section.tool.info[modelData.way] ?? null

            Layout.fillWidth: true
            icon.name: modelData.icon
            text: modelData.text
            highlighted: section.tool.open && section.tool.way === modelData.way
            focusPolicy: Qt.StrongFocus
            Accessible.name: text + ", " + note.text
            onClicked: section.tool.start(modelData.way)
            Keys.onReturnPressed: clicked()

            QQC2.Label {
                id: note

                anchors.right: parent.right
                anchors.rightMargin: Look.e3
                anchors.verticalCenter: parent.verticalCenter
                text: row.made ? section.seconds(row.made.duration_ms) : App.tr("Make a transition…")
                color: row.highlighted ? Look.noche : row.made ? Look.bruma : Look.luna
                font.weight: row.made ? Font.Normal : Font.DemiBold
            }
        }
    }

    QQC2.ToolButton {
        visible: section.tool.open && section.tool.current !== null
        text: App.tr("Remove the transition")
        onClicked: section.tool.remove()
    }
}
