import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: panels

    property var page: null
    readonly property alias adder: adder
    readonly property alias connector: connector
    readonly property alias remover: remover
    readonly property alias resetter: resetter

    function show(panel, opener) {
        panel.opener = opener;
        panel.open();
    }

    function add(opener) {
        show(adder, opener);
    }

    function connect(opener) {
        show(connector, opener);
    }

    function remove(opener) {
        show(remover, opener);
    }

    function reset(opener) {
        show(resetter, opener);
    }

    PanelChico {
        id: adder

        title: App.tr("Add server")
        text: App.tr("The address of the server. A server at home with its own certificate also needs its fingerprint.")
        quiet: App.tr("Cancel")
        principal: App.tr("Add")
        principalIcon: "plus"
        onClosed: {
            url.text = "";
            pin.text = "";
        }
        onAccepted: pin.text.trim() === "" ? App.act("AddServer", [url.text.trim()], "Sync1") : App.act("AddPinnedServer", [url.text.trim(), pin.text.trim()], "Sync1")

        QQC2.TextField {
            id: url

            width: adder.width - 2 * Look.e4
            placeholderText: "https://wallpapers.example.org"
            Accessible.name: App.tr("Address of the server")
        }

        QQC2.TextField {
            id: pin

            width: adder.width - 2 * Look.e4
            placeholderText: App.tr("Fingerprint of its own certificate, only for a server at home")
            Accessible.name: placeholderText
        }
    }

    PanelChico {
        id: connector

        title: App.tr("Connect this computer")
        text: App.tr("The server shows a code to approve with your account. Connecting gives you its repositories; your library is synced only when you turn it on.")
        quiet: App.tr("Cancel")
        principal: App.tr("Connect")
        principalIcon: "link"
        onAccepted: panels.page.link(name.text.trim(), panels.page.plain)

        QQC2.TextField {
            id: name

            width: connector.width - 2 * Look.e4
            placeholderText: App.tr("Name of this computer, seen only by your devices")
            Accessible.name: placeholderText
        }
    }

    PanelChico {
        id: remover

        title: App.tr("Remove server?")
        text: App.tr("This computer leaves the account and stops syncing. Your library stays here.")
        quiet: App.tr("Cancel")
        principal: App.tr("Remove server")
        principalIcon: "trash"
        borra: true
        onClosed: wipe.checked = false
        onAccepted: App.act("RemoveServer", [wipe.checked], "Sync1")

        QQC2.CheckBox {
            id: wipe

            width: remover.width - 2 * Look.e4
            text: App.tr("Also forget the synced copies on this computer")
        }
    }

    PanelChico {
        id: resetter

        title: App.tr("Reset synced data?")
        text: App.tr("This deletes every synced record on the server and uploads the library, playlists and settings of this computer under a new key. Every other device has to get the key again. Use it only when no device holds the key any more.")
        quiet: App.tr("Cancel")
        principal: App.tr("Reset")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("Reset", [], "Sync1")
    }
}
