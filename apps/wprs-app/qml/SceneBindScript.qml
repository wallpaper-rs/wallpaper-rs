import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Menu {
    id: menu

    required property var page
    property var rows: []

    signal chosen(var source)

    function load() {
        const found = JSON.parse(page.editor.code("list", "{}") || "{}").scripts ?? [];
        const out = [];
        for (const script of found) {
            out.push({
                    "text": App.tr("%1, every frame", script.path),
                    "source": {
                        "script": script.path
                    }
                });
            const settings = Object.keys(script.params ?? ({}));
            for (const call of script.functions ?? []) {
                const source = {
                    "script": script.path,
                    "fn": call
                };
                if (settings.length > 0) {
                    source.params = {};
                    settings.forEach(name => source.params[name] = script.params[name].default);
                }
                out.push({
                        "text": settings.length > 0 ? App.tr("%1, function %2 with the settings %3", script.path, call, settings.join(", ")) : App.tr("%1, function %2", script.path, call),
                        "source": source
                    });
            }
        }
        menu.rows = out;
    }

    title: App.tr("Follow a script")
    onAboutToShow: menu.load()

    QQC2.MenuItem {
        text: App.tr("The scene has no script to follow")
        enabled: false
        visible: menu.rows.length === 0
        height: visible ? implicitHeight : 0
    }

    SceneRepeater {
        id: items

        rows: menu.rows

        delegate: QQC2.MenuItem {
            required property int index

            text: items.rows[index]?.text ?? ""
            onTriggered: menu.chosen(items.rows[index].source)
        }
    }
}
