import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Row {
    id: own

    required property var pkg
    required property Installer installer
    readonly property var entry: pkg && pkg.own ? pkg.own : null
    readonly property alias news: news
    readonly property alias remover: remover
    readonly property alias every: every

    spacing: Look.e3

    QQC2.Button {
        id: whatsNew

        visible: own.entry !== null && !!own.entry.update && (own.entry.changes ?? "") !== ""
        flat: true
        text: App.tr("What's new")
        onClicked: news.open()
    }

    QQC2.Button {
        id: uninstall

        flat: true
        text: App.tr("Uninstall…")
        icon.name: "trash"
        onClicked: remover.open()
    }

    PanelChico {
        id: news

        opener: whatsNew
        title: own.entry ? App.tr("What's new in %1", own.entry.update ?? "") : ""
        text: own.entry ? App.plain(own.entry.changes ?? "") : ""
        quiet: App.tr("Close")
    }

    PanelChico {
        id: remover

        opener: uninstall
        title: App.tr("Uninstall “%1”?", own.pkg ? own.pkg.title : "")
        text: App.tr("You can install it again from Explore.")
        quiet: App.tr("Cancel")
        principal: App.tr("Uninstall")
        principalIcon: "trash"
        borra: true
        onAccepted: own.installer.uninstall(own.entry.id, every.checked ? "" : own.entry.version)
        onRejected: every.checked = false

        QQC2.CheckBox {
            id: every

            text: App.tr("Uninstall all versions")
        }
    }
}
