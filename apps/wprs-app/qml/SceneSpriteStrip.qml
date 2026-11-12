import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: strip

    required property var sheet
    property int selected: -1
    property int shown: 0
    property bool playing: true
    readonly property var rects: sheet?.rects ?? []
    readonly property int count: Math.max(1, Math.min(rects.length, sheet?.frames ?? rects.length))
    readonly property real fps: sheet?.fps ?? 8

    signal pick(int index)
    signal reorder(int from, int to)
    signal remove(int index)
    signal play(var fps, var frames)

    function crop(rect) {
        return Qt.rect(rect[0], rect[1], rect[2], rect[3]);
    }

    Timer {
        interval: Math.max(8, 1000 / strip.fps)
        repeat: true
        running: strip.playing && strip.rects.length > 1 && strip.visible
        onTriggered: strip.shown = (strip.shown + 1) % strip.count
    }

    RowLayout {
        Layout.fillWidth: true

        Rectangle {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 6
            Layout.preferredHeight: Kirigami.Units.gridUnit * 6
            color: Kirigami.Theme.alternateBackgroundColor

            Image {
                anchors.fill: parent
                anchors.margins: 2
                source: strip.rects.length > 0 ? App.url(strip.sheet.file) : ""
                sourceClipRect: strip.rects.length > 0 ? strip.crop(strip.rects[Math.min(strip.shown, strip.rects.length - 1)]) : Qt.rect(0, 0, 0, 0)
                fillMode: Image.PreserveAspectFit
                smooth: false
            }
        }

        ColumnLayout {
            QQC2.Button {
                icon.name: strip.playing ? "media-playback-pause" : "media-playback-start"
                text: strip.playing ? App.tr("Pause") : App.tr("Play")
                onClicked: strip.playing = !strip.playing
            }

            RowLayout {
                QQC2.Label {
                    text: App.tr("Frames a second")
                }

                QQC2.SpinBox {
                    from: 1
                    to: 1200
                    stepSize: 10
                    editable: true
                    value: Math.round(strip.fps * 10)
                    textFromValue: v => (v / 10).toFixed(1)
                    valueFromText: text => Math.round(Number(text.replace(",", ".")) * 10)
                    onValueModified: strip.play(value / 10, null)
                }
            }

            RowLayout {
                QQC2.Label {
                    text: App.tr("Frames played")
                }

                QQC2.SpinBox {
                    from: 1
                    to: Math.max(1, strip.rects.length)
                    editable: true
                    value: strip.count
                    onValueModified: strip.play(null, value)
                }
            }
        }
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.preferredHeight: Kirigami.Units.gridUnit * 5

        ListView {
            orientation: ListView.Horizontal
            spacing: Kirigami.Units.smallSpacing
            model: strip.rects
            clip: true

            delegate: Rectangle {
                required property var modelData
                required property int index

                width: Kirigami.Units.gridUnit * 3.5
                height: ListView.view.height - Kirigami.Units.gridUnit
                color: Kirigami.Theme.alternateBackgroundColor
                opacity: index < strip.count ? 1 : 0.4
                border.color: strip.selected === index ? Kirigami.Theme.highlightColor : (strip.shown === index ? Kirigami.Theme.positiveTextColor : "transparent")
                border.width: 2

                Image {
                    anchors.fill: parent
                    anchors.margins: 3
                    source: App.url(strip.sheet.file)
                    sourceClipRect: strip.crop(modelData)
                    fillMode: Image.PreserveAspectFit
                    smooth: false
                }

                QQC2.Label {
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    text: parent.index + 1
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: strip.pick(parent.index)
                }
            }
        }
    }

    RowLayout {
        QQC2.Button {
            icon.name: "go-previous"
            text: App.tr("Earlier")
            enabled: strip.selected > 0
            onClicked: strip.reorder(strip.selected, strip.selected - 1)
        }

        QQC2.Button {
            icon.name: "go-next"
            text: App.tr("Later")
            enabled: strip.selected >= 0 && strip.selected < strip.rects.length - 1
            onClicked: strip.reorder(strip.selected, strip.selected + 1)
        }

        QQC2.Button {
            icon.name: "edit-delete"
            text: App.tr("Remove the frame")
            enabled: strip.selected >= 0 && strip.selected < strip.rects.length
            onClicked: strip.remove(strip.selected)
        }
    }
}
