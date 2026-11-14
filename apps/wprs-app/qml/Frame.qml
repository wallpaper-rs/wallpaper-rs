import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: shell

    required property var host
    readonly property string kind: host.kind
    readonly property var page: host.pageStack.currentItem
    readonly property var active: kind === "work" ? work : kind === "options" ? options : title
    readonly property alias menu: menu
    readonly property alias notices: notices
    readonly property alias hints: hints
    readonly property alias wallpaper: wallpaper
    readonly property alias titleFrame: title
    readonly property alias optionsFrame: options
    readonly property alias workFrame: work
    readonly property bool routed: (page?.route ?? []).length > 0
    readonly property var keyHints: kind === "work" ? [] : menu.activeFocus ? [{
            "keys": ["up", "down"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Intro"],
            "text": App.tr("Open")
        }] : page?.keyHints ?? [{
            "keys": ["Tab"],
            "text": App.tr("Next part")
        }, {
            "keys": ["Esc"],
            "text": routed ? App.tr("Back") : App.tr("Menu")
        }]

    function stepBack() {
        if (routed && page.routeBack)
            return page.routeBack();
        menu.forceActiveFocus(Qt.ShortcutFocusReason);
    }

    anchors.fill: parent

    Wallpaper {
        id: wallpaper

        host: shell.host
    }

    Rectangle {
        anchors.fill: parent
        color: Look.noche
    }

    Backdrop {
        width: shell.kind === "options" ? Look.menuAncho : shell.width
        height: shell.height
        visible: shell.kind !== "work"
        source: wallpaper.picture
        held: wallpaper.held
        framing: shell.kind === "options" ? 0.06 : 0.7
    }

    Column {
        id: side

        x: Look.margen
        y: Look.margenArriba
        z: 2
        visible: shell.kind !== "work"
        spacing: Look.compacto ? Look.e5 : Look.e6

        Logo {
        }

        WordMenu {
            id: menu

            current: shell.host.current
            onChosen: page => shell.host.showPage(page)
        }
    }

    TitleFrame {
        id: title

        anchors.fill: parent
        visible: shell.kind === "title"
        page: visible ? shell.page : null
        sideBottom: side.y + side.height
        none: wallpaper.none
        onEscaped: shell.stepBack()
    }

    OptionsFrame {
        id: options

        anchors.fill: parent
        visible: shell.kind === "options"
        page: visible ? shell.page : null
        current: shell.host.current
        onEscaped: shell.stepBack()
        onChosen: page => shell.host.showPage(page)
    }

    WorkFrame {
        id: work

        anchors.fill: parent
        visible: shell.kind === "work"
        page: visible ? shell.page : null
        backWord: shell.host.backWord
        onLeave: shell.host.leave()
    }

    Hints {
        id: hints

        z: 3
        x: shell.kind === "title" ? (shell.active?.hintsAt?.x ?? 0) - width : (shell.active?.hintsAt?.x ?? 0)
        y: (shell.active?.hintsAt?.y ?? 0) - height
        visible: shell.kind !== "work" && model.length > 0
        model: shell.keyHints
    }

    Notices {
        id: notices

        z: 4
        host: shell.host
        x: (shell.active?.noticeAt.x ?? 0)
        y: shell.kind === "work" ? (shell.active?.noticeAt.y ?? 0) : (shell.active?.noticeAt.y ?? 0) - height
        width: Math.min(Look.avisoAncho, shell.width - x - Look.e4)
    }

    Binding {
        target: shell.host.pageStack
        property: "parent"
        value: shell.active?.seat ?? shell
    }
}
