import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function same(found, wanted, what) {
        if (found !== wanted)
            throw new Error(what + ": " + JSON.stringify(found) + " instead of " + JSON.stringify(wanted));
    }

    function open(place) {
        page.query = "";
        page.place = "";
        page.tab = 2;
        if (place !== "")
            page.open(place);
        return place === "" ? page.tabs[2] : page.shown.item;
    }

    function row(item, text) {
        const found = SettingsReach.rows(item).find(fila => fila.text === text);
        if (!found)
            throw new Error("no row “" + text + "” among " + JSON.stringify(SettingsReach.rows(item).map(fila => fila.text)));
        return found;
    }

    function equipo() {
        const tab = open("");
        same(JSON.stringify(SettingsReach.rows(tab).map(fila => fila.text)), JSON.stringify([App.tr("Scene permissions"), App.tr("Sound of the scenes"), App.tr("What you are listening to"), App.tr("RGB lights"), App.tr("Advanced")]), "the rows of the device tab");
        same(MachineWords.lights(true, {
            "connected": false,
            "devices": []
        }), App.trc("lights", "not detected"), "lights that are not found");
        same(MachineWords.lightsFound(true, {
            "connected": false
        }), false, "lights not detected are gray");
        same(MachineWords.lightsFound(false, {}), true, "lights turned off can be turned on");
        same(MachineWords.sound({
            "playback.scene_sound": true,
            "playback.scene_capture": true
        }), App.tr("They play and react to sound"), "the sound summary");
    }

    function avanzado() {
        const part = open("advanced");
        same(JSON.stringify(page.route), JSON.stringify([App.tr("Device"), App.tr("Advanced")]), "the route of Advanced");
        row(part, App.tr("Graphics card"));
        same(row(part, App.tr("HDR, extra bright colors")).off, !MachineWords.hdrSeen(App.state.outputs) && String((App.state.options ?? {})["playback.hdr"] ?? "auto") !== "off", "HDR without a screen that supports it");
        same(MachineWords.hdrSeen({
            "DP-1": {
                "hdr": {
                    "reason": "display"
                }
            }
        }), false, "a screen without HDR");
        row(part, App.tr("If something does not show"));
        page.open("depth");
        same(JSON.stringify(page.route), JSON.stringify([App.tr("Device"), App.tr("Advanced"), App.tr("Depth quality")]), "the route under Advanced");
        page.routeBack();
        same(page.place, "advanced", "back from depth goes to Advanced");
        page.routeBack();
        same(page.place, "", "back from Advanced goes to the tab");
    }

    function permisos() {
        const kept = page.inputs;
        page.inputs = {
            "grants": {
                "pond": ["pointer", "audio"]
            },
            "tracking": true
        };
        same(open("").rows[0].value, App.tr("1 scene"), "the scenes with permissions");
        const part = open("grants");
        same(row(part, App.tr("What scenes may use")).status, App.tr("A wallpaper is following the pointer now."), "what follows now");
        same(SettingsReach.rows(part).filter(fila => fila.text === "pond").length, 2, "one row per permission");
        page.inputs = kept;
    }

    function partes() {
        row(open("sound"), App.tr("Let scenes react to the sound that plays"));
        row(open("media"), App.tr("Fetch covers from the internet"));
        row(open("lights"), App.tr("OpenRGB server"));
        row(open("depth"), App.tr("Depth layers"));
        row(open("version"), App.tr("Configuration"));
        row(open("testing"), App.tr("Surface name"));
    }

    function buscar() {
        page.query = "OpenRGB";
        if (!page.found.hits.some(hit => hit.tab === 2 && hit.place === "lights"))
            throw new Error("the lights page is not found by its server");
        page.query = "";
    }

    function exercise() {
        equipo();
        avanzado();
        permisos();
        partes();
        buscar();
        destroy();
    }
}
