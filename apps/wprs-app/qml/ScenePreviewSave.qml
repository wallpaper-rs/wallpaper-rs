import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: save

    required property var page
    property int frames: 1
    property string result: ""

    function spec() {
        const wanted = {
            "quality": SceneLook.quality,
            "battery": SceneLook.battery,
            "calm": SceneLook.calm,
            "language": SceneLook.language,
            "time": save.page.editor.time,
            "frames": save.frames,
            "fps": 30
        };
        if (clock.checked)
            wanted.time_of_day = hour.value * 3600 + minute.value * 60;
        if (SceneLook.media !== null)
            wanted.media = SceneLook.media;
        if (SceneLook.values !== null)
            wanted.values = SceneLook.values;
        if (SceneLook.preset !== "")
            wanted.preset = SceneLook.preset;
        if (sweep.checked)
            wanted.pointer = {
                "from": [0.2, 0.5],
                "to": [0.8, 0.5],
                "click": true
            };
        if (split.checked) {
            wanted.against = SceneLook.versus;
            wanted.without_bloom = SceneLook.versusGlow;
        }
        return JSON.stringify(wanted);
    }

    QQC2.CheckBox {
        id: clock

        text: App.tr("Set the time the clock shows")
    }

    RowLayout {
        enabled: clock.checked

        QQC2.SpinBox {
            id: hour

            from: 0
            to: 23
            value: 12
            textFromValue: value => String(value).padStart(2, "0")
            QQC2.ToolTip.text: App.tr("Hour")
            QQC2.ToolTip.visible: hovered
            Accessible.name: App.tr("Hour")
        }

        QQC2.Label {
            text: ":"
        }

        QQC2.SpinBox {
            id: minute

            from: 0
            to: 59
            value: 0
            textFromValue: value => String(value).padStart(2, "0")
            QQC2.ToolTip.text: App.tr("Minute")
            QQC2.ToolTip.visible: hovered
            Accessible.name: App.tr("Minute")
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: SceneLook.media !== null
        text: App.tr("The picture shows the simulated media player of the preview.")
    }

    QQC2.CheckBox {
        id: sweep

        text: App.tr("Move a pointer across and click")
    }

    QQC2.CheckBox {
        id: split

        text: App.tr("Split the picture with the other side")
    }

    RowLayout {
        QQC2.Button {
            icon.name: "camera-photo"
            text: App.tr("Save a picture…")
            enabled: !save.page.editor.busy
            onClicked: {
                save.frames = 1;
                files.open();
            }
        }

        QQC2.Button {
            icon.name: "video-x-generic"
            text: App.tr("Save a clip…")
            enabled: !save.page.editor.busy
            onClicked: {
                save.frames = 90;
                files.open();
            }
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: save.result !== ""
        text: App.tr("Saved: %1", save.result)
    }

    FileDialog {
        id: files

        fileMode: FileDialog.SaveFile
        nameFilters: save.frames > 1 ? [App.tr("An animated picture") + " (*.gif)", App.tr("A picture; a clip becomes an image sequence") + " (*.png *.jpg *.webp)"] : [App.tr("A picture") + " (*.png *.jpg *.webp)"]
        onAccepted: save.page.editor.render(App.path(selectedFile), save.spec())
    }

    Connections {
        target: save.page.editor

        function onRendered(files) {
            save.result = files.split("\n")[0];
        }
    }
}
