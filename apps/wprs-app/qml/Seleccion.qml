import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

RowLayout {
    id: seleccion

    property int count
    default property alias actions: tail.data
    readonly property var widths: {
        const all = [];
        for (let i = 0; i < tail.children.length; i++) {
            const child = tail.children[i];
            if (child.visible)
                all.push(child.implicitWidth);
        }
        return all;
    }
    readonly property real line: widths.reduce((sum, width) => sum + width, 0) + Math.max(0, widths.length - 1) * tail.columnSpacing

    function fits(columns) {
        let sum = (columns - 1) * tail.columnSpacing;
        for (let column = 0; column < columns; column++) {
            let widest = 0;
            for (let i = column; i < widths.length; i += columns)
                widest = Math.max(widest, widths[i]);
            sum += widest;
        }
        return sum <= tail.width + 0.5;
    }

    spacing: Look.e3
    Accessible.role: Accessible.Grouping
    Accessible.name: chosen.text

    Text {
        id: chosen

        rightPadding: Look.e2
        text: App.trc("covers", "%1 chosen", seleccion.count)
        font.family: Look.letraTitulo
        font.pixelSize: Look.tFoco
        font.weight: Font.Bold
        font.letterSpacing: Look.tFoco * Look.apretado
        color: Look.escarcha
    }

    GridLayout {
        id: tail

        Layout.preferredWidth: seleccion.line
        Layout.minimumWidth: Math.max(0, ...seleccion.widths)
        columns: {
            for (let columns = Math.max(1, seleccion.widths.length); columns > 1; columns--) {
                if (seleccion.fits(columns))
                    return columns;
            }
            return 1;
        }
        columnSpacing: Look.e3
        rowSpacing: Look.e2
    }
}
