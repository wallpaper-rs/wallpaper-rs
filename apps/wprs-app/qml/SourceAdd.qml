import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: add

    required property string kind
    required property var taken
    readonly property var entry: SourceKinds.of(kind)
    readonly property var schema: (App.schema.providers || []).find(p => p.kind === kind) || ({
            "options": [],
            "secret": null
        })
    readonly property var needed: schema.options.filter(o => o.required)
    property var values: ({})
    property string name: SourceKinds.fresh(kind, taken)
    property string error
    property string errorName
    readonly property string address: values.url || values.server || ""
    readonly property bool plain: /^http:\/\/(?!localhost|127\.|\[::1\])/i.test(address)
    readonly property bool noKeyring: errorName.endsWith("KeyringUnavailable")
    readonly property bool ready: needed.every(o => (values[o.key] || "") !== "") && (!schema.secret || !schema.secret.required || key.text !== "") && name !== "" && !taken.includes(name)

    signal done(string id)
    signal cancelled
    signal keyFiler

    function enter() {
        named.forceActiveFocus(Qt.TabFocusReason);
    }

    function submit() {
        if (!ready)
            return;
        errorName = "";
        App.addProvider(name, kind, values, key.text, () => {
            if (add) {
                key.clear();
                add.done(add.name);
            }
        }, e => {
            if (add) {
                add.errorName = e.name;
                add.error = e.message;
            }
        });
    }

    implicitHeight: rows.implicitHeight + Look.e5 + tail.implicitHeight
    Keys.onEscapePressed: cancelled()

    Filas {
        id: rows

        Seccion {
            text: add.entry.line
        }

        Fila {
            visible: add.kind === "wallhaven"
            text: App.tr("What to look for")
            kind: "campo"
            placeholder: App.tr("nature, city at night, space…")
            onEdited: text => add.values = Object.assign({}, add.values, {
                    "query": text
                })
        }

        Repeater {
            model: add.needed

            delegate: Fila {
                required property var modelData

                text: Labels.field(modelData.key)
                kind: "campo"
                onEdited: text => add.values = Object.assign({}, add.values, {
                        [modelData.key]: text
                    })
            }
        }

        Fila {
            id: secretRow

            visible: !!add.schema.secret
            text: add.schema.secret ? Labels.field(add.schema.secret.name) : ""
            kind: "despliega"
            value: key.text === "" ? (add.schema.secret && add.schema.secret.required ? App.tr("Required") : App.tr("Optional")) : "••••••••"
            onOpened: pad.open()
        }

        Fila {
            id: named

            text: App.tr("Name")
            kind: "campo"
            fieldText: add.name
            note: add.taken.includes(add.name) ? App.tr("You already have a source with this name") : ""
            noteTone: "coral"
            onEdited: text => add.name = SourceKinds.slug(text)
        }
    }

    Column {
        id: tail

        anchors.top: rows.bottom
        anchors.topMargin: Look.e5
        width: rows.width
        spacing: Look.e3

        Nota {
            visible: add.plain
            tone: "coral"
            text: App.tr("This address uses plain http: the key and the pictures travel unprotected.")
        }

        Nota {
            visible: add.error !== ""
            tone: "coral"
            text: add.noKeyring ? App.tr("No keyring answers, so the key cannot be saved.") : add.error
        }

        QQC2.Button {
            visible: add.noKeyring
            flat: true
            text: App.tr("Keep keys in a private file…")
            onClicked: add.keyFiler()
        }

        Row {
            spacing: Look.huecoBoton

            QQC2.Button {
                anchors.verticalCenter: parent.verticalCenter
                flat: true
                text: App.tr("Cancel")
                onClicked: add.cancelled()
            }

            QQC2.Button {
                anchors.verticalCenter: parent.verticalCenter
                highlighted: true
                enabled: add.ready
                text: App.tr("Add and bring photos")
                icon.name: "plus"
                onClicked: add.submit()
            }
        }

        Nota {
            text: App.tr("The first photos reach your library right away. Then you can rotate them with one button.")
        }
    }

    PanelChico {
        id: pad

        opener: secretRow
        title: add.schema.secret ? Labels.field(add.schema.secret.name) : ""
        principal: App.tr("Save")
        quiet: App.tr("Cancel")
        onAccepted: close()
        onRejected: {
            key.clear();
            close();
        }

        QQC2.TextField {
            id: key

            width: Look.panelChico - 2 * Look.e4
            echoMode: TextInput.Password
            focus: true
            onAccepted: pad.close()
        }
    }
}
