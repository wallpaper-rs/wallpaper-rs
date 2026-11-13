import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Column {
    id: notices

    required property var host
    property string text: ""
    property string action: ""
    property var callback: null
    readonly property bool busy: App.backend.banner !== "" || account.visible

    function show(message, timeout, actionText, onAction) {
        text = message ?? "";
        action = actionText ?? "";
        callback = onAction ?? null;
        later.interval = typeof timeout === "number" && timeout > 0 ? timeout : 7000;
        later.restart();
    }

    function hide() {
        later.stop();
        text = "";
        callback = null;
    }

    width: Look.avisoAncho
    Accessible.role: Accessible.StaticText

    Timer {
        id: later

        onTriggered: notices.hide()
    }

    Loader {
        width: parent.width
        active: App.backend.banner !== ""
        visible: active

        sourceComponent: Kirigami.InlineMessage {
            type: Kirigami.MessageType.Error
            text: App.backend.banner
            visible: true
        }
    }

    AccountNotice {
        id: account

        width: visible ? parent.width : Look.avisoAncho
        host: notices.host
        visible: phase === "signed_out" && App.backend.banner === ""
    }

    Rectangle {
        id: passive

        visible: notices.text !== "" && !notices.busy
        width: Math.min(notices.width, row.implicitWidth + Look.e4 + Look.e3)
        height: row.implicitHeight + Look.e3 * 2
        radius: Look.radio
        color: Look.nocheAlta
        border.width: Look.linea
        border.color: Look.nocheLinea
        Accessible.role: Accessible.AlertMessage
        Accessible.name: notices.text

        RowLayout {
            id: row

            anchors.fill: parent
            anchors.leftMargin: Look.e4
            anchors.rightMargin: Look.e3 - Look.e1
            spacing: Look.e3

            Kirigami.Icon {
                Layout.preferredWidth: Look.e5 - Look.e1
                Layout.preferredHeight: Look.e5 - Look.e1
                source: "info"
                color: Look.bruma
            }

            QQC2.Label {
                Layout.fillWidth: true
                Layout.maximumWidth: notices.width - Look.e8 * 2
                text: notices.text
                wrapMode: Text.Wrap
                color: Look.escarcha
            }

            QQC2.Button {
                visible: notices.action !== ""
                flat: true
                text: notices.action
                onClicked: {
                    const run = notices.callback;
                    notices.hide();
                    run();
                }
            }

            QQC2.Button {
                flat: true
                icon.name: "x"
                display: QQC2.AbstractButton.IconOnly
                text: App.tr("Close")
                Accessible.name: text
                onClicked: notices.hide()
            }
        }
    }
}
