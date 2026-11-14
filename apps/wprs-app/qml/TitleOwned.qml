import QtQuick

Loader {
    required property var frame

    anchors.right: parent.right
    active: frame.owned
    visible: active
    sourceComponent: frame.owned ? frame.page.now : null
}
