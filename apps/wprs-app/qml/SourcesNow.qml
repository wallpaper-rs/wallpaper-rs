import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: now

    property int sources: 0
    property int photos: 0

    spacing: Look.e2

    Text {
        anchors.right: parent.right
        text: App.tr("Sources")
        font.family: Look.letraTitulo
        font.pixelSize: Look.tPortada
        font.weight: Font.Bold
        font.letterSpacing: -0.03 * Look.tPortada
        color: Look.escarcha
    }

    Text {
        anchors.right: parent.right
        text: now.sources === 0 ? App.tr("Photos that renew themselves · none yet") : now.sources === 1 ? App.tr("Photos that renew themselves · 1 source · %1 photos", now.photos) : App.tr("Photos that renew themselves · %1 sources · %2 photos", now.sources, now.photos)
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }

    Text {
        anchors.right: parent.right
        visible: now.sources === 0
        width: Look.vacioAncho
        horizontalAlignment: Text.AlignRight
        wrapMode: Text.WordWrap
        text: App.tr("Pick where the photos come from. They arrive in your library and you can rotate them with one button.")
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.bruma
    }
}
