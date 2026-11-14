import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.ScrollablePage {
    id: page

    property var repos: []
    property string error
    readonly property bool examples: repos.some(r => r.bundled)
    readonly property alias link: link
    readonly property alias list: list

    function load() {
        App.query("List", [], "Repos1", reply => {
            if (!page)
                return;
            page.repos = reply.repos;
            page.error = "";
        }, error => page && (page.error = error.message));
    }

    function name(repo) {
        return repo.bundled ? App.tr("Examples") : repo.name || repo.repo_id.slice(0, 8);
    }

    function exercise() {
        Qt.createComponent("RepositoriesCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }

    title: App.tr("Repositories")
    Component.onCompleted: load()
    actions: [
        Kirigami.Action {
            text: App.tr("Check for news")
            icon.name: "arrows-clockwise"
            enabled: page.repos.length > 0
            onTriggered: App.act("Refresh", [""], "Repos1")
        },
        Kirigami.Action {
            text: App.tr("Back to Explore")
            icon.name: "arrow-left"
            onTriggered: applicationWindow().showPage(4)
        }
    ]

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "repos")
                page.load();
        }
    }

    RepoAddLink {
        id: adder

        opener: link
    }

    Filas {
        Fila {
            id: link

            text: App.tr("Add by link…")
            kind: "pliega"
            explain: App.tr("Paste the address of a repository of yours or someone else's")
            onOpened: adder.open()
        }

        Seccion {
            text: App.tr("Your repositories")
        }

        Estado {
            visible: page.error !== ""
            text: page.error
            tone: "coral"
            icon: "warning"
        }

        Repeater {
            id: list

            model: page.repos

            RepoDetails {
                required property var modelData

                width: parent.width
                repo: modelData
                name: page.name(modelData)
            }
        }

        Fila {
            visible: !page.examples
            text: App.tr("Add the examples again")
            kind: "pliega"
            note: App.tr("The wallpapers that come with the app")
            explain: App.tr("Brings back the bundled examples. They need no connection.")
            onOpened: App.act("AddExamples", [], "Repos1")
        }
    }
}
