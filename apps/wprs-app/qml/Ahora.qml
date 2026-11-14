import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: ahora

    property var page
    property string picked
    property bool none: false
    property bool compact: false
    readonly property var screens: App.state.outputs ?? {}
    readonly property bool several: App.outputs.length > 1
    readonly property bool strip: several && !none && !compact
    readonly property string screen: {
        const wanted = page && page.target !== undefined ? page.target : picked;
        return App.outputs.indexOf(wanted) >= 0 ? wanted : App.outputs.length > 0 ? App.outputs[0] : "";
    }
    readonly property var info: screens[screen] ?? {}
    readonly property string item: info.item ?? ""
    readonly property var playlist: info.playlist ?? null
    readonly property string title: none ? App.tr("You have no wallpapers yet") : details.title ?? ""
    property var details: ({})
    property bool fresh: false
    readonly property alias controls: controls
    readonly property alias rotating: rotating
    readonly property alias tasks: tasks
    readonly property string kindWord: ({
            "scene": App.tr("Scene"),
            "image": App.tr("Image"),
            "video": App.tr("Video"),
            "web": App.tr("Web page")
        })[info.kind ?? ""] ?? ""

    function choose(name) {
        if (page && page.target !== undefined)
            page.target = name;
        else
            picked = name;
    }

    function load() {
        const uid = item;
        if (uid === "")
            details = {};
        else
            App.query("GetItem", [uid], "Library1", reply => ahora && ahora.item === uid && (ahora.details = reply.item ?? {}));
    }

    spacing: 0
    Accessible.role: Accessible.Grouping
    Accessible.name: App.tr("On your desktop")
    onItemChanged: {
        fresh = ahora.Window.window !== null && ahora.Window.window.visible && details.uid !== undefined;
        load();
        if (fresh)
            freshness.restart();
    }
    Component.onCompleted: load()

    Timer {
        id: freshness

        interval: 60000
        onTriggered: ahora.fresh = false
    }

    Connections {
        target: App

        function onUpdated(topic, ids) {
            if (topic === "library" && (ids.length === 0 || ids.includes(ahora.item)))
                ahora.load();
        }
    }

    Text {
        anchors.right: parent.right
        visible: (!ahora.several || ahora.none) && !ahora.compact
        text: App.tr("On your desktop")
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        font.weight: Font.Medium
        color: Look.bruma
    }

    Pantallas {
        anchors.right: parent.right
        visible: ahora.strip
        current: ahora.screen
        onPicked: name => ahora.choose(name)
    }

    Text {
        anchors.right: parent.right
        topPadding: ahora.strip ? 0 : Look.e2
        visible: text !== ""
        text: ahora.title
        width: Math.min(implicitWidth, Look.tapaFoco * 2)
        elide: Text.ElideRight
        horizontalAlignment: Text.AlignRight
        font.family: Look.letraTitulo
        font.pixelSize: Look.tPortada
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    Text {
        anchors.right: parent.right
        topPadding: ahora.strip ? Look.e1 : Look.e3
        visible: text !== "" && !ahora.none && !ahora.compact
        text: [ahora.kindWord, ahora.info.credit ? ahora.info.credit.provider : ""].filter(t => t).join(" · ")
        font.family: Look.letraTexto
        font.pixelSize: Look.tNombre
        font.weight: Font.Medium
        color: Look.escarcha
    }

    Estado {
        anchors.right: parent.right
        topPadding: Look.e3
        visible: !ahora.none && ahora.info.quality_auto === "still"
        tone: "ambar"
        text: App.tr("Still at its poster")
    }

    Text {
        anchors.right: parent.right
        topPadding: Look.e3
        visible: ahora.none
        width: Math.min(implicitWidth, Look.tapaFoco * 2)
        wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignRight
        text: App.tr("Choose where to start: the first thing that enters your library becomes your wallpaper.")
        font.family: Look.letraTexto
        font.pixelSize: Look.tNombre
        color: Look.bruma
    }

    Item {
        width: 1
        height: Look.e1
    }

    Estado {
        anchors.right: parent.right
        visible: ahora.fresh && !ahora.none
        tone: "brote"
        text: App.tr("Set a moment ago")
    }

    Item {
        width: 1
        height: Look.e5
        visible: !ahora.none
    }

    AhoraMandos {
        id: controls

        anchors.right: parent.right
        visible: !ahora.none && !(ahora.compact && rotating.visible)
        screen: ahora.screen
        info: ahora.info
        item: ahora.item
        favorite: ahora.details.favorite === true
    }

    AhoraLista {
        id: rotating

        anchors.right: parent.right
        topPadding: Look.e3
        screen: ahora.screen
        playlist: ahora.none ? null : ahora.playlist
    }

    AhoraTarea {
        id: tasks

        anchors.right: parent.right
        topPadding: Look.e3
        height: ahora.compact ? 0 : implicitHeight
        clip: ahora.compact
    }
}
