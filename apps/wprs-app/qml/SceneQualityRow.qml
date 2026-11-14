import QtQuick
import io.github.wallpaper_rs.app

Fila {
    id: quality

    required property string uid
    property var outputs: App.state.outputs ?? {}
    property var settings: App.state.options ?? {}
    property var names: App.outputs
    readonly property string screen: names.find(name => (quality.outputs[name] ?? {}).item === quality.uid) ?? ""
    readonly property string level: String(settings["playback.quality." + screen] ?? settings["playback.quality"] ?? "auto")

    function words(level) {
        const all = {
            "auto": App.tr("Automatic quality"),
            "min-q2": App.tr("Automatic, at least high"),
            "min-q1": App.tr("Automatic, at least reduced"),
            "q3": App.tr("Highest, always"),
            "q2": App.tr("High, always"),
            "q1": App.tr("Reduced, always"),
            "q0": App.tr("Lowest, always: saves battery")
        };
        return all[level] ?? level;
    }

    visible: screen !== ""
    kind: "pliega"
    text: App.tr("Quality of this scene")
    value: App.tr("Like your screen")
    note: App.tr("Your screen's: %1, in Settings › Screens", words(level))
    onOpened: applicationWindow().activate("settings")
}
