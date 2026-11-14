import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: compare

    required property var page

    RowLayout {
        QQC2.Label {
            text: App.tr("Compare with")
        }

        QQC2.ComboBox {
            Layout.fillWidth: true
            model: SceneLook.names.map((name, i) => name + " · Q" + i)
            currentIndex: SceneLook.levels.indexOf(SceneLook.versus)
            onActivated: index => SceneLook.versus = SceneLook.levels[index]
        }
    }

    QQC2.CheckBox {
        text: App.tr("Without the glow of the canvas")
        checked: SceneLook.versusGlow
        onToggled: SceneLook.versusGlow = checked
    }

    QQC2.Button {
        Layout.fillWidth: true
        icon.name: "view-split-left-right"
        text: App.tr("Hold to see the other side")
        onPressed: {
            SceneLook.rival = true;
            SceneLook.preview(compare.page);
        }
        onReleased: {
            SceneLook.rival = false;
            SceneLook.preview(compare.page);
        }
        onCanceled: {
            SceneLook.rival = false;
            SceneLook.preview(compare.page);
        }
    }
}
