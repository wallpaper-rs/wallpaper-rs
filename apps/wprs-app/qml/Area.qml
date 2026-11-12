import QtQuick
import QtQuick.Controls as QQC2

QQC2.TextArea {
    id: area

    property bool enterApplies: false

    function apply(event) {
        if (!enterApplies || (event.modifiers & Qt.ShiftModifier)) {
            event.accepted = false;
            return;
        }
        event.accepted = true;
        area.editingFinished();
    }

    function leave(event, forward) {
        if (!enterApplies) {
            event.accepted = false;
            return;
        }
        event.accepted = true;
        nextItemInFocusChain(forward).forceActiveFocus(forward ? Qt.TabFocusReason : Qt.BacktabFocusReason);
    }

    Keys.onReturnPressed: event => area.apply(event)
    Keys.onEnterPressed: event => area.apply(event)
    Keys.onTabPressed: event => area.leave(event, true)
    Keys.onBacktabPressed: event => area.leave(event, false)

    Component.onCompleted: {
        if (background)
            background.visible = true;
        else
            background = frame.createObject(area);
    }

    Component {
        id: frame

        Rectangle {
            color: "transparent"
            border.width: area.activeFocus ? 2 : 1
            border.color: area.activeFocus ? area.palette.highlight : area.palette.mid
        }
    }
}
