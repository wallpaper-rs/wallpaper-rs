import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Controls.impl
import io.github.wallpaper_rs.app

FocusScope {
    id: card

    required property var provider
    property bool rotating: false
    readonly property var kind: SourceKinds.of(provider.kind)
    readonly property string problem: provider.state === "needs_secret" ? App.tr("%1 rejected the key", kind.name) : provider.state === "locked" ? App.tr("The keyring is locked") : provider.state === "error" && !provider.offline ? said(provider.last_error) : ""
    readonly property string tone: provider.state === "locked" || provider.state === "needs_secret" ? "coral" : "ambar"
    readonly property bool broken: problem !== ""
    readonly property bool bare: provider.items === 0
    readonly property bool syncing: provider.state === "syncing"

    signal rotate
    signal refresh
    signal details
    signal fix
    signal up
    signal down

    function said(error) {
        const code = /status (?:client|server) error \((\d+)/.exec(error || "");
        if (code)
            return App.tr("%1 answered with error %2", kind.name, code[1]);
        return error || App.tr("It could not reach %1", kind.name);
    }

    function enter() {
        (primary.visible ? primary : refreshing).forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: Math.max(left.implicitHeight, Look.altoBoton * 3)
    Keys.onUpPressed: up()
    Keys.onDownPressed: down()

    Column {
        id: left

        width: Look.fuenteQuien
        spacing: Look.e2

        Row {
            spacing: Look.e2

            IconImage {
                anchors.verticalCenter: parent.verticalCenter
                name: card.kind.icon
                sourceSize: Qt.size(Look.fuenteIcono, Look.fuenteIcono)
                color: Look.luna
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: SourceKinds.heading(card.provider)
                font.family: Look.letraTitulo
                font.pixelSize: Look.tFila
                font.weight: Font.Bold
                color: Look.escarcha
            }
        }

        Estado {
            visible: card.broken
            text: card.problem
            tone: card.tone
        }

        Text {
            visible: card.broken && !card.bare
            text: App.tr("The %1 photos it already brought stay in your library.", card.provider.items)
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }

        Text {
            visible: !card.broken
            text: card.syncing ? App.tr("Bringing photos…") : card.bare ? App.tr("No photos yet") : App.tr("%1 photos, updated %2", card.provider.items, SourceKinds.ago(card.provider.last_sync))
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }

        Row {
            spacing: Look.huecoBoton

            QQC2.Button {
                id: primary

                visible: card.broken || (!card.bare && !card.rotating)
                highlighted: true
                text: card.broken ? (card.provider.state === "needs_secret" ? App.tr("Change key") : App.tr("Try again")) : App.tr("Rotate these photos")
                icon.name: card.broken ? (card.provider.state === "needs_secret" ? "key" : "arrows-clockwise") : "shuffle"
                onClicked: card.broken ? card.fix() : card.rotate()
            }

            Text {
                visible: card.rotating && !card.broken
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("They rotate on every screen")
                font.family: Look.letraTexto
                font.pixelSize: Look.tCuerpo
                color: Look.brote
            }

            QQC2.Button {
                id: refreshing

                flat: true
                enabled: !card.syncing && card.provider.state !== "needs_secret"
                text: App.tr("Update")
                icon.name: "arrows-clockwise"
                onClicked: card.refresh()
            }

            QQC2.Button {
                flat: true
                text: App.tr("Details")
                icon.name: "caret-right"
                onClicked: card.details()
            }
        }
    }

    SourceThumbs {
        anchors.left: left.right
        anchors.leftMargin: Look.fuenteHueco
        anchors.verticalCenter: left.verticalCenter
        height: Math.min(left.height, (card.width - left.width - Look.fuenteHueco - (Look.fuenteFotos - 1) * Look.e2) / Look.fuenteFotos * 9 / 16)
        source: card.provider.source
        stale: card.broken
        visible: !card.bare
    }
}
