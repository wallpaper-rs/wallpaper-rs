import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string path: ""

    title: dialog.path
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 32

    function show(path) {
        dialog.path = path;
        dialog.open();
    }

    QQC2.ScrollView {
        implicitWidth: Kirigami.Units.gridUnit * 32
        implicitHeight: Kirigami.Units.gridUnit * 20

        Area {
            readOnly: true
            wrapMode: TextEdit.NoWrap
            font.family: "monospace"
            text: dialog.path !== "" ? dialog.page.editor.scriptSource(dialog.path) : ""
        }
    }
}
