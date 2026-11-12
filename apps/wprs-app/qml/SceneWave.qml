import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: strip

    property var wave: ({})
    property real fadeIn: 0
    property real fadeOut: 0
    property bool fades: true
    readonly property var peaks: wave.peaks ?? []
    readonly property real seconds: Number(strip.seconds ?? 0)

    signal faded(string key, real seconds)

    function seconds_at(x) {
        return Math.round(Math.min(60, Math.max(0, x / Math.max(1, width) * seconds)) * 100) / 100;
    }

    implicitHeight: Kirigami.Units.gridUnit * 3
    onWaveChanged: plot.requestPaint()
    onFadeInChanged: plot.requestPaint()
    onFadeOutChanged: plot.requestPaint()
    onWidthChanged: plot.requestPaint()

    Canvas {
        id: plot

        anchors.fill: parent
        Accessible.name: App.tr("Waveform of the sound")
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            ctx.strokeStyle = Kirigami.Theme.disabledTextColor;
            ctx.strokeRect(0, 0, width, height);
            const peaks = strip.peaks;
            if (peaks.length === 0)
                return;
            const middle = height / 2, step = width / peaks.length;
            ctx.fillStyle = Kirigami.Theme.highlightColor;
            peaks.forEach((p, i) => ctx.fillRect(i * step, middle - p * middle, Math.max(1, step - 0.5), Math.max(1, p * height)));
            if (!strip.fades || strip.seconds <= 0)
                return;
            ctx.strokeStyle = Kirigami.Theme.textColor;
            ctx.lineWidth = 1.5;
            const rise = Math.min(width, strip.fadeIn / strip.seconds * width), fall = Math.min(width, strip.fadeOut / strip.seconds * width);
            ctx.beginPath();
            ctx.moveTo(0, height);
            ctx.lineTo(rise, 0);
            ctx.lineTo(Math.max(rise, width - fall), 0);
            ctx.lineTo(width, height);
            ctx.stroke();
        }
    }

    Repeater {
        model: strip.fades && strip.seconds > 0 ? [["fade_in", true], ["fade_out", false]] : []

        delegate: Rectangle {
            id: grip

            required property var modelData
            readonly property real amount: modelData[1] ? strip.fadeIn : strip.fadeOut
            readonly property real edge: Math.min(strip.width, amount / strip.seconds * strip.width)

            x: (modelData[1] ? edge : strip.width - edge) - width / 2
            y: 0
            width: Kirigami.Units.gridUnit * 0.7
            height: width
            radius: width / 2
            color: Kirigami.Theme.backgroundColor
            border.color: Kirigami.Theme.textColor
            Accessible.name: modelData[1] ? App.tr("Fade in") : App.tr("Fade out")

            DragHandler {
                yAxis.enabled: false
                xAxis.minimum: -grip.width / 2
                xAxis.maximum: strip.width - grip.width / 2
                onActiveChanged: {
                    if (!active)
                        strip.faded(grip.modelData[0], strip.seconds_at(grip.modelData[1] ? grip.x + grip.width / 2 : strip.width - grip.x - grip.width / 2));
                }
            }
        }
    }
}
