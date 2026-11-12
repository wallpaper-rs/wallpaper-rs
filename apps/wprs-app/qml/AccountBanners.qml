import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Column {
    required property var host

    width: host.width

    Kirigami.InlineMessage {
        width: parent.width
        type: Kirigami.MessageType.Error
        text: App.backend.banner
        visible: text !== ""
    }

    AccountNotice {
        width: parent.width
        host: parent.host
    }
}
