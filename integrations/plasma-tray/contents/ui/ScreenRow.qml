import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: row

    required property var daemon
    required property int index
    property bool single: false
    signal more(string name)
    required property string name
    required property var info
    readonly property string item: info.item ?? ""
    readonly property var playing: info.playlist ?? null
    readonly property string title: details.title || info.credit?.title || String(info.image ?? "").split("/").pop() || (item ? "" : i18n("Background color"))
    property string thumb
    property LeftOut reasons: LeftOut {}
    property var details: ({})
    property string animated
    readonly property var pkg: details.package ?? {}
    readonly property bool allowed: pkg.flash_disabled !== true

    function load() {
        const item = row.item;
        if (!item)
            return;
        daemon.query("GetItem", [item], "Library1", reply => {
            if (row.item !== item)
                return;
            row.details = reply.item ?? {};
            row.animated = reply.about?.animated ?? "";
            daemon.query("Thumbnail", [item, 256], "Library1", answer => {
                if (row.item === item)
                    row.thumb = answer[0] ?? "";
            });
        });
    }

    readonly property string meta: ({"scene": i18n("Scene"), "video": i18n("Video"), "image": i18n("Image"), "web": i18n("Web page"), "color": i18n("Color")})[info.kind] ?? ""
    readonly property string where: playing ? [name, i18n("“%1”, %2 of %3", playing.name, playing.position + 1, playing.count)].join(" · ") : [name, i18n("no playlist running")].join(" · ")

    spacing: Look.e2
    onItemChanged: {
        thumb = "";
        animated = "";
        details = {};
        Qt.callLater(load);
    }
    Component.onCompleted: Qt.callLater(load)

    Connections {
        target: row.daemon

        function onUpdated(topic, ids) {
            if (topic !== "library")
                return;
            if (ids.includes(row.item))
                row.load();
            else if (ids.length === 0)
                settle.restart();
        }
    }

    Timer {
        id: settle

        interval: 1000
        onTriggered: row.load()
    }

    Item {
        Layout.fillWidth: true
        Layout.preferredHeight: width * 7 / 16
        visible: row.single

        Thumbnail {
            anchors.fill: parent
            radius: 0
            daemon: row.daemon
            item: row.item
            source: row.daemon.url(row.thumb)
            animated: row.pkg.photosensitive === true && !row.allowed ? "" : row.daemon.url(row.animated)
            moving: !(row.daemon.state?.on_battery ?? false)
            scene: false
            color: row.info.background || Look.nocheAlta
        }

        Rectangle {
            anchors.fill: parent
            gradient: Gradient {
                GradientStop {
                    position: 0.4
                    color: "transparent"
                }
                GradientStop {
                    position: 1
                    color: Look.noche
                }
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: row.single ? Look.e4 : 0
        Layout.rightMargin: row.single ? Look.e4 : 0
        spacing: Look.e3

        Thumbnail {
            Layout.preferredWidth: 72
            Layout.preferredHeight: 40
            Layout.alignment: Qt.AlignTop
            visible: !row.single
            color: row.info.background || Look.nocheAlta
            daemon: row.daemon
            item: row.item
            source: row.daemon.url(row.thumb)
            animated: row.pkg.photosensitive === true && !row.allowed ? "" : row.daemon.url(row.animated)
            moving: !(row.daemon.state?.on_battery ?? false)
            scene: row.info.kind === "scene"
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 0

            Words {
                Layout.fillWidth: true
                text: row.title
                title: row.single
                font.pixelSize: row.single ? Look.tFila : Look.tCuerpo
                font.weight: row.single ? Font.Bold : Font.DemiBold
            }

            Words {
                Layout.fillWidth: true
                text: row.single ? row.meta : row.where
                soft: true
                font.pixelSize: Look.tPie
            }
        }

        Status {
            visible: row.single
            paused: row.daemon.current.paused === true
            Layout.alignment: Qt.AlignTop
        }
    }

    Notes {
        Layout.fillWidth: true
        Layout.leftMargin: row.single ? Look.e4 : Look.e1
        Layout.rightMargin: row.single ? Look.e4 : 0
        host: row
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.topMargin: Look.e1
        Layout.leftMargin: row.single ? Look.e4 : 0
        spacing: 0

        Command {
            enabled: row.playing !== null
            quiet: true
            glyph: "skip-forward"
            text: i18n("Next")
            onClicked: row.daemon.act("Next", [row.name], "Playlists1")
        }

        Command {
            quiet: true
            text: i18n("Choose…")
            onClicked: row.daemon.choose(row.name)
        }

        Item {
            Layout.fillWidth: true
        }

        Command {
            visible: !row.single
            quiet: true
            glyph: "dots-three"
            text: ""
            Accessible.name: i18n("Favorite, Fit, Filter and remove")
            onClicked: row.more(row.name)
        }
    }
}
