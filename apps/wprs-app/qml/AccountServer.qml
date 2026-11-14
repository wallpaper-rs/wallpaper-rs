import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: level

    property var page: null
    readonly property var account: page ? page.account : null
    readonly property string pin: account && account.pin ? account.pin : ""
    readonly property alias qr: qr
    readonly property alias code: code

    function enter() {
        address.forceActiveFocus(Qt.TabFocusReason);
    }

    width: parent ? parent.width : 0
    height: list.height

    Filas {
        id: list

        Fila {
            id: address

            text: App.tr("Address")
            value: level.account ? level.account.name || level.account.url : ""
            note: level.account && level.account.name ? level.account.url : ""
            explain: App.tr("Connected, you browse, install and vote on its repositories. Your library is synced only when you turn syncing on.")
        }

        Fila {
            text: App.tr("Certificate")
            value: level.pin !== "" ? App.tr("its own, pinned") : App.tr("public")
            explain: level.pin !== "" ? App.tr("Pinned by the fingerprint %1", level.pin) : App.tr("The server shows a certificate that any device trusts.")
        }

        Fila {
            id: connect

            visible: !!level.account && !level.account.device && level.account.state !== "linking"
            text: App.tr("Connect this computer…")
            note: App.tr("Approve a code with your account")
            kind: "pliega"
            onOpened: level.page.panels.connect(connect)
        }

        Fila {
            id: code

            visible: !!level.account && qr.modules !== ""
            text: App.tr("Code for the phone")
            note: App.tr("The address and the fingerprint, nothing secret")
            kind: "pliega"
            onOpened: {
                qr.opener = code;
                qr.open();
            }
        }

        Fila {
            id: remove

            text: App.tr("Remove server…")
            noteTone: "coral"
            note: App.tr("This computer leaves the account")
            kind: "pliega"
            onOpened: level.page.panels.remove(remove)
        }
    }

    AccountQr {
        id: qr

        server: level.account ? level.account.url : ""
        fingerprint: level.pin
    }
}
