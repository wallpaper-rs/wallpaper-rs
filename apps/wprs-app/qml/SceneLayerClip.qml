import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Row {
    id: clip

    required property var owner
    readonly property var page: owner.page

    function copy() {
        const ids = [owner.current.id].concat(page.picked.filter(id => id !== owner.current.id));
        const found = JSON.parse(page.editor.rig("", JSON.stringify({
            "copy": {
                "ids": ids
            }
        })) || "null");
        if (found && !found.error)
            SceneClip.share(found);
        return found;
    }

    function paste() {
        page.run("paste", {
            "clip": SceneClip.take() ?? {},
            "into": owner.current?.type === "group" ? owner.current.id : undefined
        });
    }

    QQC2.ToolButton {
        icon.name: "edit-copy-path"
        text: App.tr("Copy the layers")
        display: QQC2.AbstractButton.IconOnly
        enabled: clip.owner.current !== null
        onClicked: clip.copy()
        QQC2.ToolTip.text: App.tr("Copy the selected layer and the ones picked with Ctrl, with what they use, to paste them here or in another scene")
        QQC2.ToolTip.visible: hovered
    }

    QQC2.ToolButton {
        icon.name: "edit-paste"
        text: App.tr("Paste the layers")
        display: QQC2.AbstractButton.IconOnly
        enabled: SceneClip.pasteable
        onClicked: clip.paste()
        QQC2.ToolTip.text: App.tr("Paste the copied layers with free ids, copying the files they use")
        QQC2.ToolTip.visible: hovered
    }

    QQC2.ToolButton {
        icon.name: "edit-rename"
        text: App.tr("Rename the layer")
        display: QQC2.AbstractButton.IconOnly
        enabled: clip.owner.current !== null
        onClicked: renaming.ask("layer", clip.owner.current.id)
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: hovered
    }

    SceneRename {
        id: renaming

        page: clip.page
    }
}
