import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: input

    required property var page
    required property var field
    readonly property var value: field.shown
    readonly property bool bound: field.source !== "" && field.source !== "keys"
    readonly property var both: typeof value === "object" && value !== null && value.length >= 2 ? [Number(value[0]), Number(value[1])] : [Number(value ?? 0), Number(value ?? 0)]

    readonly property Component editor: ({
            "number": editors.number,
            "integer": editors.number,
            "bool": editors.check,
            "enum": choiceEditor,
            "variant": choiceEditor,
            "color": colorEditor,
            "accent": accentEditor,
            "framerate": rateEditor,
            "rgb": colorEditor,
            "condition": hosts.condition,
            "vec2": editors.pair,
            "curve": editors.curve,
            "stops": hosts.pairs,
            "points": hosts.pairs,
            "styles": hosts.styles,
            "emitters": hosts.emitters,
            "rules": hosts.rules,
            "smooth": hosts.smooth,
            "list": editors.list,
            "ref:mask": editors.mask,
            "ref:model": editors.rig,
            "multiline": hosts.multiline,
            "ref:font": hosts.fonts,
            "format": hosts.format,
            "texts": hosts.texts,
            "markup": hosts.texts,
            "flags": hosts.meta,
            "lines": hosts.meta,
            "computed": hosts.meta,
            "ref:thumbnail": hosts.meta,
            "ref:animated": hosts.meta,
            "ref:trigger": hosts.trigger
        })[input.field.widget] ?? (input.field.widget.startsWith("package:") ? hosts.authoring : /^(refs|map):|^ref:reach$/.test(input.field.widget) ? editors.refs : input.field.widget.startsWith("ref:") ? choiceEditor : editors.raw)
    readonly property bool oneRow: !bound && (["number", "integer", "bool", "enum", "variant", "color", "rgb", "vec2"].includes(field.widget) || editor === choiceEditor) && !(["number", "integer"].includes(field.widget) && (field.options ?? []).length > 0)

    function commit(value, gesture) {
        page.change(field.pointer, value, gesture);
    }

    function parse(text, fallback) {
        const parsed = Number(String(text).replace(",", "."));
        return Number.isFinite(parsed) ? parsed : fallback;
    }

    function at(list, option) {
        return SceneOptions.at(list, option);
    }

    function name(option) {
        return SceneOptions.name(field, option);
    }

    Layout.leftMargin: field.depth * Kirigami.Units.largeSpacing

    Rectangle {
        Layout.fillWidth: true
        Layout.preferredHeight: 2
        visible: input.page.spot === input.field.pointer
        color: Look.coral
    }

    SceneInputHead {
        input: input
        editor: input.oneRow ? input.editor : null
    }

    QQC2.Button {
        visible: input.bound
        text: App.tr("Follows %1", SceneWords.binding(input.field.value))
        icon.name: "document-edit"
        flat: true
        Layout.fillWidth: true
        onClicked: input.page.editor.select(input.field.pointer)
    }

    SceneEditors {
        id: editors

        input: parent
    }

    SceneHosts {
        id: hosts

        input: parent
    }

    Loader {
        Layout.fillWidth: true
        active: !input.bound && input.field.widget !== "group" && !input.oneRow
        sourceComponent: input.editor
    }

    SceneTextureTools {
        Layout.fillWidth: true
        visible: !input.bound && kinds.length > 0
        input: parent
    }

    Component {
        id: choiceEditor

        QQC2.ComboBox {
            readonly property bool variant: input.field.widget === "variant"
            readonly property var known: variant || input.field.required ? input.field.options : [null].concat(input.field.options)
            readonly property var options: variant || input.at(known, input.value) >= 0 || input.value === null ? known : known.concat([input.value])

            model: options.map(option => option === null ? App.tr("Not set") : input.name(option))
            currentIndex: variant ? options.findIndex(option => Object.keys(input.field.value ?? {}).includes(option) || option === input.field.value) : input.at(options, input.value)
            onActivated: index => {
                if (options[index] === null)
                    input.page.run("remove", {
                            "pointer": input.field.pointer
                        });
                else
                    input.commit(variant ? input.field.sample[options[index]] : options[index]);
            }
        }
    }

    Component {
        id: accentEditor

        SceneAccent {
            owner: input
        }
    }

    Component {
        id: rateEditor

        SceneFrameRate {
            owner: input
        }
    }

    Component {
        id: colorEditor

        SceneColor {
            alpha: input.field.widget !== "rgb"
            hex: typeof input.value === "string" ? input.value : "#ffffff"
            onPicked: hex => input.commit(hex)
        }
    }
}
