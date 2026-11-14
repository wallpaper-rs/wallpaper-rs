import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

ColumnLayout {
    id: verify

    required property var review
    property string built: ""
    property var signature: null
    readonly property string verdict: (signature ?? {}).state ?? ""

    function check() {
        signature = JSON.parse(review.publish.page.editor.rig("", JSON.stringify({
            "signature": {
                "path": built,
                "keys": [key.text]
            }
        })) || "{}");
    }

    spacing: Look.e2

    Text {
        Layout.fillWidth: true
        text: App.tr("Check the signature of a package already made")
        wrapMode: Text.WordWrap
        font.family: Look.letraTexto
        font.pixelSize: Look.tCuerpo
        color: Look.escarcha
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            Layout.fillWidth: true
            placeholderText: App.tr("Package file; the folder when empty")
            text: verify.built
            onTextEdited: verify.built = text
        }

        QQC2.Button {
            flat: true
            text: App.tr("Choose…")
            onClicked: packages.open()
        }
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.TextField {
            id: key

            Layout.fillWidth: true
            placeholderText: App.tr("Public key of the author")
        }

        QQC2.Button {
            flat: true
            text: App.tr("Choose…")
            onClicked: keys.open()
        }
    }

    QQC2.Button {
        text: App.tr("Check the signature")
        enabled: key.text !== ""
        onClicked: verify.check()
    }

    PublishNote {
        Layout.fillWidth: true
        visible: verify.verdict !== ""
        tone: verify.verdict === "valid" ? "brote" : verify.verdict === "invalid" ? "coral" : ""
        text: ({
                "unsigned": App.tr("Not signed: build it with an author key to sign it."),
                "unchecked": App.tr("Signed; give a public key to check it."),
                "valid": App.tr("Signature valid, made by %1", ((verify.signature ?? {}).signers ?? []).join(", ")),
                "invalid": App.tr("Signature not valid: %1", (verify.signature ?? {}).error ?? "")
            })[verify.verdict] ?? ""
    }

    FileDialog {
        id: packages

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: verify.built = App.path(selectedFile)
    }

    FileDialog {
        id: keys

        nameFilters: [App.tr("Public keys") + " (*.pub)"]
        onAccepted: key.text = App.path(selectedFile)
    }
}
