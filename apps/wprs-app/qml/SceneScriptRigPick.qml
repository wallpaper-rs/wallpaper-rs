import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: pick

    required property var page
    property var listing: ({})
    readonly property var meshes: (page.view.layers ?? []).filter(l => l.type === "mesh" || l.type === "model").map(l => l.id)
    readonly property string figure: meshes[layers.currentIndex] ?? ""
    readonly property var kinds: [["bone", App.tr("Bone")], ["shape", App.tr("Blend shape")], ["impulse", App.tr("Push a body")]]
    readonly property string kind: kinds[what.currentIndex][0]
    readonly property var names: kind === "shape" ? (listing.shapes ?? []) : kind === "impulse" ? (listing.bodies ?? []) : (listing.bones ?? [])
    readonly property var calls: listing.calls ? listing.calls[kind] : []
    readonly property var callWords: ({
            "turn": App.tr("Turn it"),
            "move": App.tr("Move it"),
            "scale": App.tr("Scale it"),
            "read": App.tr("Read it"),
            "reset": App.tr("Let it go"),
            "set": App.tr("Set its weight"),
            "push": App.tr("Push it")
        })
    signal inserted(string code)

    function ask(args) {
        return JSON.parse(page.editor.rig(figure, JSON.stringify({
                    "script_calls": Object.assign({
                        "layer": figure
                    }, args)
                })) || "{}");
    }

    function load() {
        listing = figure !== "" ? ask({}) : ({});
    }

    function insert() {
        const found = ask({
            "kind": kind,
            "name": names[name.currentIndex],
            "what": calls[call.currentIndex]
        });
        if (found.code === undefined)
            return;
        inserted(found.code);
    }

    visible: meshes.length > 0
    onFigureChanged: load()
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            pick.load();
        }

        target: pick.page.editor
    }

    QQC2.Label {
        text: App.tr("Insert a call that moves a mesh")
        font: Kirigami.Theme.smallFont
    }

    RowLayout {
        QQC2.ComboBox {
            id: layers

            model: pick.meshes
        }

        QQC2.ComboBox {
            id: what

            model: pick.kinds.map(k => k[1])
        }

        QQC2.ComboBox {
            id: name

            Layout.fillWidth: true
            model: pick.names
        }

        QQC2.ComboBox {
            id: call

            model: pick.calls.map(c => pick.callWords[c] ?? c)
        }

        QQC2.Button {
            text: App.tr("Insert")
            icon.name: "insert-text"
            enabled: pick.names.length > 0
            onClicked: pick.insert()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: pick.figure !== "" && pick.names.length === 0
        wrapMode: Text.Wrap
        text: pick.kind === "impulse" ? App.tr("This mesh has no bone with a body; give one under Swinging bones.") : App.tr("This mesh has none yet.")
        opacity: 0.7
    }
}
