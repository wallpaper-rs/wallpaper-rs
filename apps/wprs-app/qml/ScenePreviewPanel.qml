import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: panel

    required property var page

    spacing: Kirigami.Units.smallSpacing

    SceneQuality {
        page: panel.page
    }

    QQC2.CheckBox {
        text: App.tr("On battery")
        checked: SceneLook.battery
        onToggled: {
            SceneLook.battery = checked;
            SceneLook.preview(panel.page);
        }
    }

    QQC2.CheckBox {
        text: App.tr("Reduced motion")
        checked: SceneLook.calm
        onToggled: {
            SceneLook.calm = checked;
            SceneLook.preview(panel.page);
        }
    }

    ScenePreviewEffects {
        Layout.fillWidth: true
        page: panel.page
    }

    Kirigami.Separator {
        Layout.fillWidth: true
    }

    ScenePreviewSave {
        Layout.fillWidth: true
        page: panel.page
    }
}
