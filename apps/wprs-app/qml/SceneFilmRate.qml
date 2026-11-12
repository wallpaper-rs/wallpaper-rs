import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: controls

    required property var film

    QQC2.Label {
        text: App.tr("Speed")
    }

    QQC2.SpinBox {
        from: 0
        to: 400
        stepSize: 10
        value: Math.round((controls.film.info?.rate ?? 1) * 100)
        editable: true
        enabled: !controls.film.info?.keyed
        textFromValue: v => (v / 100).toFixed(2) + "×"
        valueFromText: text => Math.round(Number(text.replace("×", "").replace(",", ".")) * 100)
        onValueModified: controls.film.page.run("film", {
                "rate": {
                    "layer": controls.film.node.id,
                    "rate": value / 100
                }
            })
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.text: App.tr("A video plays from 0 to 4 times its speed")
    }

    QQC2.CheckBox {
        text: App.tr("Loop")
        checked: controls.film.info?.loop ?? true
        onToggled: controls.film.page.change(controls.film.info.pointer + "/loop", checked)
    }

    QQC2.Button {
        text: App.tr("Use the size of the video")
        icon.name: "zoom-fit-best"
        enabled: controls.film.info?.size !== null
        onClicked: controls.film.page.run("film", {
                "fit": {
                    "layer": controls.film.node.id
                }
            })
    }

    QQC2.Button {
        text: App.tr("Play the whole video")
        icon.name: "media-skip-backward"
        onClicked: controls.film.trim(0, null)
    }
}
