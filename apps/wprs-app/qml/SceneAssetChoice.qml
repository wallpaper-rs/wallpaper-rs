import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: choice

    required property var owner
    property string kind: ""
    readonly property var assets: (owner.page.view.assets ?? []).filter(a => a.kind === (kind === "mesh" ? "model" : kind))

    function add(source) {
        owner.page.run("add_layer", {
            "kind": kind,
            "name": SceneWords.kind(kind),
            "source": source,
            "into": owner.current?.type === "group" ? owner.current.id : undefined
        });
    }

    function offer(wanted) {
        kind = wanted;
        if (assets.length > 1)
            popup();
        else
            add(assets.length === 1 ? assets[0].id : undefined);
    }

    title: App.tr("Which asset")

    Repeater {
        model: choice.assets

        delegate: QQC2.MenuItem {
            required property var modelData

            text: modelData.id
            icon.name: choice.owner.icon(choice.kind)
            onTriggered: choice.add(modelData.id)
        }
    }
}
