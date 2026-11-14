import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

FocusScope {
    id: tab

    required property var page
    readonly property var placed: ["render.gpu", "surfaces.layer_namespace", "playback.quality", "playback.auto_pace", "playback.max_fps", "playback.lowest_below_percent", "playback.allow_flashing", "playback.hdr", "pause.on_battery", "pause.when_covered", "pause.when_locked", "rgb.enabled", "rgb.server", "rgb.when_covered", "media.remote_covers", "transition.kind", "transition.less_motion", "transition.duration_ms", "shortcuts.enabled", "shortcuts.next", "shortcuts.previous", "library.hide_mature", "library.show_mature", "library.watch", "library.thumb_cache_mb", "repos.auto_refresh", "repos.auto_update", "repos.vote_counts"]
    readonly property var fields: (App.schema.options ?? []).filter(field => !placed.includes(field.key))
    readonly property var rows: fields.map(field => ({
                "text": App.field(field.key)
            })).concat([App.tr("HDR (extra bright colors)"), App.tr("Draw with"), App.tr("RGB lighting"), App.tr("Surface name"), App.tr("Wallpaper permissions"), App.tr("Media player"), App.tr("Check the whole library again"), App.tr("Reload configuration")].map(text => ({
                "text": text
            })))

    implicitHeight: form.implicitHeight

    Kirigami.FormLayout {
        id: form

        width: tab.width

        Settings {
            daemon: App
            model: tab.fields
            values: App.state.options ?? {}
            onEdited: (key, value) => App.act("SetOption", [key, value])
        }

        FillNote {}

        HdrPicker {
            Kirigami.FormData.label: App.tr("HDR (extra bright colors)")
            daemon: App
        }

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
            Kirigami.FormData.label: App.tr("Graphics card")
        }

        GpuPicker {
            Kirigami.FormData.label: App.tr("Draw with")
        }

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
            Kirigami.FormData.label: App.tr("RGB lighting")
        }

        SettingsRgb {}

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
        }

        LayerNamespaceField {
            Kirigami.FormData.label: App.tr("Surface name")
        }

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
            Kirigami.FormData.label: App.tr("Wallpaper permissions")
        }

        SettingsGrants {
            inputs: tab.page.inputs
        }

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
            Kirigami.FormData.label: App.tr("Media player")
        }

        SettingsMedia {
            inputs: tab.page.inputs
        }

        Kirigami.Separator {
            Kirigami.FormData.isSection: true
        }

        QQC2.Label {
            Kirigami.FormData.label: App.tr("Version:")
            text: [App.state.version, App.state.adapter, App.tr("data model %1", App.state.model ?? "")].filter(part => !!part).join(" · ")
        }

        QQC2.Label {
            Kirigami.FormData.label: App.tr("Configuration:")
            visible: !!App.state.config_error || !!App.state.library_error
            text: [App.state.config_error, App.state.library_error].filter(part => !!part).join("\n")
            color: Look.coral
        }

        QQC2.Button {
            text: App.tr("Check the whole library again")
            onClicked: tab.page.sources.forEach(source => App.act("Rescan", [source.uid], "Library1"))
        }

        QQC2.Button {
            text: App.tr("Reload configuration")
            onClicked: App.act("Reload", [])
        }
    }
}
