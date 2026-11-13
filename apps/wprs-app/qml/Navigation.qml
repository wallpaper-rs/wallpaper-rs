import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.GlobalDrawer {
    id: drawer

    required property var host

    function holds(item) {
        for (let at = item; at; at = at.parent) {
            if (at === drawer.contentItem)
                return true;
        }
        return false;
    }

    function leave() {
        if (modal)
            close();
        else if (collapsible)
            collapsed = true;
        host.pageStack.currentItem?.forceActiveFocus();
    }

    collapsible: true
    collapseButtonVisible: false
    focus: modal
    footer: QQC2.ToolButton {
        visible: drawer.collapsible
        icon.name: drawer.collapsed ? "sidebar-expand-left" : "sidebar-collapse-left"
        text: drawer.collapsed ? "" : App.tr("Close sidebar")
        onClicked: drawer.collapsed = !drawer.collapsed

        QQC2.ToolTip.visible: drawer.collapsed && hovered
        QQC2.ToolTip.text: App.tr("Open sidebar")
        QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
    }

    Shortcut {
        sequence: "Escape"
        enabled: !!drawer.host.activeFocusItem && drawer.holds(drawer.host.activeFocusItem)
        onActivated: drawer.leave()
    }

    onOpened: modal && contentItem.forceActiveFocus(Qt.PopupFocusReason)

    actions: [
        Kirigami.Action {
            text: App.tr("Library")
            icon.name: "folder-pictures"
            checked: host.current === 0
            onTriggered: host.showPage(0)
        },
        Kirigami.Action {
            text: App.tr("Scenes")
            icon.name: "draw-freehand"
            checked: host.current === 7
            onTriggered: host.showPage(7)
        },
        Kirigami.Action {
            text: App.tr("Playlists")
            icon.name: "view-media-playlist"
            checked: host.current === 1
            onTriggered: host.showPage(1)
        },
        Kirigami.Action {
            text: App.tr("Profiles and presets")
            icon.name: "video-display"
            checked: host.current === 2
            onTriggered: host.showPage(2)
        },
        Kirigami.Action {
            text: App.tr("Providers")
            icon.name: "internet-services"
            checked: host.current === 3
            onTriggered: host.showPage(3)
        },
        Kirigami.Action {
            text: App.tr("Repositories")
            icon.name: "package-x-generic"
            checked: host.current === 4
            onTriggered: host.showPage(4)
        },
        Kirigami.Action {
            text: App.tr("Settings")
            icon.name: "configure"
            checked: host.current === 5
            onTriggered: host.showPage(5)
        },
        Kirigami.Action {
            text: App.tr("Account")
            icon.name: "folder-cloud"
            checked: host.current === 6
            onTriggered: host.showPage(6)
        }
    ]
}
