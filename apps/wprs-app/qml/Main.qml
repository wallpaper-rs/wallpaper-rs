import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.ApplicationWindow {
    id: root

    property int current: 0
    property var inputs: ({})
    property int from: 0
    property int waiting: 0
    property var held: []
    readonly property var names: ["library", "playlists", "profiles", "providers", "repos", "settings", "account", "scenes", "repositories"]
    readonly property alias shell: shell
    readonly property string kind: current < 0 ? "work" : pageStack.currentItem && pageStack.currentItem.skeleton ? pageStack.currentItem.skeleton : [2, 5, 6, 8].includes(current) ? "options" : "title"
    readonly property string backWord: (shell.menu.all.find(entry => entry[0] === shell.menu.owner(from)) ?? [0, ""])[1]
    readonly property list<Component> pages: [
        Component {
            LibraryPage {
            }
        },
        Component {
            PlaylistsPage {
            }
        },
        Component {
            SettingsPage {
                place: "layouts"
            }
        },
        Component {
            ProvidersPage {
            }
        },
        Component {
            ReposPage {
            }
        },
        Component {
            SettingsPage {
            }
        },
        Component {
            AccountPage {
            }
        },
        Component {
            CreatePage {
            }
        },
        Component {
            RepositoriesPage {
            }
        }
    ]

    function open(component, properties) {
        const old = pageStack.currentItem;
        pageStack.clear();
        old?.destroy();
        pageStack.push(component.createObject(pageStack, properties ?? {}));
    }

    function loadInputs() {
        App.query("GetInputs", [], "Inputs1", reply => root && (root.inputs = reply));
    }

    function work(component, properties) {
        if (current >= 0)
            from = current;
        current = -1;
        open(typeof component === "string" ? Qt.createComponent(component) : component, properties);
        size.roomy();
    }

    function leave() {
        showPage(from);
    }

    function showPassiveNotification(message, timeout, actionText, callBack) {
        shell.notices.show(message, timeout, actionText, callBack);
    }

    function hidePassiveNotification() {
        shell.notices.hide();
    }

    function showPage(index, properties) {
        current = index;
        open(pages[index], properties);
    }

    function activate(page) {
        const index = names.indexOf(page);
        if (index >= 0 && index !== current && pageStack.currentItem?.editor?.dirty !== true)
            showPage(index);
        if (visibility === Window.Minimized)
            showNormal();
        raise();
        requestActivate();
    }

    title: "wallpaper-rs"
    minimumWidth: Kirigami.Units.gridUnit * 30
    minimumHeight: Kirigami.Units.gridUnit * 24
    pageStack.globalToolBar.style: Kirigami.ApplicationHeaderStyle.None
    Component.onCompleted: {
        size.restore();
        showPage(Math.max(0, names.indexOf(App.backend.requested())));
        loadInputs();
    }
    Frame {
        id: shell

        host: root
        focus: true
    }

    WindowSize {
        id: size

        host: root
    }

    Connections {
        target: App.backend

        function onActivated(page) {
            root.activate(page);
        }

        function onChanged(topic) {
            if (topic === "inputs" || topic === "state")
                root.loadInputs();
        }
    }

    PermisosAsk {
        host: root
    }

    StillAsk {
    }

    LayoutOffer {
        host: root
    }

    MainRoutes {
        host: root
    }

    Tour {
        host: root
    }

    LookWindow {
        host: root
        tight: shell.titleFrame.tight
    }
}
