import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Binding {
    required property var host

    target: Look
    property: "escalon"
    value: Look.step(host.width, host.height)
    Component.onCompleted: {
        const theme = host.Kirigami.Theme;
        theme.negativeTextColor = Look.coral;
        theme.neutralTextColor = Look.ambar;
        theme.positiveTextColor = Look.brote;
        theme.linkColor = Look.luna;
    }
}
