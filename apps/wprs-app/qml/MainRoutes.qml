import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: routes

    required property var host

    Connections {
        target: App

        function onNavigate(page, query) {
            routes.host.showPage(page, {
                    "query": query
                });
        }

        function onEdit(item) {
            routes.host.work(editor, {
                    "item": item
                });
        }

        function onCompose(folder, starter, text) {
            const kind = starter ? (["web", "preset", "image", "video"].includes(starter) ? starter : "scene") : probe.kindOf(folder, text);
            routes.host.work(["scene", "assets"].includes(kind) ? composer : packager, {
                    "folder": folder,
                    "starter": starter,
                    "heading": starter ? text : "",
                    "source": starter ? "" : text
                });
        }
    }

    Component {
        id: editor

        EditPage {
        }
    }

    Component {
        id: composer

        ScenePage {
        }
    }

    Component {
        id: packager

        PackagePage {
        }
    }

    SceneEditor {
        id: probe
    }
}
