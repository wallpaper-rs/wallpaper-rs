pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var asked: ["pointer", "audio", "clicks", "media", "media_network", "audio_network"]
    readonly property var icons: ({
            "pointer": "cursor",
            "audio": "speaker-high",
            "clicks": "cursor-click",
            "media": "play",
            "media_network": "globe",
            "audio_network": "globe"
        })

    function icon(input) {
        return icons[input] ?? "question";
    }

    function title(input) {
        switch (input) {
        case "pointer":
            return App.tr("The pointer");
        case "audio":
            return App.tr("The sound of the computer");
        case "clicks":
            return App.tr("Clicks and taps");
        case "media":
            return App.tr("Show what the media player plays");
        case "media_network":
            return App.tr("Show what plays even though it can reach the network");
        case "audio_network":
            return App.tr("Follow the sound even though it can reach the network");
        }
        return input;
    }

    function reason(input) {
        switch (input) {
        case "pointer":
            return App.tr("Only the latest pointer position over this screen reaches the wallpaper; positions are never kept and keys are never read.");
        case "audio":
            return App.tr("The sound is turned into levels on this computer while the wallpaper is shown; nothing is recorded, kept or sent, and the desktop may show a microphone indicator.");
        case "clicks":
            return App.tr("Only real clicks and taps on the wallpaper of this screen reach it; they are never kept and keys are never read.");
        case "media":
            return App.tr("While the wallpaper is shown, the title, artist, album, cover and progress of what a media player on this computer plays reach it; nothing of it is kept, logged or sent.");
        case "media_network":
            return App.tr("This web wallpaper may also reach the sites it names, so the title, artist and cover of what plays could leave this computer. Allow it only if you trust it with them.");
        case "audio_network":
            return App.tr("This web wallpaper may also reach the sites it names, so what it hears, the sound that plays or the microphone, could leave this computer. Allow it only if you trust it with that.");
        }
        return "";
    }

    function open(inputs) {
        const shown = inputs.shown ?? {};
        for (const output of Object.keys(shown)) {
            const s = shown[output];
            const list = (s.undecided ?? []).filter(input => asked.includes(input));
            if (list.length > 0)
                return {
                    "output": output,
                    "scene": s.scene,
                    "key": s.key ?? "",
                    "name": s.title || s.scene,
                    "inputs": list
                };
        }
        return null;
    }
}
