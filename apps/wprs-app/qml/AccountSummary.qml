import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: summary

    property var page: null
    readonly property var account: page && page.account ? page.account : ({})

    function enter() {
        first.forceActiveFocus(Qt.TabFocusReason);
    }

    width: parent ? parent.width : 0
    height: list.height

    Filas {
        id: list

        Fila {
            id: first

            text: App.tr("Server")
            value: summary.account.name || ""
            note: summary.account.url || ""
        }

        Fila {
            visible: (summary.account.pin || "") !== ""
            text: App.tr("Certificate")
            value: App.tr("its own, pinned")
            explain: App.tr("Pinned by the fingerprint %1", summary.account.pin || "")
        }

        Fila {
            text: App.tr("Account")
            value: summary.account.handle || "–"
        }

        Fila {
            text: App.tr("This computer")
            value: summary.account.device ? summary.account.label : "–"
            note: summary.account.device ? App.tr("%1 for the server", summary.account.device) : ""
        }

        Fila {
            text: App.tr("State")
            value: App.value(summary.account.state || "")
            note: App.tr("Last sync: %1", App.time(summary.account.last_sync) || "–")
        }

        Fila {
            text: App.tr("Storage")
            value: App.tr("%1 of %2", App.bytes(summary.account.used_bytes || 0), App.bytes(summary.account.quota_bytes || 0))
        }

        Fila {
            text: App.tr("What syncs")
            explain: App.tr("Titles, tags, ratings, favorites and image edits of your library, playlists, profiles, presets and settings.")
        }

        Fila {
            text: App.tr("What stays on each computer")
            explain: App.tr("Which wallpaper is set on each screen and how it is placed, and the settings that depend on the computer.")
        }

        Fila {
            text: App.tr("Wallpaper files")
            explain: App.tr("Only the ones you send with “Send files to my other computers”; your other computers download them from the Library or from here.")
        }

        Fila {
            visible: (summary.account.quarantined || 0) > 0
            text: App.tr("Skipped records")
            value: String(summary.account.quarantined || 0)
            explain: App.tr("%1 records cannot be decrypted with this key and are skipped", summary.account.quarantined || 0)
        }

        Fila {
            visible: (summary.account.unapplied || 0) > 0
            text: App.tr("Waiting changes")
            value: String(summary.account.unapplied || 0)
            explain: App.tr("%1 changes from other devices cannot be applied here yet; they are tried again at every sync", summary.account.unapplied || 0)
        }

        Fila {
            visible: !!summary.account.last_error
            text: App.tr("Error")
            status: summary.account.last_error || ""
            statusTone: "coral"
            explain: App.tr("The last sync stopped with this error.")
        }
    }
}
