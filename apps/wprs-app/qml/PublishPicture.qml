import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Item {
    id: picture

    required property var publish
    readonly property string preview: publish.editing && publish.manifest.preview ? "file://" + publish.page.editor.folder + "/" + publish.manifest.preview : ""
    readonly property string kind: publish.editing ? (publish.page.kind ?? "scene") : ""

    clip: true

    Image {
        anchors.fill: parent
        source: picture.preview
        fillMode: Image.PreserveAspectCrop
        horizontalAlignment: Image.AlignRight
        asynchronous: true
        cache: false
        sourceSize.width: width
    }

    Rectangle {
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        width: Math.min(parent.width, 640)
        height: Math.min(parent.height, 300)
        gradient: Gradient {
            orientation: Gradient.Horizontal

            GradientStop {
                position: 0
                color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.2 + Look.velo * 0.75)
            }

            GradientStop {
                position: 1
                color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0)
            }
        }
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: Look.e6
        spacing: Look.e2

        Text {
            Layout.fillWidth: true
            text: picture.publish.review.blocking > 0 && picture.publish.at === 1 ? App.tr("%1 (with problems)", picture.publish.name) : picture.publish.name
            elide: Text.ElideRight
            font.family: Look.letraTitulo
            font.pixelSize: Look.tPagina
            font.weight: Font.Bold
            font.letterSpacing: -0.03 * Look.tPagina
            color: Look.escarcha
        }

        Text {
            Layout.fillWidth: true
            visible: picture.publish.version !== ""
            text: [picture.kind === "scene" ? App.tr("Scene") : App.tr("Wallpaper"), App.tr("version %1", picture.publish.version)].concat(picture.publish.author !== "" ? [App.tr("by %1", picture.publish.author)] : []).join(" · ")
            elide: Text.ElideRight
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            font.weight: Font.Medium
            color: Look.escarcha
        }
    }
}
