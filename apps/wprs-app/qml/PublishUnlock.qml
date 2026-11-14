import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: unlock

    required property var sign
    readonly property var current: sign.current
    readonly property bool locked: current !== null && current.encrypted !== false
    readonly property alias pass: pass

    spacing: Look.e3

    Text {
        Layout.fillWidth: true
        text: App.tr("Signed with your author key “%1”.", unlock.sign.keyName)
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    PublishError {
        visible: unlock.sign.wrong
        title: App.tr("That is not the password of “%1”", unlock.sign.keyName)
        text: App.tr("Type it again; capitals count.")
        more: App.tr("Forgot it? It cannot be recovered. You can create another signature and move your wallpapers to it, or publish this time without signing.")

        QQC2.Button {
            text: App.tr("Manage keys…")
            onClicked: unlock.sign.manage()
        }

        QQC2.Button {
            flat: true
            text: App.tr("Publish without signing")
            onClicked: unlock.sign.skip()
        }
    }

    Text {
        visible: unlock.locked
        text: App.tr("Password of “%1”", unlock.sign.keyName)
        font.family: Look.letraTexto
        font.pixelSize: Look.tPie
        color: Look.bruma
    }

    QQC2.TextField {
        id: pass

        Layout.fillWidth: true
        visible: unlock.locked
        echoMode: TextInput.Password
        Accessible.name: App.tr("Password of “%1”", unlock.sign.keyName)
        Keys.onReturnPressed: unlock.sign.confirm()
    }

    PublishMore {
        Text {
            text: App.tr("Sign with")
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }

        Repeater {
            model: unlock.sign.found

            delegate: Opcion {
                required property var modelData

                Layout.fillWidth: true
                icon: unlock.current !== null && unlock.current.secret === modelData.secret ? "radio-button-fill" : "circle"
                lit: unlock.current !== null && unlock.current.secret === modelData.secret
                text: modelData.name
                onClicked: unlock.sign.chosen = modelData.secret
            }
        }

        Text {
            Layout.fillWidth: true
            visible: unlock.current !== null && unlock.current.fingerprint !== ""
            text: App.tr("Fingerprint of “%1”: %2", unlock.sign.keyName, unlock.current ? unlock.current.fingerprint : "")
            wrapMode: Text.WrapAnywhere
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }

        Flow {
            Layout.fillWidth: true
            spacing: Look.e2

            QQC2.Button {
                flat: true
                text: App.tr("Save a copy…")
                onClicked: unlock.sign.copyAway()
            }

            QQC2.Button {
                flat: true
                text: App.tr("Manage keys…")
                onClicked: unlock.sign.manage()
            }

            QQC2.Button {
                flat: true
                text: App.tr("Choose a key file…")
                onClicked: unlock.sign.choose()
            }

            QQC2.Button {
                flat: true
                text: App.tr("Publish without signing")
                onClicked: unlock.sign.skip()
            }
        }
    }

    RowLayout {
        spacing: Look.e3

        QQC2.Button {
            highlighted: true
            text: App.tr("Sign it")
            enabled: unlock.current !== null
            onClicked: unlock.sign.confirm()
        }

        KeyCap {
            key: "Intro"
        }
    }
}
