import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: pickers

    required property var daemon
    property string output: ""
    property var info: ({})

    HdrPicker {
        daemon: pickers.daemon
        output: pickers.output
        info: pickers.info
    }

    TransitionPicker {
        daemon: pickers.daemon
        output: pickers.output
        info: pickers.info
    }
}
