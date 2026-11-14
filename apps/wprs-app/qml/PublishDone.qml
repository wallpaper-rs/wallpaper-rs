import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: done

    required property var publish
    readonly property var result: publish.result ?? ({})
    readonly property string kind: result.kind ?? ""
    readonly property string named: App.tr("“%1” %2", publish.name, publish.version)
    readonly property var sign: publish.sign

    visible: publish.result !== null
    spacing: Look.e3
    activeFocusOnTab: true
    Keys.onReturnPressed: back.clicked()

    Estado {
        text: App.tr("Done")
        tone: "brote"
    }

    Text {
        Layout.fillWidth: true
        text: done.kind === "file" ? App.tr("%1 is in a file", done.named) : done.kind === "server" ? App.tr("%1 was uploaded to %2", done.named, done.publish.account ? (done.publish.account.name ?? done.publish.where.host) : "") : App.tr("%1 is in %2", done.named, done.result.name ?? "")
        wrapMode: Text.WordWrap
        font.family: Look.letraTitulo
        font.pixelSize: Look.tSeccion
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    Text {
        Layout.fillWidth: true
        text: done.kind === "file" ? App.tr("Hand this file to whoever you want. They add it to their library with “Add wallpapers…”.") : done.kind === "server" ? App.tr("%1 reviews what is uploaded: it stays in review and people see it once it is approved. Then it will be public; to give it to someone, send them the link.", done.publish.where.host) : App.tr("This repository lives in the folder “%1” on your computer. For others to add it, that folder has to be somewhere they can reach, like your server or a shared folder. From there they add it in Explore › Repositories.", Publishing.base(done.result.folder ?? ""))
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tExplica
        color: Look.bruma
    }

    PublishNote {
        Layout.fillWidth: true
        visible: done.kind === "repo" && done.result.made === true
        tone: "brote"
        text: App.tr("Signed, “%1” %2 was added and its list was published", done.publish.name, done.publish.version)
    }

    PublishCopy {
        Layout.fillWidth: true
        text: App.tr("Your signature “%1” has no saved copy; without it you cannot publish as %1 from another computer.", done.sign.keyName)
        visible: done.kind === "file" && done.sign.current !== null && !done.publish.unsigned && !done.sign.copied
        onSave: done.sign.copyAway()
    }

    PublishGive {
        visible: done.kind === "file"
        text: App.tr("%1, in the folder “%2”", Publishing.base(done.result.path ?? ""), Publishing.folderName(done.result.path ?? ""))
        action: App.tr("Show in the folder")
        onActed: Qt.openUrlExternally("file://" + Publishing.parent(done.result.path ?? ""))
    }

    PublishGive {
        visible: done.kind === "server"
        text: done.result.link ?? ""
        action: App.tr("Copy the link")
        copied: done.result.link ?? ""
    }

    PublishNote {
        Layout.fillWidth: true
        visible: done.kind === "server"
        text: App.tr("In review. How it goes: Account › Your uploads.")
    }

    PublishGive {
        visible: done.kind === "repo"
        text: done.result.name ?? ""
        action: App.tr("Copy the address")
        copied: done.result.address ?? ""
    }

    Detalles {
        Layout.fillWidth: true
        visible: done.kind === "repo" && (done.result.address ?? "") !== ""
        details: done.result.address ?? ""
    }

    PublishNote {
        Layout.fillWidth: true
        visible: done.kind === "repo" && (done.result.expires ?? "") !== ""
        text: App.tr("Publish again before %1: on that day the list of the repository expires.", Qt.formatDate(new Date(done.result.expires ?? 0), "dd-MM"))
    }

    RowLayout {
        spacing: Look.e3

        QQC2.Button {
            visible: done.publish.editing && done.kind !== "server"
            text: App.tr("Set as wallpaper")
            onClicked: done.publish.page.editor.apply("")
        }

        QQC2.Button {
            id: back

            highlighted: true
            text: done.publish.editing ? App.tr("Back to the editor") : App.tr("Close")
            onClicked: done.publish.close()
        }
    }
}
