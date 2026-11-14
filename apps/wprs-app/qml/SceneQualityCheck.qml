import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function exercise() {
        const row = Qt.createComponent("SceneQualityRow.qml").createObject(page, {
            "uid": "pond",
            "names": ["DP-1", "DP-2"],
            "outputs": {
                "DP-2": {
                    "item": "pond"
                }
            },
            "settings": {
                "playback.quality": "auto",
                "playback.quality.DP-2": "q3"
            }
        });
        must(row !== null, "the piece SceneQualityRow.qml does not load");
        const fixed = [row.visible, row.screen, row.level, row.value];
        row.settings = {};
        const general = row.note;
        row.outputs = {};
        const shown = row.visible;
        row.destroy();
        must(fixed.join("|") === ["true", "DP-2", "q3", App.tr("Like your screen")].join("|"), "the quality of this scene does not show the level of its screen: " + fixed.join("|"));
        must(general === App.tr("Your screen's: %1, in Settings › Screens", App.tr("Automatic quality")) && !shown, "the quality of this scene does not fall back to the general level or shows without a screen: " + general);
    }
}
