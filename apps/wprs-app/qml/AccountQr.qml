import QtQuick
import io.github.wallpaper_rs.app

PanelChico {
    id: qr

    property string server
    property string fingerprint
    readonly property string modules: server === "" ? "" : maker.modules(server, fingerprint)
    readonly property int side: Math.round(Math.sqrt(modules.length))

    title: App.tr("Connect your phone")
    text: App.tr("Scan it with the wallpaper-rs app on your phone: it adds this server and checks its certificate. It carries no key and no password.")
    quiet: App.tr("Close")

    ConnectCode {
        id: maker
    }

    Rectangle {
        width: qr.width - 2 * Look.e4
        height: width
        radius: Look.radioChico
        color: Look.escarcha
        Accessible.role: Accessible.Graphic
        Accessible.name: App.tr("Code with the address of the server")

        Canvas {
            id: canvas

            anchors.fill: parent
            anchors.margins: Look.e4
            onPaint: {
                const context = getContext("2d");
                context.reset();
                if (qr.side < 1)
                    return;
                const cell = Math.floor(Math.min(width, height) / qr.side);
                const start = Math.floor((width - cell * qr.side) / 2);
                context.fillStyle = Look.noche;
                for (let at = 0; at < qr.modules.length; at++) {
                    if (qr.modules[at] === "1")
                        context.fillRect(start + (at % qr.side) * cell, start + Math.floor(at / qr.side) * cell, cell, cell);
                }
            }
        }
    }

    onModulesChanged: canvas.requestPaint()
    onOpened: canvas.requestPaint()
}
