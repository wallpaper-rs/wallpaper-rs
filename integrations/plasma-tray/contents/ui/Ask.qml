import QtQuick
import QtQuick.Layouts
import "../look"

Rectangle {
    id: ask

    required property var daemon
    property var inputs: ({})
    readonly property alias logic: logic
    readonly property string message: [daemon.tr("The wallpaper %1 asks for your permission:", String(logic.ask.title).replace(/</g, "‹").replace(/>/g, "›"))].concat(logic.ask.inputs.map(input => logic.explained[input].join(": "))).concat(logic.pending ? [daemon.tr("Confirm it in the desktop notification.")] : []).concat(logic.quiet && !logic.pending ? [daemon.tr("The notification was closed without an answer; try again in a few minutes.")] : []).concat(logic.problem ? [logic.problem] : []).join("\n")

    visible: logic.ask.inputs.length > 0
    implicitHeight: body.implicitHeight + 2 * Look.e3
    radius: Look.radio
    color: Look.nocheAlta
    border.width: Look.linea
    border.color: logic.problem ? Look.coral : Look.nocheLinea

    Consent {
        id: logic

        visible: false
        daemon: ask.daemon
        inputs: ask.inputs
    }

    ColumnLayout {
        id: body

        anchors.fill: parent
        anchors.margins: Look.e3
        spacing: Look.e2

        Words {
            Layout.fillWidth: true
            text: ask.message
            wrapMode: Text.Wrap
            color: logic.problem ? Look.coral : Look.escarcha
        }

        RowLayout {
            spacing: Look.e2

            Command {
                glyph: "check"
                text: ask.daemon.tr("Allow")
                enabled: !logic.pending && !logic.quiet
                onClicked: logic.answer(true)
            }

            Command {
                quiet: true
                text: ask.daemon.tr("Not now")
                onClicked: logic.answer(false)
            }
        }
    }
}
