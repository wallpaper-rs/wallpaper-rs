import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: network

    required property var page
    readonly property var origins: page.manifest.permissions?.network?.allow ?? []
    readonly property int most: 8

    function add() {
        const text = entry.text.trim();
        if (text === "")
            return;
        page.manifestEdit("network", {
                "origins": [text],
                "add": true
            });
        if (network.origins.some(o => o === text.replace(/\/+$/, "").toLowerCase()))
            entry.text = "";
    }

    function drop(origin) {
        page.manifestEdit("network", {
                "origins": [origin],
                "remove": true
            });
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: network.origins.length ? App.tr("The page may reach these https origins once the user allows the network; nothing else is reachable.") : App.tr("The page reaches no network. List the https origins it needs to let it ask the user for the network.")
    }

    Repeater {
        model: network.origins

        delegate: RowLayout {
            id: row

            required property string modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                text: row.modelData
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: network.drop(row.modelData)
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true
        enabled: network.origins.length < network.most

        QQC2.TextField {
            id: entry

            Layout.fillWidth: true
            placeholderText: App.tr("https://api.example.org")
            inputMethodHints: Qt.ImhUrlCharactersOnly
            onAccepted: network.add()
        }

        QQC2.Button {
            text: App.tr("Add origin")
            icon.name: "list-add"
            enabled: entry.text.trim() !== ""
            onClicked: network.add()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: network.origins.length ? App.tr("The network input is declared for you while origins are listed, and removed with the last one.") : App.tr("At most %1 origins; the network input is declared for you.", network.most)
    }
}
