import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        const files = Qt.createComponent("SceneLayerFiles.qml").createObject(page, {
                "owner": {
                    "page": page,
                    "current": null
                }
            });
        if (files.count !== 5)
            throw new Error("files: the menu offers " + files.count + " kinds of file, not 5");
        for (const kind of ["image", "video", "sound", "mesh"]) {
            files.kind = kind;
            if (!/\(\*\.[a-z0-9]+/.test(files.dialog.nameFilters[0] ?? ""))
                throw new Error("files: no file filter for " + kind);
        }
        files.destroy();
        const calls = [];
        const importer = Qt.createComponent("SceneModelImport.qml").createObject(page, {
                "page": {
                    "view": ({}),
                    "editor": {
                        "rig": () => "null"
                    },
                    "run": (action, args) => calls.push([action, args])
                },
                "into": ""
            });
        importer.file = "/somewhere/Crate Box.glb";
        importer.accepted();
        if (calls.length !== 1 || calls[0][0] !== "add_layer" || calls[0][1].kind !== "model" || calls[0][1].file !== "/somewhere/Crate Box.glb" || calls[0][1].name !== "crate_box")
            throw new Error("files: accepting a 3D model file does not add its layer in one step: " + JSON.stringify(calls));
        importer.destroy();
    }
}
