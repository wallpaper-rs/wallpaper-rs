import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: view

    required property var page
    readonly property var tool: page.settings ? page.settings.passage : null
    readonly property var current: tool ? tool.current : null
    readonly property string scene: page.heading ?? ""
    readonly property bool coming: tool !== null && tool.way === "in"
    readonly property var points: current ? (current.fields ?? []).filter(f => f.type === "vec2") : []

    function value(f) {
        const set = current.params ?? {};
        return set[f.id] ?? f["default"] ?? [0.5, 0.5];
    }

    function put(f, x, y) {
        const params = {};
        params[f.id] = [Math.round(Math.max(0, Math.min(1, x)) * 1000) / 1000, Math.round(Math.max(0, Math.min(1, y)) * 1000) / 1000];
        tool.act({
            "params": params
        });
    }

    visible: tool !== null && tool.open

    Image {
        anchors.fill: parent
        source: view.tool ? view.tool.shown : ""
        fillMode: Image.Stretch
        cache: false
        asynchronous: true
        smooth: true
    }

    Row {
        x: Look.e3
        y: Look.e3
        spacing: Look.e2

        Repeater {
            model: [App.tr("%1 leaves", view.coming ? (view.tool ? view.tool.beforeName : "") : view.scene), App.tr("%1 comes in · %2 %", view.coming ? view.scene : (view.tool ? view.tool.beforeName : ""), Math.round((view.tool ? view.tool.progress : 0) * 100))]

            delegate: Rectangle {
                required property string modelData

                width: chip.implicitWidth + Look.e3 * 2
                height: chip.implicitHeight + Look.e1 * 2
                radius: Look.radioChico
                color: Look.nocheAlta
                border.color: Look.nocheLinea

                QQC2.Label {
                    id: chip

                    anchors.centerIn: parent
                    text: parent.modelData
                    font.pixelSize: Look.tPie
                }
            }
        }
    }

    Repeater {
        model: view.points

        delegate: Rectangle {
            id: handle

            required property var modelData
            readonly property var at: view.value(modelData)

            x: at[0] * view.width - width / 2
            y: at[1] * view.height - height / 2
            width: Look.e5
            height: Look.e5
            radius: width / 2
            color: drag.drag.active ? Look.ambar : Look.luna
            border.color: activeFocus ? Look.escarcha : Look.noche
            border.width: activeFocus ? Look.linea * 3 : Look.linea * 2
            activeFocusOnTab: true
            Accessible.role: Accessible.Slider
            Accessible.name: App.tr("%1 of the transition", view.tool.named(modelData.label, modelData.id))
            Keys.onLeftPressed: view.put(modelData, at[0] - 0.01, at[1])
            Keys.onRightPressed: view.put(modelData, at[0] + 0.01, at[1])
            Keys.onUpPressed: view.put(modelData, at[0], at[1] - 0.01)
            Keys.onDownPressed: view.put(modelData, at[0], at[1] + 0.01)

            MouseArea {
                id: drag

                anchors.fill: parent
                cursorShape: Qt.SizeAllCursor
                drag.target: handle
                onPressed: handle.forceActiveFocus()
                onReleased: view.put(handle.modelData, (handle.x + handle.width / 2) / view.width, (handle.y + handle.height / 2) / view.height)
            }
        }
    }
}
