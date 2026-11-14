import QtQuick
import io.github.wallpaper_rs.app

Item {
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

    function rename() {
        renaming.ask("layer", owner.current.id);
    }

    SceneRename {
        id: renaming

        page: clip.page
    }
}
