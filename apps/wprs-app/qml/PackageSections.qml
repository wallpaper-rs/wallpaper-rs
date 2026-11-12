import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: sections

    required property var page
    readonly property var pages: (page.view.files ?? []).filter(file => file.kind === "page").map(file => file.path)
    readonly property bool web: page.kind === "web"
    readonly property bool preset: page.kind === "preset"

    spacing: Kirigami.Units.largeSpacing

    Kirigami.FormLayout {
        Layout.fillWidth: true

        QQC2.Label {
            Kirigami.FormData.label: App.tr("Kind:")
            text: Labels.value(sections.page.kind)
        }

        QQC2.ComboBox {
            Kirigami.FormData.label: App.tr("Entry page:")
            visible: sections.web
            model: sections.pages
            currentIndex: sections.pages.indexOf(sections.page.manifest.entry ?? "")
            onActivated: index => sections.page.change("/manifest/entry", sections.pages[index])
        }

        QQC2.Button {
            Kirigami.FormData.label: App.tr("Package:")
            text: App.tr("Title, license, origin and more…")
            icon.name: "documentinfo"
            onClicked: sections.page.editor.select("/manifest")
        }
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: sections.web
        level: 4
        text: App.tr("Page")
    }

    PackageWeb {
        Layout.fillWidth: true
        visible: sections.web
        page: sections.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: sections.web
        level: 4
        text: App.tr("Network")
    }

    PackageNetwork {
        Layout.fillWidth: true
        visible: sections.web
        page: sections.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: sections.web
        level: 4
        text: App.tr("Inputs")
    }

    PackageInputs {
        Layout.fillWidth: true
        visible: sections.web
        page: sections.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: sections.web
        level: 4
        text: App.tr("Playback")
    }

    PackagePlayback {
        Layout.fillWidth: true
        visible: sections.web
        page: sections.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: sections.preset
        level: 4
        text: App.tr("Preset")
    }

    PackagePreset {
        Layout.fillWidth: true
        visible: sections.preset
        page: sections.page
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        visible: !sections.preset
        level: 4
        text: App.tr("Required packages")
    }

    PackageRequires {
        Layout.fillWidth: true
        visible: !sections.preset
        page: sections.page
    }
}
