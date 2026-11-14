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
    readonly property Component grants: Component {
        SettingsGrants {
            page: list.page
        }
    }
    readonly property Component sound: Component {
        SettingsSound {
            page: list.page
        }
    }
    readonly property Component media: Component {
        SettingsMedia {
            page: list.page
        }
    }
    readonly property Component lights: Component {
        SettingsRgb {
            page: list.page
        }
    }
    readonly property Component advanced: Component {
        SettingsAdvanced {
            page: list.page
        }
    }
    readonly property Component depth: Component {
        SettingsDepth {
            page: list.page
        }
    }
    readonly property Component version: Component {
        SettingsVersion {
            page: list.page
        }
    }
    readonly property Component testing: Component {
        SettingsTesting {
            page: list.page
        }
    }
}
