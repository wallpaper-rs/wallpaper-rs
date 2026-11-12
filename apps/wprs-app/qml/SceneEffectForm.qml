import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: form

    required property var page
    property string effect: ""
    property var source: ({})
    readonly property var parsed: source.parsed ?? {}
    readonly property var kinds: source.kinds ?? []
    readonly property var floats: (parsed.params ?? []).filter(p => p.type === "float").map(p => p.id)
    property var report: ({})
    property var costs: null

    function load() {
        form.source = JSON.parse(page.editor.rig("", JSON.stringify({
                        "shader_source": {
                            "effect": form.effect
                        }
                    })) || "null") ?? {};
        form.costs = JSON.parse(page.editor.rig("", JSON.stringify({
                        "effect_cost": {
                            "effect": form.effect,
                            "optional": optional.checked
                        }
                    })) || "null");
        checker.check(form.effect, form.source.descriptor ?? "", form.source.wgsl ?? "");
    }

    function show(effect) {
        form.effect = effect;
        form.load();
        form.open();
    }

    function edit(kind, args) {
        const step = {};
        step[kind] = Object.assign({
                "effect": form.effect
            }, args);
        page.run("shader", step);
        form.load();
    }

    function param(id, args) {
        edit("param", Object.assign({
                    "id": id
                }, args));
    }

    function pass(index, args, lighter) {
        const more = Object.assign({}, args, lighter ? {
                "lighter": true
            } : {});
        edit("pass", index === null ? more : Object.assign({
                    "index": index
                }, more));
    }

    function field(name, value) {
        edit("field", {
                "field": name,
                "value": value
            });
    }

    title: App.tr("Descriptor of %1", form.effect)
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 46

    ShaderCheck {
        id: checker

        onReportChanged: form.report = JSON.parse(report || "{}")
    }

    ColumnLayout {
        implicitWidth: Kirigami.Units.gridUnit * 44

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: true
            type: checker.checking ? Kirigami.MessageType.Information : form.report.ok ? Kirigami.MessageType.Positive : Kirigami.MessageType.Error
            text: checker.checking ? App.tr("Checking…") : form.report.ok ? App.tr("Builds") : (form.report.error ?? "")
        }

        Kirigami.Heading {
            level: 5
            text: App.tr("Parameters")
        }

        SceneRepeater {
            id: params

            rows: form.parsed.params ?? []

            delegate: SceneEffectParam {
                Layout.fillWidth: true
                form: form
                param: params.rows[index]
            }
        }

        RowLayout {
            QQC2.TextField {
                id: fresh

                placeholderText: App.tr("Id of a new parameter")
                validator: RegularExpressionValidator {
                    regularExpression: /[a-z0-9_]{1,32}/
                }
            }

            QQC2.ComboBox {
                id: kind

                model: form.kinds.map(k => SceneEffects.type(k))
            }

            QQC2.Button {
                icon.name: "list-add"
                text: App.tr("Add the parameter")
                enabled: fresh.acceptableInput
                onClicked: {
                    form.param(fresh.text, {
                            "set": {
                                "type": form.kinds[kind.currentIndex]
                            }
                        });
                    fresh.text = "";
                }
            }
        }

        Kirigami.Heading {
            level: 5
            visible: (form.source.stage ?? "layer") === "layer"
            text: App.tr("Passes")
        }

        SceneRepeater {
            id: passes

            rows: (form.source.stage ?? "layer") === "layer" ? (form.parsed.passes ?? []) : []

            delegate: SceneEffectPass {
                Layout.fillWidth: true
                form: form
                pass: passes.rows[index]
            }
        }

        QQC2.Button {
            visible: (form.source.stage ?? "layer") === "layer"
            icon.name: "list-add"
            text: App.tr("Add a pass")
            onClicked: form.pass(null, {})
        }

        SceneEffectFields {
            Layout.fillWidth: true
            form: form
        }

        SceneEffectLighter {
            Layout.fillWidth: true
            form: form
        }

        QQC2.CheckBox {
            id: optional

            text: App.tr("Cost when a layer marks it optional")
            onToggled: form.load()
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: form.costs ? App.tr("Cost per quality level: %1", (form.costs.levels ?? []).map(l => l.quality + " " + (l.units === null ? App.tr("dropped") : (l.form === "lighter" ? App.tr("lighter %1", Math.round(l.units * 100) / 100) : Math.round(l.units * 100) / 100))).join(", ")) : ""
        }
    }
}
