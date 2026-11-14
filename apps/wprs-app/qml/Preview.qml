import QtQuick
import io.github.wallpaper_rs.app

Window {
    id: preview

    readonly property var args: Qt.application.arguments
    property string output: args.indexOf("--browse") >= 0 ? (args[args.indexOf("--browse") + 1] ?? "") : ""
    property var session: ({})
    property string failure: ""
    property bool began: false
    property int left: -1
    property real sample: 1
    readonly property bool active: session.active === true
    readonly property var candidate: session.candidate ?? null
    readonly property string kind: !candidate ? "" : candidate.kind === "package" ? (candidate.package?.kind ?? "") : candidate.kind
    property alias words: words
    readonly property var current: session.now ?? null
    readonly property var plays: session.plays ?? ({})
    readonly property bool calm: plays.why === "calm" || plays.why === "still"
    readonly property bool alone: !candidate || session.count <= 1 && current !== null && candidate.uid === current.uid
    readonly property bool sampling: sample < 1 && !calm
    readonly property var screens: App.outputs
    readonly property string mode: failure !== "" ? "error" : began && (!active || alone) ? "vacio" : sampling ? "transicion" : calm ? "menos-movimiento" : "normal"
    readonly property var keyHints: mode === "vacio" ? [hint("Intro", App.tr("Open the library")), hint("Esc", App.tr("Close"))] : mode === "error" ? [hint("Intro", App.tr("Try again")), hint("Esc", App.tr("Close"))] : [hint("left", App.tr("Previous")), hint("right", App.tr("Next"))].concat(screens.length > 1 ? [hint("Tab", App.tr("Change screen"))] : []).concat([hint("Intro", App.tr("Apply")), hint("Esc", App.tr("Cancel"))])

    function hint(key, text) {
        return {
            "keys": [key],
            "text": text
        };
    }

    function ask(member, values) {
        App.query(member, values, "Playlists1", reply => preview && preview.take(reply), error => preview && preview.fail(error));
    }

    function load() {
        failure = "";
        ask("GetBrowse", [output]);
    }

    function take(state) {
        began = true;
        failure = "";
        if (state.ended)
            return finish(0);
        if (state.active)
            output = state.output;
        const was = candidate ? candidate.uid : "";
        session = state;
        if (candidate && candidate.uid !== was && !calm && !alone)
            sampler.restart();
    }

    function fail(error) {
        began = true;
        failure = error.message || App.tr("The wallpaper service does not answer.");
    }

    function heard(where, state) {
        if (where === output || state.active && !active)
            take(state);
    }

    function finish(code) {
        left = code;
        sampler.stop();
        if (!App.checking)
            Qt.exit(code);
    }

    function swap() {
        if (screens.length < 2 || !active)
            return;
        const was = output;
        output = screens[(screens.indexOf(was) + 1) % screens.length];
        ask("Browse", [output, session.source ?? "", 1]);
        App.query("BrowseCancel", [was], "Playlists1", () => {}, () => {});
    }

    function press(key) {
        const working = mode !== "vacio" && mode !== "error";
        if (key === Qt.Key_Escape)
            active ? App.query("BrowseCancel", [output], "Playlists1", reply => preview && preview.take(reply), () => preview && preview.finish(0)) : finish(0);
        else if (key === Qt.Key_Return || key === Qt.Key_Enter)
            mode === "vacio" ? finish(3) : mode === "error" ? load() : ask("BrowseApply", [output, ""]);
        else if (working && (key === Qt.Key_Left || key === Qt.Key_Right))
            ask("Browse", [output, session.source ?? "", key === Qt.Key_Left ? -1 : 1]);
        else if (working && key === Qt.Key_Tab)
            swap();
        else
            return false;
        return true;
    }

    visible: false
    color: Look.noche
    title: App.tr("Change wallpaper")
    Component.onCompleted: {
        const screens = Qt.application.screens;
        for (let i = 0; i < screens.length; i++)
            if (screens[i].name === output)
                screen = screens[i];
        showFullScreen();
        load();
    }

    LookWindow {
        host: preview
    }

    Timer {
        interval: PreviewLook.esperaServicio
        running: !preview.began
        onTriggered: preview.fail({})
    }

    NumberAnimation {
        id: sampler

        target: preview
        property: "sample"
        from: 0
        to: 1
        duration: PreviewLook.muestra
    }

    Connections {
        function onUpdated(topic, ids) {
            if (topic === "browse")
                preview.heard(ids[0], JSON.parse(ids[1]));
        }

        target: App
    }

    PreviewStage {
        anchors.fill: parent
        visible: preview.began
        preview: preview
    }

    Item {
        anchors.fill: parent
        visible: preview.began
        anchors.leftMargin: Look.margen
        anchors.rightMargin: Look.margen
        anchors.topMargin: Look.margenArriba
        anchors.bottomMargin: Look.margenAbajo
        focus: true
        Accessible.role: Accessible.Dialog
        Accessible.name: preview.title
        Keys.onPressed: event => event.accepted = preview.press(event.key)

        PreviewTop {
            anchors.right: parent.right
            preview: preview
        }

        PreviewText {
            id: words

            anchors.left: parent.left
            anchors.bottom: hints.top
            anchors.bottomMargin: Look.e8
            preview: preview
        }

        PreviewStrip {
            anchors.right: parent.right
            anchors.bottom: hints.top
            anchors.bottomMargin: Look.e8
            visible: preview.mode !== "vacio" && preview.mode !== "error"
            preview: preview
        }

        Hints {
            id: hints

            anchors.right: parent.right
            anchors.bottom: parent.bottom
            model: preview.keyHints
        }
    }
}
