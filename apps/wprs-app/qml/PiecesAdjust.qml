import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property int part: 0
    readonly property var parts: [rows, more, said, panels]
    readonly property Item shown: stack.children[part] ?? null

    function show(index) {
        part = Math.max(0, Math.min(parts.length - 1, index));
        shown.enter?.();
    }

    padding: 0
    title: App.tr("Pieces")
    background: Rectangle {
        color: Look.noche
    }
    Keys.onDigit1Pressed: show(0)
    Keys.onDigit2Pressed: show(1)
    Keys.onDigit3Pressed: show(2)
    Keys.onDigit4Pressed: show(3)
    Component.onCompleted: {
        show(0);
    }

    Item {
        id: stack

        anchors.fill: parent
        anchors.leftMargin: Look.margen
        anchors.topMargin: Look.margenArriba
        anchors.rightMargin: Look.margen

        PiecesRows {
            id: rows

            anchors.fill: parent
            visible: page.part === 0
        }

        PiecesMore {
            id: more

            anchors.fill: parent
            visible: page.part === 1
        }

        PiecesSaid {
            id: said

            anchors.fill: parent
            visible: page.part === 2
        }

        PiecesPanels {
            id: panels

            anchors.fill: parent
            visible: page.part === 3
        }
    }
}
