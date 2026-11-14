import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: panel

    property string provider
    property string error

    function ask(id) {
        provider = id;
        error = "";
        open();
    }

    title: App.tr("Change key")
    principal: App.tr("Save")
    principalIcon: "key"
    quiet: App.tr("Cancel")
    onAccepted: App.setSecret(provider, entry.text, () => {
        if (panel) {
            entry.clear();
            panel.close();
        }
    }, e => panel && (panel.error = e.message))
    onRejected: {
        entry.clear();
        close();
    }

    QQC2.TextField {
        id: entry

        width: Look.panelChico - 2 * Look.e4
        echoMode: TextInput.Password
        focus: true
    }

    Nota {
        visible: panel.error !== ""
        tone: "coral"
        text: panel.error
    }
}
