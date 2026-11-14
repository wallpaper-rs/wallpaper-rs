import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: top

    required property var page
    readonly property var history: page.view.history ?? ({})
    readonly property bool dirty: page.editor.dirty
    readonly property bool filled: (page.view.layers ?? []).length > 0

    spacing: Look.e3

    Estado {
        text: top.dirty ? App.tr("Unsaved ·") : App.tr("Saved")
        tone: top.dirty ? "" : "brote"
        icon: top.dirty ? "circle-half" : "check-circle"
    }

    QQC2.Button {
        visible: top.dirty
        flat: true
        text: App.tr("Save now")
        enabled: !top.page.editor.busy
        onClicked: top.page.editor.save()
    }

    Rectangle {
        Layout.leftMargin: Look.e1
        Layout.rightMargin: Look.e1
        implicitWidth: Look.linea
        implicitHeight: Look.entreAlto
        color: Look.nocheLinea
    }

    QQC2.Button {
        flat: true
        icon.name: "arrow-counter-clockwise"
        text: App.tr("Undo")
        enabled: (top.history.done ?? []).length > 0
        QQC2.ToolTip.text: top.history.undo ? App.tr("Undo: %1", top.history.undo) : ""
        QQC2.ToolTip.visible: hovered && QQC2.ToolTip.text !== ""
        onClicked: top.page.run("undo")
    }

    QQC2.Button {
        flat: true
        icon.name: "arrow-clockwise"
        text: App.tr("Redo")
        enabled: (top.history.undone ?? []).length > 0
        QQC2.ToolTip.text: top.history.redo ? App.tr("Redo: %1", top.history.redo) : ""
        QQC2.ToolTip.visible: hovered && QQC2.ToolTip.text !== ""
        onClicked: top.page.run("redo")
    }

    Item {
        Layout.fillWidth: true
    }

    QQC2.Button {
        id: view

        implicitHeight: Look.altoBotonArriba
        icon.name: "monitor-play"
        text: App.tr("View as wallpaper")
        visible: top.page.viewable !== false
        enabled: top.page.editor.ready
        onClicked: top.page.use.view(view)
    }

    QQC2.Button {
        id: place

        implicitHeight: Look.altoBotonArriba
        icon.name: "image"
        text: App.tr("Set as wallpaper")
        enabled: top.page.editor.ready
        onClicked: top.page.use.place(place)
    }

    QQC2.Button {
        implicitHeight: Look.altoBotonArriba
        highlighted: top.filled
        palette.highlight: Look.luna
        icon.name: "paper-plane-tilt"
        text: App.tr("Publish…")
        enabled: top.page.editor.ready
        onClicked: top.page.use.publish()
    }

    Shortcut {
        sequence: "Ctrl+Z"
        enabled: (top.history.done ?? []).length > 0
        onActivated: top.page.run("undo")
    }

    Shortcut {
        sequence: "Ctrl+Shift+Z"
        enabled: (top.history.undone ?? []).length > 0
        onActivated: top.page.run("redo")
    }

    Shortcut {
        sequence: "Ctrl+S"
        enabled: top.dirty && !top.page.editor.busy
        onActivated: top.page.editor.save()
    }
}
