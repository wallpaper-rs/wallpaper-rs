import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: stage

    required property var preview
    readonly property string kind: preview.plays.kind ?? "fade"
    readonly property real sample: preview.sampling ? preview.sample : 1

    function paint() {
        Look.tone(JSON.parse(Look.bridge.derive(after.shown)));
    }

    PreviewPicture {
        anchors.fill: parent
        corner: 0
        whole: true
        item: stage.preview.current
        visible: stage.sample < 1
    }

    Item {
        width: stage.kind === "own" ? stage.width * stage.sample : stage.width
        height: stage.height
        clip: stage.kind === "own"
        opacity: stage.kind === "fade" ? stage.sample : stage.kind === "none" && stage.sample < 0.5 ? 0 : 1

        PreviewPicture {
            id: after

            width: stage.width
            height: stage.height
            corner: 0
            whole: true
            item: stage.preview.alone ? stage.preview.current : stage.preview.candidate
            file: stage.preview.alone ? "" : String(stage.preview.session.path ?? "")
            onShownChanged: stage.paint()
        }
    }

    RoundVeil {
        x: stage.width - width
        width: PreviewLook.veloEsquinaAncho
        height: PreviewLook.veloEsquinaAlto
        centerX: 1
        centerY: 0
        inner: Look.velo * 0.9
        middle: Look.velo * 0.7
    }

    BandVeil {
        y: stage.height * 0.45
        width: stage.width
        height: stage.height - y
    }
}
