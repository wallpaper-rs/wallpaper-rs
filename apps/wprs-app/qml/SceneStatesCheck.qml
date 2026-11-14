import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    readonly property SceneFit fit: SceneFit {}

    function made(file, args, width) {
        return Qt.createComponent(file).createObject(page, Object.assign({
            "page": page,
            "width": width,
            "height": 900
        }, args));
    }

    function chosen(pointer, wanted) {
        page.editor.select(pointer);
        const side = made("SceneFields.qml", {}, Look.lateralDer);
        try {
            fit.fits("normal " + pointer, side, wanted);
        } finally {
            side.destroy();
        }
    }

    function normal() {
        const before = page.editor.selected;
        chosen("/scene/canvas", [App.tr("Bright parts glow"), App.tr("Layers move in depth"), App.tr("Light around"), App.tr("Light from above")]);
        const layer = (page.view.layers ?? []).find(item => /^\/scene\/layers\/\d+$/.test(item.pointer));
        if (!layer)
            throw new Error("normal: the scene has no layer to choose");
        chosen(layer.pointer, [layer.id, SceneWords.kind(layer.type)]);
        page.editor.select(before);
    }

    function seeAs() {
        const panel = made("ScenePreviewPanel.qml", {}, Look.panelChico - 2 * Look.e3);
        try {
            fit.fits("see as", panel, [App.tr("See the scene as on a"), App.tr("On battery"), App.tr("Reduced motion"), App.tr("Restart the preview"), App.tr("Effects on this computer")].concat(SceneLook.names));
        } finally {
            panel.destroy();
        }
    }

    function error() {
        const layer = (page.view.layers ?? [])[0];
        const pointer = layer.pointer + "/opacity";
        const row = made("SceneProblemRow.qml", {
            "problem": {
                "pointer": pointer,
                "message": pointer + ": the opacity is out of range",
                "warning": false,
                "fix": {
                    "set": {
                        "pointer": pointer,
                        "value": 1
                    }
                }
            }
        }, Look.lateralDer - Look.e4 - Look.e3 - Look.linea);
        try {
            fit.fits("error", row, [App.tr("%1: %2", layer.id, "the opacity is out of range"), App.tr("Go"), App.tr("Fix")]);
        } finally {
            row.destroy();
        }
        const list = made("SceneProblems.qml", {
            "scriptError": "",
            "scriptLines": []
        }, Look.lateralDer - Look.e4 - Look.e3 - Look.linea);
        const count = (page.view.problems ?? []).length;
        try {
            fit.fits("problems", list, [count > 0 ? App.tr("Problems (%1)", count) : App.tr("No problems")]);
        } finally {
            list.destroy();
        }
    }

    function exercise() {
        normal();
        seeAs();
        error();
    }
}
