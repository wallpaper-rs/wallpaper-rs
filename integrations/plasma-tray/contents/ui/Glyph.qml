import QtQuick
import org.kde.kirigami as Kirigami
import "../look"

Kirigami.Icon {
    property string name
    property string weight: "regular"
    readonly property bool filled: name.endsWith("-fill")
    readonly property bool heavy: name.endsWith("-bold")
    readonly property string base: filled ? name.slice(0, -5) : heavy ? name.slice(0, -5) : name
    readonly property string folder: filled ? "fill" : heavy ? "bold" : weight
    property color tone: Look.bruma

    implicitWidth: Look.icono
    implicitHeight: Look.icono
    source: name ? Qt.resolvedUrl("../look/icons/" + folder + "/" + base + ".svg") : ""
    isMask: true
    color: tone
}
