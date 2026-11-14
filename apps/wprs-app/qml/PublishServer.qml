import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: server

    required property var where
    readonly property var publish: where.publish
    readonly property alias fault: fault

    Layout.fillWidth: true
    spacing: Look.e1

    PublishPlace {
        icon: "cloud-arrow-up"
        title: App.tr("Upload to my server")
        note: server.publish.linked ? server.where.server : App.tr("You have no server connected.")
        lit: server.publish.place === "server"
        off: !server.publish.linked
        last: server.publish.memory.place === "server" && server.publish.linked
        onPicked: server.where.pick("server")

        Opcion {
            Layout.fillWidth: true
            icon: server.where.hidden ? "circle" : "radio-button-fill"
            lit: !server.where.hidden
            text: App.tr("Public")
            note: App.tr("in the catalog and its search")
            onClicked: server.where.hidden = false
        }

        Opcion {
            Layout.fillWidth: true
            icon: server.where.hidden ? "radio-button-fill" : "circle"
            lit: server.where.hidden
            text: App.tr("Hidden")
            note: App.tr("only whoever has the link; it is not private")
            onClicked: server.where.hidden = true
        }

        PublishNote {
            Layout.fillWidth: true
            visible: !server.publish.unsigned
            text: App.tr("The first time, your signature is registered in your account by itself; we let you know.")
        }

        PublishFault {
            id: fault

            Layout.fillWidth: true
            where: server.where
            place: "server"
        }
    }

    QQC2.Button {
        Layout.leftMargin: PublishLook.iconoDestino + 2 * Look.e3
        visible: !server.publish.linked
        flat: true
        text: App.tr("Connect one in Account ›")
        onClicked: {
            const window = applicationWindow();
            window.current = window.names.indexOf("account");
        }
    }
}
