import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: veils

    required property Item host
    required property Item lead
    readonly property alias band: bandVeil

    anchors.fill: parent

    RoundVeil {
        y: -120
        width: Look.menuAncho + 300
        height: veils.host.sideBottom + 320
        centerX: 0
        centerY: 0.4
        reachY: height * 0.6
        inner: Look.velo * 0.95
        middle: Look.velo * 0.82
    }

    RoundVeil {
        visible: !veils.host.full
        x: veils.lead.x - 132
        y: veils.lead.y - 96
        width: veils.host.width - x
        height: veils.lead.height + 96 + 176
        centerX: 1
        centerY: 0.4
        reachY: height * 0.72
        inner: 0.2 + Look.velo * 0.78
        middle: 0.14 + Look.velo * 0.76
    }

    Rectangle {
        visible: veils.host.full
        anchors.fill: parent
        color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.55 + Look.velo * 0.3)
    }

    BandVeil {
        id: bandVeil

        visible: !veils.host.full
        y: veils.host.page && veils.host.page.bandFrom !== undefined ? Math.max(0, Math.min(veils.host.bandLow > 0 ? veils.host.height : veils.host.height * 0.55, veils.host.seat.y + veils.host.page.bandFrom - Look.veloBanda)) : veils.host.bandLow > 0 ? Math.max(0, veils.host.band.y - Look.veloBanda) : veils.host.height * 0.55
        width: veils.host.width
        height: veils.host.height - y
    }
}
