import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: middle

    required property var page
    readonly property string kind: page.kind
    readonly property var names: ({
            "web": ["settings", "properties", "preview"],
            "video": ["settings", "presets", "preview"],
            "image": ["settings", "preview"]
        })[kind] ?? ["settings"]
    readonly property var titles: ({
            "settings": App.tr("Settings"),
            "properties": App.tr("Options"),
            "presets": App.tr("Presets"),
            "preview": App.tr("See as"),
            "try": App.tr("Try")
        })
    readonly property string current: names.concat(["try"])[tabs.currentIndex] ?? "settings"

    spacing: 0

    QQC2.TabBar {
        id: tabs

        Layout.fillWidth: true
        visible: middle.names.length > 1

        Repeater {
            model: middle.names.concat(["try"])

            delegate: QQC2.TabButton {
                required property string modelData

                text: middle.titles[modelData]
            }
        }
    }

    StackLayout {
        Layout.fillWidth: true
        Layout.fillHeight: true
        currentIndex: ["settings", "properties", "presets", "preview", "try"].indexOf(middle.current)

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

        PackagePresets {
            page: middle.page
        }

        PackagePreview {
            page: middle.page
        }

        SceneTry {
            page: middle.page
        }
    }
}
