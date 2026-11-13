import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: menu

    property int current: 0
    property int at: 0
    readonly property var main: [[0, App.tr("Library")], [4, App.tr("Explore")], [1, App.tr("Lists")], [3, App.tr("Sources")], [7, App.tr("Create")]]
    readonly property var apart: [[5, App.tr("Settings")], [6, App.tr("Account")]]
    readonly property var all: main.concat(apart)
    readonly property bool keyTarget: true

    signal chosen(int page)

    function owner(page) {
        return page === 2 ? 5 : page;
    }

    function move(by) {
        at = Math.max(0, Math.min(all.length - 1, at + by));
    }

    function word(index) {
        return index < main.length ? first.itemAt(index) : second.itemAt(index - main.length);
    }

    implicitWidth: Math.max(top.implicitWidth, bottom.implicitWidth)
    implicitHeight: bottom.y + bottom.implicitHeight
    activeFocusOnTab: true
    Accessible.role: Accessible.PageTabList
    Accessible.name: App.tr("Sections")
    onActiveFocusChanged: activeFocus && (at = Math.max(0, all.findIndex(entry => entry[0] === owner(current))))
    Keys.onUpPressed: move(-1)
    Keys.onDownPressed: move(1)
    Keys.onReturnPressed: chosen(all[at][0])
    Keys.onEnterPressed: chosen(all[at][0])
    Keys.onSpacePressed: chosen(all[at][0])

    Column {
        id: top

        Repeater {
            id: first

            model: menu.main

            delegate: MenuWord {
                required property var modelData
                required property int index

                page: modelData[0]
                text: modelData[1]
                apart: false
                current: menu.owner(menu.current) === page
                lit: menu.activeFocus && menu.at === index
                onClicked: menu.chosen(page)
            }
        }
    }

    Column {
        id: bottom

        y: top.implicitHeight + Look.e4

        Repeater {
            id: second

            model: menu.apart

            delegate: MenuWord {
                required property var modelData
                required property int index

                page: modelData[0]
                text: modelData[1]
                apart: true
                current: menu.owner(menu.current) === page
                lit: menu.activeFocus && menu.at === menu.main.length + index
                onClicked: menu.chosen(page)
            }
        }
    }
}
