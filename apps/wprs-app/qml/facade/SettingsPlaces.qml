pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var tabs: ({
            "more": 0,
            "folders": 0,
            "lists": 0,
            "pause": 1,
            "screen": 1,
            "layouts": 1,
            "grants": 2,
            "sound": 2,
            "media": 2,
            "lights": 2,
            "advanced": 2,
            "depth": 2,
            "version": 2,
            "testing": 2
        })
    readonly property var parents: ({
            "depth": "advanced",
            "version": "advanced",
            "testing": "advanced"
        })

    function tab(place) {
        return tabs[place] ?? 0;
    }

    function up(place) {
        return parents[place] ?? "";
    }

    function trail(place, output) {
        return (up(place) === "" ? [] : [word(up(place), output)]).concat([word(place, output)]);
    }

    function word(place, output) {
        const own = MachineWords.place(place);
        if (own !== "")
            return own;
        if (place === "more")
            return App.tr("More");
        if (place === "folders")
            return App.tr("Library folders");
        if (place === "lists")
            return App.outputs.length > 1 ? App.tr("Lists per screen") : App.tr("List that plays");
        if (place === "pause")
            return App.tr("Pause");
        if (place === "screen")
            return App.tr("More of %1", output);
        return App.tr("Layouts");
    }

    function pair(keys, text) {
        return {
            "keys": keys,
            "text": text
        };
    }

    function hints(searching, place, at) {
        const move = pair(["up", "down"], App.tr("Move around"));
        if (searching)
            return [move, pair(["Intro"], App.tr("Go to the setting")), pair(["Esc"], App.tr("Clear the search"))];
        if (place === "folders")
            return [move, pair(["Intro"], App.tr("Choose what to do")), pair(["Esc"], App.tr("Back"))];
        const change = pair(["left", "right"], App.tr("Change"));
        if (place !== "")
            return [move, change, pair(["Intro"], App.tr("Open")), pair(["Esc"], App.tr("Back"))];
        return [move, change, pair(["Intro"], at === 1 ? App.tr("More options") : App.tr("Open")), pair(["Tab"], App.tr("Tabs"))];
    }
}
