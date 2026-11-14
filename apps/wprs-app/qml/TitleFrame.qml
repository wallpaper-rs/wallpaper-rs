import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: frame

    required property var page
    required property real sideBottom
    required property bool none
    readonly property bool full: !!(page && page.full)
    readonly property alias seat: seat
    readonly property real bandLow: page && page.full !== true && page.bandLow > 0 ? page.bandLow : 0
    readonly property real upperLow: bandLow > 0 && page.upperHeight > 0 ? page.upperHeight : 0
    readonly property real bandBottom: height - Look.margenAbajo
    readonly property real bandTop: Math.max(sideBottom, bandBottom - (bandLow > 0 && page.bandHigh > 0 ? page.bandHigh : bandBottom))
    readonly property bool fits: bandLow > 0 && bandBottom - bandTop >= bandLow && bandTop - Look.e3 - column.y >= upperLow
    property bool banded: false
    property real placedFor: 0
    readonly property real stacked: upperLow > 0 ? upperLow + Look.e4 : 0
    readonly property rect band: banded ? Qt.rect(Look.margen, bandTop, width - 2 * Look.margen, bandBottom - bandTop) : Qt.rect(column.x, column.y + stacked, column.width, Math.max(0, column.height - stacked))
    readonly property rect upper: banded ? Qt.rect(column.x, bandTop - Look.e3 - upperLow, column.width, upperLow) : Qt.rect(column.x, column.y, column.width, upperLow)
    readonly property point noticeAt: banded ? Qt.point(column.x + Look.e4, upper.y - Look.e3) : Qt.point(column.x + Look.e4, column.y + column.height - Look.e3)
    readonly property point hintsAt: Qt.point(width - Look.margen, height - Look.margenAbajo)

    signal escaped

    function place(grow) {
        placedFor = bandLow;
        if (grow || !fits)
            banded = fits;
    }

    function resized() {
        place(true);
        regrow.restart();
    }

    Keys.onEscapePressed: frame.escaped()
    Component.onCompleted: resized()
    onWidthChanged: resized()
    onHeightChanged: resized()
    onSideBottomChanged: resized()
    onPageChanged: resized()
    onBandLowChanged: {
        if (banded)
            settle.restart();
        else if (placedFor === 0)
            resized();
    }

    Timer {
        id: settle

        interval: Look.paso * 2
        onTriggered: frame.place(false)
    }

    Timer {
        id: regrow

        interval: Look.paso * 2
        onTriggered: frame.place(true)
    }

    RoundVeil {
        y: -120
        width: Look.menuAncho + 300
        height: frame.sideBottom + 320
        centerX: 0
        centerY: 0.4
        reachY: height * 0.6
        inner: Look.velo * 0.95
        middle: Look.velo * 0.82
    }

    RoundVeil {
        visible: !frame.full
        x: now.x - 132
        y: now.y - 96
        width: frame.width - x
        height: now.height + 96 + 176
        centerX: 1
        centerY: 0.4
        reachY: height * 0.72
        inner: 0.2 + Look.velo * 0.78
        middle: 0.14 + Look.velo * 0.76
    }

    Rectangle {
        visible: frame.full
        anchors.fill: parent
        color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.55 + Look.velo * 0.3)
    }

    BandVeil {
        visible: !frame.full
        y: frame.bandLow > 0 ? Math.max(0, (frame.page.bandFrom !== undefined ? frame.page.bandFrom : frame.band.y) - Look.veloBanda) : frame.height * 0.55
        width: frame.width
        height: frame.height - y
    }

    Column {
        id: now

        visible: !frame.full
        anchors.top: parent.top
        anchors.right: parent.right
        anchors.topMargin: Look.margenArriba
        anchors.rightMargin: Look.margen
        spacing: Look.e3

        Rectangle {
            anchors.right: parent.right
            visible: frame.none
            width: example.implicitWidth + Look.e3 * 2
            height: example.implicitHeight + Look.e2 * 2
            radius: Look.radioChico
            color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.82)

            RowLayout {
                id: example

                anchors.centerIn: parent
                spacing: Look.e2

                Kirigami.Icon {
                    Layout.preferredWidth: Look.e4
                    Layout.preferredHeight: Look.e4
                    source: "info"
                    color: Look.bruma
                }

                Text {
                    text: App.tr("Example picture: “%1”, one of the bundled scenes", App.tr("Snowfall"))
                    color: Look.bruma
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tPie
                }
            }
        }

        StatusBar {
            anchors.right: parent.right
            width: Math.min(implicitWidth, frame.width - Look.menuAncho - Look.margen * 2)
        }
    }

    Kirigami.ActionToolBar {
        id: tools

        x: column.x
        y: frame.full ? Look.margenArriba : now.y + now.height + Look.e5
        width: column.width
        alignment: Qt.AlignRight
        actions: frame.page?.actions ?? []
    }

    Item {
        id: column

        x: Look.margen + Look.menuAncho
        y: tools.actions.length > 0 ? tools.y + tools.height + Look.e3 : tools.y
        width: frame.width - x - Look.margen
        height: frame.height - y - Look.margenAbajo - Look.atajosAlto - Look.e4
        onYChanged: frame.resized()
    }

    Item {
        id: seat

        x: frame.bandLow > 0 ? 0 : column.x
        y: frame.bandLow > 0 ? 0 : column.y
        width: frame.bandLow > 0 ? frame.width : column.width
        height: frame.bandLow > 0 ? frame.height : column.height
    }

    Binding {
        target: frame.page
        property: "band"
        value: frame.band
        when: frame.bandLow > 0
    }

    Binding {
        target: frame.page
        property: "upper"
        value: frame.upper
        when: frame.bandLow > 0
    }
}
