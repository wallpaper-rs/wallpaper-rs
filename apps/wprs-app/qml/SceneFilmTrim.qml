import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: film

    required property var page
    required property var node
    property var info: null
    readonly property real total: Math.max(info?.length ?? 0, info?.end ?? 0, 0.1)
    readonly property var frames: JSON.parse(strips.frames || "{}")[info?.file ?? ""] ?? []

    function load() {
        const shown = node && (node.type === "video" || node.type === "image");
        info = shown ? JSON.parse(page.editor.rig(node.id, JSON.stringify({
                        "film": {
                            "layer": node.id
                        }
                    })) || "null") : null;
        if (info?.file)
            strips.ask(info.file, 12);
    }

    function trim(start, end, gesture) {
        const span = Math.max(0.05, total);
        const first = Math.round(Math.max(0, Math.min(span - 0.05, start)) * 100) / 100;
        const last = end === null ? null : Math.round(Math.max(first + 0.05, Math.min(span, end)) * 100) / 100;
        page.run("film", {
                "trim": {
                    "layer": node.id,
                    "start": first,
                    "end": last
                }
            }, gesture === true);
    }

    function at(x) {
        return Math.max(0, Math.min(1, x / bar.width)) * total;
    }

    visible: info !== null
    Component.onCompleted: load()
    onNodeChanged: load()

    Strips {
        id: strips
    }

    Connections {
        function onViewChanged() {
            film.load();
        }

        target: film.page.editor
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Video")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: film.info ? [film.info.size ? film.info.size[0] + " × " + film.info.size[1] + " px" : "", film.info.length ? App.tr("%1 s long", film.info.length.toFixed(2)) : App.tr("length unknown")].filter(w => w).join(" · ") : ""
    }

    Item {
        id: bar

        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 3

        Rectangle {
            anchors.fill: parent
            color: Kirigami.Theme.alternateBackgroundColor
        }

        Repeater {
            model: film.frames

            delegate: Image {
                required property var modelData
                required property int index

                x: modelData.at / film.total * bar.width
                width: bar.width / Math.max(1, film.frames.length) * Math.min(1, (film.info?.length ?? film.total) / film.total)
                height: bar.height
                source: App.url(modelData.file)
                fillMode: Image.PreserveAspectCrop
                asynchronous: true
            }
        }

        Rectangle {
            x: 0
            width: grips.startX
            height: bar.height
            color: Kirigami.Theme.backgroundColor
            opacity: 0.7
        }

        Rectangle {
            x: grips.endX
            width: bar.width - grips.endX
            height: bar.height
            color: Kirigami.Theme.backgroundColor
            opacity: 0.7
        }

        Repeater {
            model: Math.floor(film.total) + 1

            delegate: Rectangle {
                required property int index

                x: index / film.total * bar.width
                y: bar.height - height
                width: 1
                height: Kirigami.Units.smallSpacing * (index % 5 === 0 ? 3 : 1.5)
                visible: x < bar.width
                color: Kirigami.Theme.textColor
                opacity: 0.6
            }
        }

        QtObject {
            id: grips

            readonly property real startX: (film.info?.start ?? 0) / film.total * bar.width
            readonly property real endX: (film.info?.end ?? film.total) / film.total * bar.width
        }

        Repeater {
            model: ["start", "end"]

            delegate: Rectangle {
                id: grip

                required property string modelData
                readonly property real seconds: (modelData === "start" ? film.info?.start : film.info?.end) ?? film.total

                x: seconds / film.total * bar.width - width / 2
                width: Kirigami.Units.gridUnit * 0.7
                height: bar.height
                radius: 2
                color: Kirigami.Theme.highlightColor
                border.color: Kirigami.Theme.backgroundColor
                QQC2.ToolTip.visible: over.hovered || move.active
                QQC2.ToolTip.text: (modelData === "start" ? App.tr("Start %1 s", held) : App.tr("End %1 s", held))

                readonly property string held: film.at(grip.x + grip.width / 2).toFixed(2)

                HoverHandler {
                    id: over
                }

                DragHandler {
                    id: move

                    target: grip
                    yAxis.enabled: false
                    xAxis.minimum: -grip.width / 2
                    xAxis.maximum: bar.width - grip.width / 2
                    onActiveChanged: {
                        if (active)
                            return;
                        const seconds = film.at(grip.x + grip.width / 2);
                        if (grip.modelData === "start")
                            film.trim(seconds, film.info.end ?? null);
                        else
                            film.trim(film.info.start ?? 0, seconds);
                    }
                }
            }
        }
    }

    SceneFilmRate {
        Layout.fillWidth: true
        film: film
    }
}
