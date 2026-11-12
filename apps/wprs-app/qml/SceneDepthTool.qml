import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: tool

    required property var input
    readonly property var page: input.page
    readonly property var images: (page.view.assets ?? []).filter(a => a.kind === "image" && !a.media).map(a => a.id)
    property var model: null

    function ask() {
        model = JSON.parse(page.editor.rig("", JSON.stringify({
                        "depth_model": {}
                    })) || "null");
    }

    Component.onCompleted: ask()

    RowLayout {
        QQC2.ComboBox {
            id: source

            Layout.fillWidth: true
            model: tool.images
        }

        QQC2.Button {
            text: App.tr("Make a depth map")
            icon.name: "layer-visible-on"
            enabled: tool.model?.ready === true && tool.images.length > 0
            onClicked: tool.page.run("depth_map", {
                    "asset": tool.images[source.currentIndex],
                    "pointer": tool.input.field.pointer
                })
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: tool.model !== null && tool.model.ready !== true
        wrapMode: Text.Wrap
        text: tool.model?.installed === false ? App.tr("The depth model is not installed, so no depth map can be made yet. Install it with: %1", tool.model?.install ?? "") : App.tr("The library that runs the depth model is not installed, so no depth map can be made yet.")
    }
}
