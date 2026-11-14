import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: first

    required property var sign
    readonly property string author: name.text.trim()

    function make() {
        if (author !== "")
            sign.create(author, password.text);
        password.text = "";
    }

    spacing: Look.e3
    onVisibleChanged: visible && name.text === "" && (name.text = sign.publish.author)

    Text {
        Layout.fillWidth: true
        text: App.tr("Your signature shows that a wallpaper is yours and that nobody changed it on the way. It is made once and serves for everything you publish.")
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tExplica
        color: Look.bruma
    }

    Text {
        text: App.tr("Your author name")
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    QQC2.TextField {
        id: name

        Layout.fillWidth: true
        Accessible.name: App.tr("Your author name")
        Keys.onReturnPressed: first.make()
    }

    Text {
        text: App.tr("Password, if you want one")
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    QQC2.TextField {
        id: password

        Layout.fillWidth: true
        echoMode: TextInput.Password
        Accessible.name: App.tr("Password, if you want one")
        Keys.onReturnPressed: first.make()
    }

    Nota {
        Layout.fillWidth: true
        text: App.tr("It keeps anyone else who uses this computer from signing as you. We ask for it every time you publish.")
    }

    PublishMore {
        QQC2.Button {
            flat: true
            text: App.tr("Choose a key file…")
            onClicked: first.sign.choose()
        }

        QQC2.Button {
            flat: true
            text: App.tr("Publish without signing")
            onClicked: first.sign.skip()
        }
    }

    RowLayout {
        spacing: Look.e3

        QQC2.Button {
            highlighted: true
            text: App.tr("Create my signature")
            enabled: first.author !== ""
            onClicked: first.make()
        }

        KeyCap {
            key: "Intro"
        }
    }
}
