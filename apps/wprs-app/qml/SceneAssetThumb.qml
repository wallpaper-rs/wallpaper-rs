import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: thumb

    required property string kind
    required property string file
    required property string still
    property real size: Kirigami.Units.gridUnit * 2
    readonly property var icons: ({
            "image": "image-x-generic",
            "video": "video-x-generic",
            "sound": "audio-x-generic",
            "font": "font-x-generic",
            "lut": "color-management",
            "model": "draw-cuboid"
        })
    readonly property bool plain: ["png", "jpg", "jpeg", "webp", "gif"].includes(file.split(".").pop().toLowerCase())
    readonly property bool drawn: (kind === "image" && plain && picture.status === Image.Ready) || (kind === "video" && frame.status === Image.Ready) || (kind === "font" && face.status === FontLoader.Ready)

    width: size
    height: size
    radius: 3
    color: Kirigami.Theme.alternateBackgroundColor
    clip: true

    Image {
        id: picture

        anchors.fill: parent
        visible: thumb.kind === "image"
        source: thumb.kind === "image" && thumb.plain ? App.url(thumb.file) : ""
        sourceSize.width: thumb.size * 2
        sourceSize.height: thumb.size * 2
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }

    Image {
        id: frame

        anchors.fill: parent
        visible: thumb.kind === "video"
        source: thumb.still !== "" ? App.url(thumb.still) : ""
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }

    FontLoader {
        id: face

        source: thumb.kind === "font" ? App.url(thumb.file) : ""
    }

    QQC2.Label {
        anchors.centerIn: parent
        visible: thumb.kind === "font" && face.status === FontLoader.Ready
        text: "Aa"
        font.family: face.name
        font.pixelSize: thumb.size * 0.5
    }

    Kirigami.Icon {
        anchors.centerIn: parent
        width: thumb.size * 0.6
        height: width
        visible: !thumb.drawn
        source: thumb.icons[thumb.kind] ?? "text-x-generic"
    }
}
