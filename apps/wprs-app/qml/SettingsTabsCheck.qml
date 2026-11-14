import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    readonly property var places: ["more", "folders", "lists", "pause", "screen", "layouts"]

    function same(found, wanted, what) {
        if (found !== wanted)
            throw new Error(what + ": " + JSON.stringify(found) + " instead of " + JSON.stringify(wanted));
    }

    function home(at) {
        page.query = "";
        page.place = "";
        page.tab = at;
    }

    function texts(item) {
        return SettingsReach.rows(item).map(row => row.text);
    }

    function has(item, text, what) {
        const row = SettingsReach.rows(item).find(fila => fila.text === text);
        if (!row)
            throw new Error(what + ": no row “" + text + "” among " + JSON.stringify(texts(item)));
        return row;
    }

    function fondos() {
        home(0);
        for (const text of [App.tr("Mature content"), App.tr("Flashes"), App.tr("Library folders"), App.tr("Lists per screen"), App.tr("More")])
            has(page.tabs[0], text, "the wallpapers tab");
        same(has(page.tabs[0], App.tr("Flashes"), "flashes").options.length, 2, "the flashes choice");
    }

    function mas() {
        home(0);
        page.open("more");
        same(page.route.length, 2, "the route of More");
        has(page.shown.item, App.tr("Keyboard shortcuts"), "More");
        same(SettingsFolder.joined(SettingsFolder.keys("LOGO+ALT+N")), "LOGO+ALT+N", "a shortcut read and written back");
    }

    function pantallas() {
        home(1);
        has(page.tabs[1], App.tr("Screen you adjust"), "two screens");
        has(page.tabs[1], App.tr("Transition on every screen"), "the general transition");
        has(page.tabs[1], App.tr("Less motion"), "less motion");
        const words = SettingsWords.transition(true, "DP-1", {
            "transition": {
                "own": "fade",
                "why": "reduced"
            }
        }, {});
        same(words.values[words.current], "fade", "the own transition of a screen");
        same(words.why, App.tr("Last change: a plain fade, because of less motion."), "the reason of the last change");
        const fixed = SettingsWords.quality(false, "DP-1", {
            "quality_auto": "lower"
        }, {
            "playback.quality": "q2"
        });
        same(fixed.words[fixed.current], App.tr("Always high"), "a level from More shown in the row");
        same(fixed.note, App.tr("You set it: it stays on battery") + App.tr(", even though the automatic one would lower it now"), "a fixed level wins");
        page.open("pause");
        const battery = has(page.shown.item, App.tr("Pause on battery"), "pause");
        same(battery.off, !(App.state.on_battery === true || App.state.battery_percent !== null && App.state.battery_percent !== undefined), "a battery that is not detected");
    }

    function disposicion() {
        home(1);
        page.open("layouts");
        same(has(page.shown.item, "Desk", "the layouts").value, App.tr("In use"), "the layout in use");
        const offer = Qt.createComponent("LayoutOffer.qml").createObject(check, {
            "host": page
        });
        same(offer.message("DP-1,HDMI-A-1", page.profiles), "", "screens a layout already keeps");
        const told = offer.message("DP-1", page.profiles);
        same(told, App.tr("Screens now: %1. Remember this layout?", "DP-1"), "a screen removed");
        applicationWindow().showPassiveNotification(told, 15000, App.tr("Save"), () => {});
        offer.destroy();
    }

    function buscar() {
        home(0);
        page.query = App.tr("Screens").toUpperCase();
        if (page.found.hits.length < 1 || page.found.hits.some(hit => !SettingsReach.matches(hit.text, page.query)))
            throw new Error("searching ignores case: " + JSON.stringify(page.found.hits.map(hit => hit.text)));
        same(SettingsReach.matches("Pantallas", "PANTÁLLAS"), true, "searching ignores accents");
        page.query = App.tr("battery");
        if (!page.found.hits.some(hit => hit.tab === 1))
            throw new Error("battery is not found in the screens tab");
    }

    function vacio() {
        home(0);
        page.query = "tema";
        same(page.found.hits.length, 0, "a word no setting has");
        same(page.found.tried.visible, true, "the words to try");
        page.found.tried.picked(0);
        same(page.query, page.found.tries[0], "a word tried");
    }

    function error() {
        const kept = page.sources;
        page.sources = kept.concat([{
                "uid": "lost",
                "path": "photos/old",
                "error": "missing",
                "items": 3
            }]);
        home(0);
        same(has(page.tabs[0], App.tr("Library folders"), "folders").note, App.tr("%1 cannot be found", "old"), "a folder that is lost");
        page.open("folders");
        same(has(page.shown.item, "old", "the lost folder").status, App.tr("This folder cannot be found"), "the lost folder row");
        page.sources = kept;
    }

    function labels() {
        const named = page.entries.map(entry => entry.text);
        const lacking = [];
        const look = item => SettingsReach.all(item).filter(row => row.objectName !== "dynamic" && !named.includes(row.text)).forEach(row => lacking.push(row.text));
        for (let at = 0; at < page.tabs.length; at++) {
            home(at);
            look(page.tabs[at]);
        }
        for (const place of places) {
            home(SettingsPlaces.tab(place));
            page.open(place);
            look(page.shown.item);
        }
        same(JSON.stringify(lacking), "[]", "rows the search cannot find");
        same(named.every(text => page.entries.some(entry => SettingsReach.matches(entry.text, text))), true, "every label found by itself");
    }

    function exercise() {
        const tab = page.tab;
        const place = page.place;
        fondos();
        mas();
        pantallas();
        disposicion();
        buscar();
        vacio();
        error();
        labels();
        home(tab);
        page.place = place;
    }
}
