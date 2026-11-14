import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Binding {
    required property var host
    property bool tight: false

    target: Look
    property: "escalon"
    value: tight && Look.step(host.width, host.height) !== "bajo" ? "compacto" : Look.step(host.width, host.height)
    Component.onCompleted: {
        const theme = host.Kirigami.Theme;
        theme.negativeTextColor = Look.coral;
        theme.neutralTextColor = Look.ambar;
        theme.positiveTextColor = Look.brote;
        theme.linkColor = Look.luna;
        theme.highlightColor = Qt.binding(() => Look.luna);
        theme.highlightedTextColor = Qt.binding(() => Look.noche);
        theme.focusColor = Look.escarcha;
        theme.hoverColor = Qt.binding(() => Look.bruma);
        theme.activeTextColor = Qt.binding(() => Look.luna);
    }
}
