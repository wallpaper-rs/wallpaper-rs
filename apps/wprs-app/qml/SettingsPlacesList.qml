import QtQuick

QtObject {
    id: list

    required property var page
    readonly property Component more: Component {
        SettingsMore {
            page: list.page
        }
    }
    readonly property Component folders: Component {
        SettingsFolders {
            page: list.page
        }
    }
    readonly property Component lists: Component {
        SettingsLists {
            page: list.page
        }
    }
    readonly property Component pause: Component {
        SettingsPause {
            page: list.page
        }
    }
    readonly property Component screen: Component {
        SettingsScreenMore {
            page: list.page
        }
    }
    readonly property Component layouts: Component {
        SettingsLayouts {
            page: list.page
        }
    }
}
