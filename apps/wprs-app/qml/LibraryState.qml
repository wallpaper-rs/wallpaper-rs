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
        visible: state.starting
        run: first
        tools: state.page.tools
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
        visible: state.failed
        title: App.tr("The wallpaper service does not answer")
        text: App.tr("Your library and the wallpapers of your desktop depend on it. What you had set is not lost, and this page comes back by itself when the service answers.")
        big: true
        details: state.library.error
        action: App.tr("Try again")
        onActed: state.page.search()
    }
}
