import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: ambient

    required property var page
    property var shown: null

    function load() {
        shown = JSON.parse(page.editor.rig("", JSON.stringify({
            "hemisphere": {}
        })) || "null");
    }

    function hex(name) {
        const v = JSON.parse(page.editor.value("/scene/canvas/" + name) || "null");
        return typeof v === "string" ? v : "#000000";
    }

    spacing: Look.e3
    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            ambient.load();
        }

        target: ambient.page.editor
    }

    Image {
        Layout.preferredWidth: Look.e7
        Layout.preferredHeight: Look.e7
        sourceSize.width: Look.e7 * 2
        sourceSize.height: Look.e7 * 2
        fillMode: Image.PreserveAspectFit
        source: ambient.shown?.sphere ?? ""
        cache: false
    }

    ColumnLayout {
        Layout.fillWidth: true
        spacing: 0

        SceneValueRow {
            text: App.tr("Light around")

            SceneColor {
                Layout.fillWidth: true
                Layout.maximumWidth: implicitWidth
                alpha: false
                hex: ambient.hex("ambient")
                onPicked: hex => ambient.page.run("ambient", {
                    "ambient": hex
                })
            }
        }

        SceneValueRow {
            text: App.tr("Light from above")

            SceneColor {
                Layout.fillWidth: true
                Layout.maximumWidth: implicitWidth
                alpha: false
                hex: ambient.hex("sky")
                onPicked: hex => ambient.page.run("ambient", {
                    "sky": hex
                })
            }
        }
    }
}
