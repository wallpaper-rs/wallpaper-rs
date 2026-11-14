import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property int tab: SettingsPlaces.tab(place)
    property string place: ""
    property string query: ""
    property int screen: 0
    property Item opener: null
    property var sources: []
    property var inputs: ({})
    property var playlists: []
    property var profiles: []
    property string active
    readonly property var outputs: App.outputs
    readonly property string output: outputs[Math.min(screen, outputs.length - 1)] ?? ""
    readonly property var tabs: [wallpapers, screens, machine]
    readonly property var words: [App.tr("Wallpapers"), App.tr("Screens"), App.tr("Device")]
    readonly property var entries: tabs.reduce((all, owner, at) => all.concat(owner.rows.map(row => Object.assign({
                    "tab": at
                }, row))), [])
    readonly property alias found: found
    readonly property var route: place === "" ? [] : [words[SettingsPlaces.tab(place)]].concat(SettingsPlaces.trail(place, output))
    readonly property var keyHints: SettingsPlaces.hints(query !== "", place, tab)
    readonly property alias shown: shown
    readonly property Component head: Component {
        SettingsSearch {
            owner: page
        }
    }

    function load() {
        App.query("ListSources", [], "Library1", reply => page && (page.sources = reply.sources));
        App.query("GetInputs", [], "Inputs1", reply => page && (page.inputs = reply));
        App.query("List", [], "Playlists1", reply => page && (page.playlists = reply.playlists));
        App.query("List", [], "Profiles1", reply => {
            if (!page)
                return;
            page.profiles = reply.profiles;
            page.active = reply.active ?? "";
        });
    }

    function open(where, from) {
        opener = from ?? null;
        place = where;
        enter();
    }

    function enter() {
        const first = SettingsReach.rows(shown.item ?? tabs[tab])[0];
        if (first)
            first.forceActiveFocus(Qt.TabFocusReason);
    }

    function reach(entry) {
        query = "";
        tab = entry.tab;
        place = entry.place ?? "";
        Qt.callLater(() => {
            const row = SettingsReach.rows(shown.item ?? tabs[tab]).find(fila => fila.text === entry.text);
            if (row)
                row.forceActiveFocus(Qt.TabFocusReason);
            else
                enter();
        });
    }

    function routeBack() {
        const back = opener;
        const left = SettingsPlaces.word(place, output);
        place = SettingsPlaces.up(place);
        opener = null;
        const up = place;
        if (up !== "")
            Qt.callLater(() => place === up && (SettingsReach.rows(shown.item).find(row => row.text === left) ?? shown.item).forceActiveFocus(Qt.TabFocusReason));
        else if (back)
            back.forceActiveFocus(Qt.TabFocusReason);
    }

    function exercise() {
        Qt.createComponent("SettingsCheck.qml").createObject(page).exercise();
        Qt.createComponent("SettingsTabsCheck.qml").createObject(page, {
            "page": page
        }).exercise();
    }

    title: App.tr("Settings")
    padding: 0
    Component.onCompleted: load()

    SettingsWatch {
        owner: page
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "playlists" || topic === "profiles")
                page.load();
        }
    }

    Filtro {
        id: bar

        visible: page.place === "" && page.query === ""
        sections: true
        label: App.tr("Tabs")
        words: page.words.map(text => ({
                    "text": text
                }))
        current: page.tab
        onPicked: index => page.tab = index
    }

    Flickable {
        id: scroll

        y: bar.visible ? bar.height + Look.e5 : 0
        width: parent.width
        height: parent.height - y
        contentHeight: column.height + Look.e5
        boundsBehavior: Flickable.StopAtBounds
        clip: true

        Item {
            id: column

            width: scroll.width
            height: Math.max(found.visible ? found.height : 0, shown.active ? shown.height : 0, wallpapers.visible ? wallpapers.height : 0, screens.visible ? screens.height : 0, machine.visible ? machine.height : 0)

            SettingsWallpapers {
                id: wallpapers

                width: parent.width
                page: page
                visible: page.query === "" && page.place === "" && page.tab === 0
            }

            SettingsScreens {
                id: screens

                width: parent.width
                page: page
                visible: page.query === "" && page.place === "" && page.tab === 1
            }

            SettingsMachine {
                id: machine

                width: parent.width
                page: page
                visible: page.query === "" && page.place === "" && page.tab === 2
            }

            Loader {
                id: shown

                width: parent.width
                active: page.query === "" && page.place !== ""
                sourceComponent: places[page.place] ?? null
            }

            SettingsFound {
                id: found

                width: parent.width
                page: page
                visible: page.query !== ""
            }
        }
    }

    SettingsPlacesList {
        id: places

        page: page
    }

    Connections {
        target: page.Window.window

        function onActiveFocusItemChanged() {
            const item = page.Window.activeFocusItem;
            if (item && SettingsReach.inside(item, column))
                SettingsReach.reveal(scroll, item);
        }
    }
}
