import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: middle

    required property var page

    spacing: 0

    QQC2.TabBar {
        id: tabs

        Layout.fillWidth: true
        visible: middle.page.kind === "web"

        QQC2.TabButton {
            text: App.tr("Settings")
        }

        QQC2.TabButton {
            text: App.tr("Properties")
        }

        QQC2.TabButton {
            text: App.tr("Preview")
        }
    }

    StackLayout {
        Layout.fillWidth: true
        Layout.fillHeight: true
        currentIndex: middle.page.kind === "web" ? tabs.currentIndex : 0

        QQC2.ScrollView {
            contentWidth: availableWidth
            clip: true

            PackageSections {
                width: parent.width
                page: middle.page
            }
        }

        SceneProperties {
            page: middle.page
        }

        PagePreview {
            page: middle.page
        }
    }
}
