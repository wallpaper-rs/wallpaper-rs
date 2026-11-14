import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: state

    required property var page
    required property var bar
    readonly property var library: page.library
    readonly property bool empty: library.total === 0 && !library.loading
    readonly property bool failed: empty && library.error !== ""
    readonly property bool unmatched: empty && !failed && bar.filtered

    visible: empty

    Vacio {
        visible: !state.failed && state.unmatched
        anchors.centerIn: parent
        title: state.bar.text !== "" ? App.tr("Nothing matches “%1”", state.bar.text) : App.tr("Nothing matches these filters")
        text: App.tr("Try other words, or look in every kind of wallpaper.")
        action: App.tr("Clear the filters")
        actionIcon: "x"
        onActed: state.bar.clear()
    }

    Vacio {
        visible: !state.failed && !state.unmatched
        anchors.centerIn: parent
        title: App.tr("Nothing here yet")
        text: App.tr("Add folders in the settings, import files or drop them here.")
        action: App.tr("Add wallpapers…")
        actionIcon: "plus"
        onActed: state.page.tools.dialogs.importer.open()
    }

    Fallo {
        visible: state.failed
        anchors.centerIn: parent
        title: App.tr("The library could not be read")
        text: App.tr("The service answered with an error. Try again in a moment.")
        details: state.library.error
        action: App.tr("Try again")
        onActed: state.page.search()
    }
}
