import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

PanelChico {
    id: copying

    property string file
    property var facts: ({})
    property string problem: ""
    property bool changing: false
    property bool more: false
    readonly property bool refused: facts.verdict === "refused"
    readonly property bool ready: facts.id !== undefined && folder.text !== "" && newId.text !== "" && (!refused || mine.checked)
    readonly property Recents paths: Recents {
    }
    readonly property string spot: paths.shown(folder.text)

    function start(path, from, picture) {
        file = path;
        problem = "";
        changing = false;
        more = false;
        mine.checked = false;
        facts = JSON.parse(helper.copyFacts(path));
        if (facts.error !== undefined) {
            problem = facts.error;
            facts = ({});
        } else {
            folder.text = helper.folderFor(App.tr("%1 copy", facts.title));
            newId.text = facts.new_id;
        }
        opener = from;
        cover = picture;
        open();
    }

    function make() {
        const reply = JSON.parse(helper.copyInto(file, folder.text, newId.text, mine.checked));
        if (reply.error !== undefined) {
            problem = reply.error;
            return open();
        }
        App.compose(reply.folder, "", "");
    }

    title: App.tr("Edit a copy of “%1”", facts.title ?? "")
    quiet: App.tr("Close")
    principal: App.tr("Edit a copy")
    principalIcon: "copy"
    onAccepted: make()
    onImplicitHeightChanged: {
        if (visible)
            place();
    }

    SceneEditor {
        id: helper
    }

    Row {
        spacing: Look.e2
        visible: copying.facts.license !== undefined && !copying.refused

        IconImage {
            name: "seal-check"
            sourceSize: Qt.size(Look.icono, Look.icono)
            color: Look.brote
        }

        Text {
            width: Look.panelChico - 2 * Look.e4 - Look.icono - Look.e2
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            font.weight: Font.Medium
            color: Look.escarcha
            text: (copying.facts.conditions ?? []).length > 0 ? App.tr("%1: you can modify it and publish it under your name if you: %2.", copying.facts.license ?? "", (copying.facts.conditions ?? []).join(", ")) : App.tr("%1: you can modify it and publish it under your name.", copying.facts.license ?? "")
        }
    }

    Nota {
        visible: copying.refused
        tone: "ambar"
        text: App.tr("%1 allows no derived works.", copying.facts.license ?? "")
    }

    QQC2.CheckBox {
        id: mine

        visible: copying.refused
        text: App.tr("I made this package or its author allows a copy")
    }

    Nota {
        visible: copying.problem !== ""
        tone: "coral"
        text: copying.problem
    }

    Text {
        width: Look.panelChico - 2 * Look.e4
        visible: !copying.changing && copying.facts.id !== undefined
        wrapMode: Text.WordWrap
        textFormat: Text.StyledText
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
        text: App.tr("It will be created as <b>“%1”</b> in %2", copying.facts.title ?? "", copying.spot)
    }

    QQC2.Button {
        flat: true
        visible: !copying.changing && copying.facts.id !== undefined
        text: App.tr("Change…")
        onClicked: copying.changing = true
    }

    Column {
        visible: copying.changing || (copying.problem !== "" && copying.facts.id === undefined)
        spacing: Look.e2

        QQC2.Label {
            text: App.tr("Folder")
        }

        Row {
            spacing: Look.e2

            QQC2.TextField {
                id: folder

                width: Look.panelChico - 2 * Look.e4 - pick.width - Look.e2
            }

            QQC2.Button {
                id: pick

                text: App.tr("Choose…")
                onClicked: folders.open()
            }
        }
    }

    QQC2.Button {
        flat: true
        visible: copying.facts.id !== undefined && !copying.more
        text: App.tr("More")
        icon.name: "caret-right"
        onClicked: copying.more = true
    }

    Column {
        visible: copying.more
        spacing: Look.e2

        QQC2.Label {
            text: App.tr("Id of the copy")
        }

        QQC2.TextField {
            id: newId

            width: Look.panelChico - 2 * Look.e4
        }
    }

    FolderDialog {
        id: folders

        onAccepted: folder.text = App.path(selectedFolder)
    }
}
