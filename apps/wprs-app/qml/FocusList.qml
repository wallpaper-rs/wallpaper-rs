import QtQuick
import org.kde.kirigami as Kirigami

ListView {
    id: list

    readonly property bool focusList: true

    activeFocusOnTab: true
    keyNavigationEnabled: true
    highlightFollowsCurrentItem: true
    highlightMoveDuration: 0
    highlight: Rectangle {
        visible: list.activeFocus
        color: "transparent"
        border.color: Kirigami.Theme.focusColor
        border.width: 2
        radius: Kirigami.Units.smallSpacing
    }
}
