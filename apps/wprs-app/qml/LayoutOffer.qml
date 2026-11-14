import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: offer

    required property var host
    readonly property string seen: App.outputs.slice().sort().join(",")
    property string known: ""

    function same(profile, names) {
        const kept = ((profile.auto ?? {}).outputs ?? []).slice().sort().join(",");
        return kept === names;
    }

    function save(names) {
        const list = names.split(",");
        App.act("Save", [list.join(" + "), false], "Profiles1", reply => offer && App.act("SetAuto", [reply[0], "outputs", names], "Profiles1"));
    }

    function message(names, profiles) {
        if (profiles.some(profile => same(profile, names)))
            return "";
        return App.tr("Screens now: %1. Remember this layout?", names.split(",").join(" + "));
    }

    function show(text, names) {
        host.showPassiveNotification(text, 15000, App.tr("Save"), () => offer.save(names));
    }

    function check(names) {
        App.query("List", [], "Profiles1", reply => {
            if (!offer || names !== offer.seen)
                return;
            const text = offer.message(names, reply.profiles);
            if (text !== "")
                offer.show(text, names);
        });
    }

    onSeenChanged: {
        const before = known;
        known = seen;
        if (before !== "" && seen !== "")
            check(seen);
    }
    Component.onCompleted: known = seen
}
