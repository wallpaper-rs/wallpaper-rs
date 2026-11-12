import QtQuick

Item {
    id: hosts

    required property var input
    readonly property Component pairs: pairsEditor
    readonly property Component condition: conditionEditor
    readonly property Component texts: textsEditor
    readonly property Component meta: metaEditor
    readonly property Component styles: stylesEditor
    readonly property Component emitters: emittersEditor
    readonly property Component rules: rulesEditor
    readonly property Component authoring: packageFields
    readonly property Component fonts: fontsEditor
    readonly property Component multiline: multilineEditor
    readonly property Component format: formatEditor
    readonly property Component trigger: triggerEditor

    visible: false

    Component {
        id: pairsEditor

        ScenePairs {
            host: hosts.input
        }
    }

    Component {
        id: conditionEditor

        SceneCondition {
            host: hosts.input
        }
    }

    Component {
        id: textsEditor

        SceneTexts {
            host: hosts.input
        }
    }

    Component {
        id: metaEditor

        SceneMeta {
            host: hosts.input
        }
    }

    Component {
        id: stylesEditor

        SceneStyles {
            host: hosts.input
        }
    }

    Component {
        id: rulesEditor

        SceneWakeRules {
            host: hosts.input
        }
    }

    Component {
        id: packageFields

        PackageFields {
            host: hosts.input
        }
    }

    Component {
        id: emittersEditor

        SceneEmitterList {
            host: hosts.input
        }
    }

    Component {
        id: multilineEditor

        SceneMultiline {
            host: hosts.input
        }
    }

    Component {
        id: fontsEditor

        SceneFontPicker {
            host: hosts.input
        }
    }

    Component {
        id: formatEditor

        SceneFormat {
            host: hosts.input
        }
    }

        id: triggerEditor

        SceneTrigger {
            host: hosts.input
        }
    }
}
