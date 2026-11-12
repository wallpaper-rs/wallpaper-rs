import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: table

    required property var page
    required property var script
    readonly property var ids: Object.keys(script.params ?? ({}))
    property int revision: 0
    readonly property var kinds: [["number", App.tr("Number")], ["bool", App.tr("Switch")], ["text", App.tr("Text")], ["color", App.tr("Color")], ["vec2", App.tr("Pair of numbers")]]

    function commit(args) {
        const body = Object.assign({
                "path": script.path
            }, args);
        page.run("script", {
                "param": body
            });
        if (page.editor.error !== "")
            revision++;
    }

    function add() {
        let n = 1;
        while (ids.includes("setting_" + n))
            n++;
        commit({
                "id": "setting_" + n,
                "type": "number"
            });
    }

    spacing: Kirigami.Units.smallSpacing

    QQC2.Label {
        Layout.fillWidth: true
        text: App.tr("Settings of the module")
        font.bold: true
    }

    SceneRepeater {
        id: list

        rows: table.ids.map(id => ({
                        "id": id
                    }))

        delegate: SceneScriptParam {
            required property int index
            readonly property string ident: list.rows[index]?.id ?? ""

            owner: table
            name: ident
            spec: table.script.params[ident] ?? ({})
            Layout.fillWidth: true
        }
    }

    QQC2.Button {
        icon.name: "list-add"
        text: App.tr("Add a setting")
        onClicked: table.add()
    }
}
