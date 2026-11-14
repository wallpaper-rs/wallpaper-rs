import QtQuick
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

Item {
    id: state

    required property var page
    required property var bar
    readonly property var library: page.library
    readonly property bool empty: library.total === 0 && !library.loading
    readonly property bool failed: empty && library.error !== ""
    readonly property bool unmatched: empty && !failed && bar.filtered

    readonly property bool starting: empty && !failed && !unmatched

    readonly property bool alone: starting || failed
    readonly property alias fault: fault
    readonly property alias start: start
    readonly property real low: starting ? start.low : failed ? fault.implicitHeight : 0
    readonly property real high: starting ? start.high : 0
    property bool restarting: false
    property string refused

    function restart() {
        restarting = true;
        refused = "";
        App.act("Restart", [], "Service", () => state && (state.restarting = false, state.page.search()), error => state && (state.restarting = false, state.refused = error.message));
    }

    visible: empty

    Vacio {
        visible: !state.failed && state.unmatched
        title: state.bar.text !== "" ? App.tr("Nothing matches “%1”", state.bar.text) : App.tr("Nothing matches these filters")
        text: App.tr("Try other words, or look in every kind of wallpaper.")
        action: App.tr("Clear the filters")
        actionIcon: "x"
        onActed: state.bar.clear()
    }

    FirstStart {
        id: start

        visible: state.starting
        focus: state.starting
        width: state.width
        height: state.height
        run: first
        page: state.page
        adder: folders
    }

    FirstRun {
        id: first

        bare: state.starting
    }

    FolderDialog {
        id: folders

        onAccepted: App.act("Import", [[App.path(selectedFolder)], false], "Library1")
    }

    Timer {
        interval: 3000
        repeat: true
        running: state.failed && state.visible
        onTriggered: state.page.search()
    }

    Fallo {
        id: fault

        readonly property bool banded: (state.page.band?.width ?? 0) > 0

        y: banded ? Math.max(0, state.height - height) : 0
        visible: state.failed
        focus: state.failed
        principal: true
        actionIcon: "arrow-clockwise"
        Keys.onReturnPressed: state.restarting || state.restart()
        Keys.onEnterPressed: state.restarting || state.restart()
        title: App.tr("The wallpaper service does not answer")
        text: state.refused !== "" ? App.tr("The service could not be started from here. The details say why; this page comes back by itself when the service answers.") : App.tr("Your library and the wallpapers of your desktop depend on it. Restarting it is usually enough; what you had set is not lost.")
        big: true
        details: [state.refused, state.library.error].filter(line => line !== "").join("\n")
        action: state.restarting ? App.tr("Restarting the service…") : App.tr("Restart the service")
        onActed: state.restarting || state.restart()
    }
}
