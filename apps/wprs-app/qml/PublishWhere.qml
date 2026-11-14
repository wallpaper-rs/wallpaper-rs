import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

PublishStep {
    id: where

    required property var publish
    property string out: ""
    property bool hidden: false
    property int days: 7
    readonly property var account: publish.account ?? ({})
    readonly property string host: String(account.url ?? "").replace(/^[a-z]+:\/\//, "").replace(/\/.*$/, "")
    readonly property string server: account.name ? App.tr("%1, at %2", account.name, host) : host
    readonly property string suggested: publish.editing ? Publishing.parent(publish.page.editor.folder) + "/" + (publish.manifest.id ?? "package") + "-" + publish.version + ".wpak" : publish.file
    readonly property string target: out !== "" ? out : suggested
    readonly property alias repo: repo
    readonly property string action: publish.place === "server" ? App.tr("Upload to %1", host) : publish.place === "repo" ? (repo.making ? App.tr("Create and publish") : App.tr("Add and publish")) : App.tr("Save the file")
    readonly property bool can: publish.place === "server" ? publish.linked : publish.place === "repo" ? repo.ready : publish.editing

    function pick(name) {
        publish.place = name;
        publish.fault = null;
        if (name === "repo")
            repo.look();
    }

    function go() {
        if (can && !publish.work.busy)
            publish.work.run();
    }

    number: 3
    title: App.tr("Where?")
    done: publish.place === "server" ? (hidden ? App.tr("Uploaded to <b>%1</b>, hidden", Publishing.escaped(account.name ?? host)) : App.tr("Uploaded to <b>%1</b>, public", Publishing.escaped(account.name ?? host))) : publish.place === "repo" ? App.tr("Added to <b>%1</b>", Publishing.escaped(repo.name)) : App.tr("Saved as <b>%1</b>", Publishing.escaped(Publishing.base(target)))
    mode: publish.at === 3 ? "abierto" : publish.at > 3 ? "hecho" : "cerrado"
    changeable: false
    Keys.onReturnPressed: go()

    PublishPlace {
        icon: "floppy-disk"
        title: App.tr("Save the file")
        note: App.tr("A file to hand to someone.")
        lit: where.publish.place === "file"
        off: !where.publish.editing
        last: where.publish.memory.place === "file"
        onPicked: where.pick("file")

        RowLayout {
            Layout.fillWidth: true

            Text {
                Layout.fillWidth: true
                text: App.tr("In the folder “%1”, as %2", Publishing.folderName(where.target), Publishing.base(where.target))
                wrapMode: Text.WordWrap
                font.family: Look.letraTexto
                font.pixelSize: Look.tPie
                color: Look.bruma
            }

            QQC2.Button {
                flat: true
                text: App.tr("Change…")
                onClicked: saving.open()
            }
        }
    }

    PublishServer {
        where: where
    }

    PublishPlace {
        icon: "folder-simple-plus"
        title: App.tr("Add to a repository in a folder")
        note: repo.known.length > 0 ? App.tr("%1, or another", repo.known[0].name) : App.tr("You do not have one yet; you can create it here.")
        lit: where.publish.place === "repo"
        last: where.publish.memory.place === "repo"
        onPicked: where.pick("repo")

        PublishRepo {
            id: repo

            Layout.fillWidth: true
            where: where
        }
    }

    PublishFault {
        Layout.fillWidth: true
        where: where
    }

    PublishMore {
        Text {
            text: App.tr("Days until the index expires")
            visible: where.publish.place === "repo"
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }

        QQC2.SpinBox {
            visible: where.publish.place === "repo"
            from: 1
            to: 365
            value: where.days
            onValueModified: where.days = value
        }

        QQC2.CheckBox {
            visible: where.publish.place === "repo"
            text: App.tr("Add it hidden")
            checked: where.hidden
            onToggled: where.hidden = checked
        }
    }

    RowLayout {
        spacing: Look.e3
        visible: where.publish.fault === null

        QQC2.Button {
            highlighted: true
            text: where.publish.work.busy ? where.publish.work.doing : where.action
            enabled: where.can && !where.publish.work.busy
            onClicked: where.go()
        }

        KeyCap {
            visible: where.can
            key: "Intro"
        }
    }

    FileDialog {
        id: saving

        fileMode: FileDialog.SaveFile
        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: where.out = App.path(selectedFile)
    }
}
