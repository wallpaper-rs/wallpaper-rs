import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

Item {
    id: ficha

    property string title
    property string caption
    property string tone
    property string icon
    property bool fields
    property alias middle: mid.data
    default property alias actions: row.data
    property bool stacked: false

    function decide() {
        stacked = fields && heading.implicitWidth + mid.implicitWidth + row.implicitWidth + 2 * Look.e5 > width;
    }

    implicitHeight: stacked ? Look.e4 + words.implicitHeight + Look.e3 + mid.implicitHeight + Look.e3 : Math.max(words.implicitHeight, row.implicitHeight, fields ? mid.implicitHeight : 0) + Look.e4
    onWidthChanged: Qt.callLater(decide)
    onFieldsChanged: decide()
    Accessible.role: Accessible.Grouping
    Accessible.name: title

    Rectangle {
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    Column {
        id: words

        y: ficha.stacked ? Look.e4 : Look.e4 + (parent.height - Look.e4 - height) / 2
        width: ficha.fields ? heading.implicitWidth : parent.width - row.width - Look.e5

        Text {
            id: heading

            width: parent.width
            text: ficha.title
            elide: Text.ElideRight
            font.family: Look.letraTitulo
            font.pixelSize: Look.tFoco
            font.weight: Font.Bold
            font.letterSpacing: Look.tFoco * Look.apretado
            color: Look.escarcha
        }

        Nota {
            visible: ficha.caption !== "" && !ficha.fields
            text: ficha.caption
            tone: ficha.tone
            icon: ficha.icon
        }
    }

    Row {
        id: mid

        visible: ficha.fields
        onImplicitWidthChanged: Qt.callLater(ficha.decide)
        x: ficha.stacked ? 0 : heading.implicitWidth + Look.e5
        y: ficha.stacked ? words.y + words.height + Look.e3 : Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e4
    }

    RowLayout {
        id: row

        x: parent.width - width
        y: ficha.stacked ? words.y + (words.height - height) / 2 : Look.e4 + (parent.height - Look.e4 - height) / 2
        spacing: Look.e3
    }
}
