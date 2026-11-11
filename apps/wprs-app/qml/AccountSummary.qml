import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.FormLayout {
    id: summary

    required property var account
    property bool expanded

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Server:")
        text: [summary.account.name, summary.account.url].filter(t => t).join(" · ")
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Certificate:")
        visible: (summary.account.pin ?? "") !== ""

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.WrapAnywhere
            text: summary.expanded ? App.tr("its own, pinned by the fingerprint %1", summary.account.pin ?? "") : App.tr("its own, pinned")
        }

        QQC2.ToolButton {
            icon.name: "documentinfo"
            text: App.tr("Details…")
            display: QQC2.AbstractButton.IconOnly
            QQC2.ToolTip.text: text
            QQC2.ToolTip.visible: hovered
            checkable: true
            checked: summary.expanded
            onToggled: summary.expanded = checked
        }
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Account:")
        text: summary.account.handle ?? "–"
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("This computer:")
        text: summary.account.device ? App.tr("%1, named %2 for your devices", summary.account.device, summary.account.label) : "–"
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("State:")
        text: App.value(summary.account.state)
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Last sync:")
        text: App.time(summary.account.last_sync) || "–"
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Storage:")
        text: App.tr("%1 of %2", App.bytes(summary.account.used_bytes ?? 0), App.bytes(summary.account.quota_bytes ?? 0))
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Skipped records:")
        visible: (summary.account.quarantined ?? 0) > 0
        text: App.tr("%1 records cannot be decrypted with this key and are skipped", summary.account.quarantined ?? 0)
        wrapMode: Text.Wrap
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Waiting changes:")
        visible: (summary.account.unapplied ?? 0) > 0
        text: App.tr("%1 changes from other devices cannot be applied here yet; they are tried again at every sync", summary.account.unapplied ?? 0)
        wrapMode: Text.Wrap
    }

    QQC2.Label {
        Kirigami.FormData.label: App.tr("Error:")
        visible: !!summary.account.last_error
        text: summary.account.last_error ?? ""
        color: Kirigami.Theme.negativeTextColor
        wrapMode: Text.Wrap
    }
}
