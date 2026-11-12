import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function find(item, test) {
        if (test(item))
            return item;
        const kids = Array.from(item.children ?? []);
        if (item.contentItem)
            kids.push(item.contentItem);
        for (const kid of kids) {
            const found = find(kid, test);
            if (found)
                return found;
        }
        return null;
    }

    function expect(ok, message) {
        if (!ok)
            throw new Error("media: " + message);
    }

    function tracks() {
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
        if (seen[1] !== SceneMediaWords.tracks[1] || seen[count] !== SceneMediaWords.tracks[0] || new Set(seen.slice(1)).size !== count)
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

    function duration() {
        const player = Qt.createComponent("SceneMedia.qml").createObject(page, {
                "page": page
            });
        expect(player !== null, "the player tab did not load");
        SceneLook.media = Object.assign({}, SceneLook.sample);
        player.change("duration", 3700);
        expect(SceneLook.media.duration === 3700, "the duration slider value was not kept");
        const spin = find(player, item => item.editable !== undefined && item.valueModified !== undefined);
        expect(spin !== null, "the duration has no seconds field");
        expect(spin.value === 3700, "the seconds field does not show the duration");
        spin.value = 7200;
        spin.valueModified();
        expect(SceneLook.media.duration === 7200, "typing seconds did not set the duration");
        expect(SceneMediaWords.clock(7200) === "2:00:00" && SceneMediaWords.clock(75) === "1:15" && SceneMediaWords.clock(-3) === "0:00", "the clock reads " + SceneMediaWords.clock(7200) + " " + SceneMediaWords.clock(75));
        const before = SceneLook.media.title;
        player.skip();
        expect(SceneLook.media.title !== before && SceneLook.media.position === 0, "Next track did not change the track");
        SceneLook.media = null;
        player.destroy();
    }

    function covers() {
        const input = Qt.createQmlObject("import QtQuick; QtObject { property var page; property var value: ''; property var field: ({pointer: '/scene/layers/0/image'}) }", page);
        input.page = page;
        const pick = Qt.createComponent("SceneCoverPick.qml").createObject(page, {
                "input": input
            });
        expect(pick !== null, "the cover buttons did not load");
        const current = find(pick, item => item.text !== undefined && item.clicked !== undefined && item.icon !== undefined && item.icon.name === "media-playback-start");
        expect(current !== null, "there is no button for the cover of what plays");
        const available = pick.pictures.length > 0 || pick.assets.some(a => a.media === "current");
        expect(current.enabled === available, "the cover button is " + (current.enabled ? "on" : "off") + " with " + pick.pictures.length + " pictures");
        if (available) {
            current.clicked();
            expect(page.editor.error === "", "the cover button was refused: " + page.editor.error);
        }
        pick.destroy();
        input.destroy();
    }

    function exercise() {
        tracks();
        duration();
        covers();
    }
}
