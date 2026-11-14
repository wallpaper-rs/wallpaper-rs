import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: detail

    required property var provider
    readonly property var kind: SourceKinds.of(provider.kind)
    readonly property bool plain: provider.plain_http === true
    property bool purge: false

    signal closed
    signal removed
    signal listed
    signal keyAsked
    signal more

    function enter() {
        rotate.forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: rows.implicitHeight + Look.e3 + tail.implicitHeight
    Keys.onEscapePressed: closed()

    Filas {
        id: rows

        Seccion {
            text: detail.provider.state === "idle" ? App.tr("Up to date") : detail.provider.state === "syncing" ? App.tr("Bringing photos…") : App.tr("Needs your attention")
        }

        Fila {
            text: App.tr("Photos")
            value: App.tr("%1 photos, updated %2", detail.provider.items, SourceKinds.ago(detail.provider.last_sync))
            explain: detail.provider.next_sync ? App.tr("The next update comes %1.", App.time(detail.provider.next_sync)) : ""
        }

        Fila {
            text: App.tr("Connects to")
            value: detail.provider.hosts.join(", ")
            visible: detail.provider.hosts.length > 0
        }

        Fila {
            visible: !!(App.schema.providers || []).find(p => p.kind === detail.provider.kind && p.secret)
            text: App.tr("Key")
            value: detail.provider.has_secret ? App.tr("Saved in your keyring") : App.tr("None")
            words: [{
                    "text": App.tr("Change")
                }, {
                    "text": App.tr("Forget")
                }]
            onWorded: index => index === 0 ? detail.keyAsked() : App.act("ForgetSecret", [detail.provider.id], "Providers1")
        }

        Fila {
            text: App.tr("Stored photos")
            value: App.bytes(detail.provider.cache_bytes)
        }

        Fila {
            visible: detail.kind.kind === "wallhaven"
            kind: "despliega"
            text: App.tr("More options")
            onOpened: detail.more()
        }

        Fila {
            id: rotate

            kind: "despliega"
            text: App.tr("See these photos in the library")
            onOpened: App.navigate(0, "source:" + detail.provider.source)
        }

        Fila {
            kind: "despliega"
            text: App.tr("Make a list with these photos")
            onOpened: detail.listed()
        }

        Fila {
            id: removal

            kind: "despliega"
            text: App.tr("Remove this source")
            onOpened: ask.open()
        }
    }

    Column {
        id: tail

        anchors.top: rows.bottom
        anchors.topMargin: Look.e3
        spacing: Look.e2

        Nota {
            visible: detail.plain
            tone: "coral"
            text: App.tr("This address uses plain http: the key and the pictures travel unprotected.")
        }

        Nota {
            visible: detail.provider.orphan_secret === true
            tone: "ambar"
            text: App.tr("A key was left behind by an earlier source of this name.")
        }

        Nota {
            visible: !!detail.provider.last_error
            tone: "ambar"
            text: detail.provider.last_error || ""
        }
    }

    PanelChico {
        id: ask

        opener: removal
        title: App.tr("Remove this source?")
        text: App.tr("Its photos leave your library only if you choose so below.")
        principal: App.tr("Remove")
        principalIcon: "trash"
        quiet: App.tr("Cancel")
        borra: true
        onAccepted: {
            App.act("Remove", [detail.provider.id, detail.purge], "Providers1");
            close();
            detail.removed();
        }
        onRejected: close()

        QQC2.CheckBox {
            text: App.tr("Also delete its stored photos")
            checked: detail.purge
            onToggled: detail.purge = checked
        }
    }
}
