import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: band

    required property var page
    readonly property alias scene: scene
    readonly property alias single: single
    readonly property alias extra: extra
    readonly property alias learn: learn
    readonly property real oneLine: bar.height + spacing + (page.learn ? examples.implicitHeight : Math.max(sceneGroup.implicitHeight, singleGroup.implicitHeight, extraGroup.visible ? extraGroup.implicitHeight : 0))

    spacing: Look.e4

    RowLayout {
        id: bar

        width: parent.width
        spacing: Look.e5

        Filtro {
            label: App.tr("How to start")
            current: band.page.learn ? 1 : 0
            words: [{
                    "text": App.tr("Start a new one")
                }, {
                    "text": App.tr("Learn from an example"),
                    "count": band.page.scenes.length
                }]
            onPicked: index => {
                band.page.learn = index === 1;
                band.page.focused = index === 1 ? learn : scene;
            }
        }

        Item {
            Layout.fillWidth: true
        }

        QQC2.Button {
            visible: !band.page.learn
            Layout.fillWidth: true
            Layout.maximumWidth: implicitWidth
            flat: true
            text: App.tr("More: preset, resource pack")
            icon.name: "caret-right"
            LayoutMirroring.enabled: true
            onClicked: band.page.more = !band.page.more
        }

        QQC2.Button {
            text: App.tr("Open a folder…")
            icon.name: "folder-open"
            onClicked: band.page.pickFolder()
        }
    }

    Flow {
        visible: !band.page.learn
        width: parent.width
        spacing: Look.e7

        Column {
            id: sceneGroup

            spacing: Look.e2

            CreateHead {
                text: App.tr("A scene")
                small: App.tr("layers, effects, animation")
            }

            CreateRow {
                id: scene

                width: Math.min(band.width, contentWidth)
                height: implicitHeight
                label: App.tr("A scene")
                model: band.page.facts.sceneModel
                active: band.page.focused === scene
                onTaken: band.page.focused = scene
                onActivated: band.page.act(scene)
            }
        }

        Column {
            id: singleGroup

            spacing: Look.e2

            CreateHead {
                text: App.tr("A single-file wallpaper")
                small: App.tr("picture, video or page")
            }

            CreateRow {
                id: single

                width: Math.min(band.width, contentWidth)
                height: implicitHeight
                label: App.tr("A single-file wallpaper")
                model: band.page.facts.singleModel
                active: band.page.focused === single
                onTaken: band.page.focused = single
                onActivated: band.page.act(single)
            }
        }

        Column {
            id: extraGroup

            visible: band.page.more
            spacing: Look.e2

            CreateHead {
                text: App.tr("More")
            }

            CreateRow {
                id: extra

                width: Math.min(band.width, contentWidth)
                height: implicitHeight
                label: App.tr("More")
                model: band.page.facts.moreModel
                active: band.page.focused === extra
                onTaken: band.page.focused = extra
                onActivated: band.page.act(extra)
            }
        }
    }

    Column {
        id: examples

        visible: band.page.learn
        width: parent.width
        spacing: Look.e2

        CreateHead {
            text: App.tr("Example scenes")
            small: App.tr("these first; then the others you have")
        }

        CreateRow {
            id: learn

            width: parent.width
            height: implicitHeight
            label: App.tr("Example scenes")
            model: band.page.facts.learnModel
            active: band.page.focused === learn
            onTaken: band.page.focused = learn
            onActivated: band.page.act(learn)
        }
    }
}
