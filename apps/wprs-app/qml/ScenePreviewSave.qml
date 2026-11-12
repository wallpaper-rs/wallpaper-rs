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
            "time": save.page.editor.time,
            "frames": save.frames,
            "fps": 30
        };
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
