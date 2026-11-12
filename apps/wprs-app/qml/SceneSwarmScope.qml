import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: scope

    required property var tools
    readonly property var info: tools.info?.scope ?? ({})
    readonly property var layers: info.layers ?? []
    readonly property var names: ({
            "stretch": App.tr("Stretch"),
            "frames": App.tr("Frames"),
            "fps": App.tr("Frames per second"),
            "mix": App.tr("Mix frames"),
            "brightness": App.tr("Brightness"),
            "cycles": App.tr("Cycles"),
            "order": App.tr("Drawing order"),
            "rope": App.tr("Rope"),
            "trail": App.tr("Trail"),
            "texture": App.tr("Texture"),
            "cutout": App.tr("Cutout"),
            "orient": App.tr("Orientation"),
            "angle": App.tr("Angle"),
            "shader": App.tr("Shader"),
            "lit": App.tr("Light"),
            "normal": App.tr("Normal map"),
            "refract": App.tr("Refraction"),
            "size_over_life": App.tr("Size over life"),
            "color_over_life": App.tr("Color over life")
        })
    readonly property string layer: tools.renderLayer

    visible: layers.length > 0

    Kirigami.Heading {
        level: 4
        text: App.tr("Render settings for")
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: [App.tr("The system, for every layer")].concat(scope.layers.map(id => App.tr("Only the layer %1", id)))
        currentIndex: Math.max(0, scope.layers.indexOf(scope.layer) + 1)
        onActivated: index => scope.tools.renderLayer = index > 0 ? scope.layers[index - 1] : ""
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: scope.layer !== "" && !scope.info.own
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("This layer follows the render of the system. Your first change below gives it a render of its own, copied from the system's.")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: scope.layer !== "" && !!scope.info.own
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: (scope.info.differs ?? []).length ? App.tr("This layer has its own render. It differs from the system's in: %1", scope.info.differs.map(k => scope.names[k] ?? k).join(", ")) : App.tr("This layer has its own render, equal to the system's for now.")
    }

    QQC2.Button {
        visible: scope.layer !== "" && !!scope.info.own
        text: App.tr("Use the render of the system again")
        icon.name: "edit-undo"
        onClicked: scope.tools.edit("scope", {
                "layer": scope.layer,
                "own": false
            })
    }
}
