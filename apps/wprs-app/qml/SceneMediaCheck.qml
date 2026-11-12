import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        const player = Qt.createComponent("SceneMedia.qml").createObject(page, {
                "page": page
            });
        SceneLook.media = Object.assign({}, SceneLook.sample);
        const seen = [SceneLook.media.title];
        const count = SceneMediaWords.tracks.length;
        for (let i = 0; i < count; i++) {
            player.skip();
            seen[i + 1] = SceneLook.media.title;
        }
        if (seen[1] !== SceneMediaWords.tracks[1] || seen[count] !== SceneMediaWords.tracks[0] || new Set(seen).size !== count)
            throw new Error("media: Next track did not walk the titles: " + JSON.stringify(seen));
        if (SceneLook.media.position !== 0)
            throw new Error("media: Next track kept the position");
        player.change("duration", 14400);
        player.change("playing", false);
        if (SceneLook.media.duration !== 14400 || SceneLook.media.playing !== false || SceneMediaWords.clock(14400) !== "4:00:00")
            throw new Error("media: the made-up track did not change: " + JSON.stringify(SceneLook.media));
        if (SceneMediaWords.nextTitle("Open Road") !== SceneMediaWords.tracks[0] || SceneMediaWords.nextTitle("") !== SceneMediaWords.tracks[0])
            throw new Error("media: an unknown title did not start the list");
        SceneLook.media = null;
        SceneLook.preview(page);
        player.destroy();
    }
}
