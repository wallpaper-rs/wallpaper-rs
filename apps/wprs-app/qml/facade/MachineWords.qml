pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var fills: ({
            "lama": App.tr("Smart fill"),
            "simple": App.tr("Simple fill"),
            "off": App.tr("No fill")
        })

    function place(where) {
        return ({
                "grants": App.tr("Scene permissions"),
                "sound": App.tr("Sound of the scenes"),
                "media": App.tr("What you are listening to"),
                "lights": App.tr("RGB lights"),
                "advanced": App.tr("Advanced"),
                "depth": App.tr("Depth quality"),
                "version": App.tr("Version %1", App.state.version ?? ""),
                "testing": App.tr("For tests")
            })[where] ?? "";
    }

    function sound(options) {
        const plays = options["playback.scene_sound"] !== false;
        const hears = options["playback.scene_capture"] !== false;
        if (plays && hears)
            return App.tr("They play and react to sound");
        if (plays)
            return App.tr("They play their sound");
        return hears ? App.tr("They react to sound") : App.tr("Silent");
    }

    function depth(options) {
        const fill = String(options["parallax.fill"] ?? "lama");
        return App.tr("%1, %2 layers", fills[fill] ?? fill, Number(options["parallax.layers"] ?? 5));
    }

    function scenes(count) {
        return count === 1 ? App.tr("1 scene") : App.tr("%1 scenes", count);
    }

    function following(inputs) {
        return [inputs.tracking ? App.tr("A wallpaper is following the pointer now.") : "", inputs.capturing ? App.tr("A wallpaper is following the sound now.") : ""].filter(t => t).join(" ");
    }

    function hdrSeen(outputs) {
        return Object.values(outputs ?? {}).some(info => info && info.hdr && info.hdr.reason !== "display");
    }

    function lightsFound(on, status) {
        return !on || status.connected === true || (status.devices ?? []).length > 0 || status.connected === undefined;
    }

    function lights(on, status) {
        if (!on)
            return App.trc("lights", "Off");
        if (!lightsFound(on, status))
            return App.trc("lights", "not detected");
        const count = (status.devices ?? []).length;
        return count === 1 ? App.tr("1 device") : App.tr("%1 devices", count);
    }
}
