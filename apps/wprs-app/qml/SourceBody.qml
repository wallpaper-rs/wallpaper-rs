import QtQuick
import io.github.wallpaper_rs.app

Flickable {
    id: scroll

    required property var host
    required property var dialogs
    readonly property Item shown: body.item

    function reveal(item) {
        if (!item || !body.item || !body.item.contains(item.mapToItem(body.item, 0, 0)))
            return;
        const y = item.mapToItem(contentItem, 0, 0).y;
        if (y < contentY)
            contentY = Math.max(0, y - Look.e4);
        else if (y + item.height > contentY + height)
            contentY = y + item.height - height + Look.e4;
    }

    visible: scroll.host.view === "add" || scroll.host.view === "detail" || scroll.host.view === "more"
    contentHeight: Math.max(height, body.implicitHeight)
    interactive: contentHeight > height
    clip: true

    Connections {
        target: scroll.host.Window.window

        function onActiveFocusItemChanged() {
            scroll.reveal(scroll.host.Window.window.activeFocusItem);
        }
    }

    Loader {
        id: body

        anchors.bottom: parent.bottom
        width: Math.min(scroll.width, Look.filasAncho)
        active: scroll.visible && (scroll.host.view === "add" || !!scroll.host.chosen.id)
        sourceComponent: scroll.host.view === "add" ? addForm : scroll.host.view === "more" ? moreForm : detailForm
    }

    Component {
        id: addForm

        SourceAdd {
            kind: scroll.host.kind
            taken: scroll.host.providers.map(p => p.id)
            onDone: scroll.host.open("list")
            onCancelled: scroll.host.routeBack()
            onKeyFiler: scroll.dialogs.filer.open()
        }
    }

    Component {
        id: detailForm

        SourceDetail {
            provider: scroll.host.chosen
            onClosed: scroll.host.open("list")
            onRemoved: scroll.host.open("list")
            onKeyAsked: scroll.dialogs.keys.ask(scroll.host.chosen.id)
            onMore: scroll.host.deeper("")
            onListed: App.act("Create", [SourceKinds.heading(scroll.host.chosen), "source:" + scroll.host.chosen.source], "Playlists1", () => App.navigate(1, ""))
        }
    }

    Component {
        id: moreForm

        SourceMore {
            provider: scroll.host.chosen
            level: scroll.host.level
            onDeeper: next => scroll.host.deeper(next)
            onClosed: scroll.host.routeBack()
        }
    }
}
