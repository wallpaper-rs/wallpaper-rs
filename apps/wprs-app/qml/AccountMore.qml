import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: more

    property var page: null
    readonly property var account: page ? page.account : null
    readonly property bool linked: !!page && page.linked
    readonly property bool encrypted: !!account && account.encrypted === true
    readonly property string recovery: account ? account.recovery : "off"
    readonly property var words: ({
            "off": App.tr("No recovery passphrase: if every device is lost, so is the synced data."),
            "current": App.tr("The server keeps the current sync key, locked with your recovery passphrase."),
            "stale": App.tr("A device that may know the recovery passphrase was revoked, so the server keeps an older sync key: set a new, different passphrase.")
        })
    readonly property alias plain: plain

    function enter() {
        (approve.visible ? approve : plain).forceActiveFocus(Qt.TabFocusReason);
    }

    width: parent ? parent.width : 0
    height: column.height

    Column {
        id: column

        width: more.width
        spacing: Look.e5

        Filas {
            Seccion {
                visible: more.linked
                text: App.tr("Another device")
            }

            Fila {
                id: approve

                visible: more.linked
                text: App.tr("Approve another device…")
                note: App.tr("A new device of yours shows a code")
                kind: "pliega"
                explain: App.tr("Check that both screens show the same number before you approve.")
                onOpened: more.page.codes.approve(approve)
            }

            Seccion {
                visible: more.encrypted
                text: App.tr("Recovery passphrase")
            }

            Fila {
                id: escrow

                visible: more.encrypted && more.account.state !== "needs_key"
                text: more.recovery === "off" ? App.tr("Set a passphrase…") : App.tr("Change the passphrase…")
                kind: "pliega"
                explain: more.words[more.recovery] || ""
                onOpened: more.page.secrets.show(more.page.secrets.escrower, escrow)
            }

            Fila {
                id: unlock

                visible: more.encrypted && more.recovery !== "off" && more.account.state !== "needs_key"
                text: App.tr("Remove the passphrase…")
                kind: "pliega"
                onOpened: more.page.secrets.show(more.page.secrets.unlocker, unlock)
            }

            Fila {
                id: restore

                visible: more.encrypted && more.account.state === "needs_key"
                text: App.tr("Restore with the recovery passphrase…")
                kind: "pliega"
                onOpened: more.page.secrets.show(more.page.secrets.restorer, restore)
            }

            Seccion {
                text: App.tr("Encryption")
            }

            Fila {
                id: plain

                text: App.tr("Allow syncing without encryption")
                note: App.tr("Only for an account that already syncs that way")
                kind: "llave"
                checked: !!more.page && more.page.plain
                value: checked ? App.tr("On") : App.tr("Off")
                explain: App.tr("The operator of the server will be able to read your library, playlists and settings.")
                status: more.account && more.account.encrypted === false ? App.tr("This account syncs without encryption") : ""
                statusTone: "coral"
                onToggled: more.page.plain = !more.page.plain
            }

            Seccion {
                visible: !!more.account
                text: App.tr("Sync key and server")
            }

            Fila {
                visible: !!more.page && more.page.keyed && more.encrypted
                text: App.tr("Replace the sync key")
                kind: "pliega"
                explain: App.tr("Every device of the account gets the new key at its next sync.")
                onOpened: App.act("Rotate", [], "Sync1")
            }

            Fila {
                id: reset

                visible: more.linked
                text: App.tr("Reset synced data…")
                kind: "pliega"
                onOpened: more.page.panels.reset(reset)
            }

            Fila {
                id: remove

                visible: !!more.account
                text: App.tr("Remove server…")
                noteTone: "coral"
                note: App.tr("This computer leaves the account")
                kind: "pliega"
                onOpened: more.page.panels.remove(remove)
            }
        }

        AccountFiles {
            width: Math.min(more.width, Look.filasAncho)
            account: more.account
        }

        AccountSessions {
            width: Math.min(more.width, Look.filasAncho)
            account: more.account
        }

        AccountUploads {
            width: Math.min(more.width, Look.filasAncho)
            visible: !!more.account && items.length > 0
            account: more.account
        }

        AccountKeys {
            width: Math.min(more.width, Look.filasAncho)
            visible: !!more.account
            account: more.account
        }
    }
}
