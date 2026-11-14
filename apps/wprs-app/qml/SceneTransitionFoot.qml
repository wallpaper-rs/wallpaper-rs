import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: foot

    required property var page
    readonly property var tool: page.settings ? page.settings.passage : null
    readonly property var current: tool ? tool.current : null
    readonly property var families: ({
            "sine": App.tr("gentle"),
            "quad": App.tr("light"),
            "cubic": App.tr("medium"),
            "quart": App.tr("strong"),
            "quint": App.tr("very strong"),
            "expo": App.tr("sharp"),
            "circ": App.tr("round"),
            "back": App.tr("going past"),
            "elastic": App.tr("springy"),
            "bounce": App.tr("bouncing")
        })

    function curveName(curve) {
        if (Array.isArray(curve))
            return App.tr("own curve");
        if (curve === "linear")
            return App.tr("even");
        if (curve === "step")
            return App.tr("jump at the end");
        if (curve === "hold")
            return App.tr("hold, then jump");
        const both = curve.indexOf("in_out_") === 0;
        const family = families[curve.replace(/^(in_out_|in_|out_)/, "")] ?? curve;
        if (both)
            return App.tr("soft at both ends, %1", family);
        if (curve.indexOf("in_") === 0)
            return App.tr("soft at the start, %1", family);
        return App.tr("soft at the end, %1", family);
    }

    visible: tool !== null && tool.open
    color: Look.noche

    Rectangle {
        width: parent.width
        height: Look.linea
        color: Look.nocheLinea
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.topMargin: Look.linea
        anchors.leftMargin: Look.e3
        anchors.rightMargin: Look.e3
        anchors.bottomMargin: Look.e2
        spacing: Look.e1

        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: Look.barraLienzoAlto - Look.linea
            spacing: Look.e2

            QQC2.Label {
                text: App.tr("Progress")
                font.weight: Font.DemiBold
            }

            QQC2.Slider {
                id: slider

                Layout.fillWidth: true
                from: 0
                to: 1
                stepSize: 0.01
                value: foot.tool ? foot.tool.progress : 0
                Accessible.name: App.tr("Progress")
                onMoved: {
                    foot.tool.playing = false;
                    foot.tool.progress = value;
                }
            }

            QQC2.Label {
                text: foot.current ? App.tr("%1 s / curve: %2", Number(foot.current.duration_ms / 1000).toLocaleString(Qt.locale(), "f", 1), foot.curveName(foot.current.curve)) : ""
                color: Look.bruma
            }

            QQC2.ToolButton {
                id: presets

                text: App.tr("Curve…")
                icon.name: "chart-line"
                enabled: foot.current !== null
                onClicked: menu.popup(presets, 0, presets.height)

                QQC2.Menu {
                    id: menu

                    Repeater {
                        model: foot.tool && foot.tool.info.curves ? foot.tool.info.curves : []

                        delegate: QQC2.MenuItem {
                            required property string modelData

                            text: foot.curveName(modelData)
                            checkable: true
                            checked: foot.current !== null && foot.current.curve === modelData
                            onTriggered: foot.tool.act({
                                "curve": modelData
                            })
                        }
                    }
                }
            }
        }

        SceneTransitionCurve {
            Layout.fillWidth: true
            Layout.fillHeight: true
            tool: foot.tool
        }
    }
}
