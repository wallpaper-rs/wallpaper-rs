import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var phrases: ({
            "audio": App.tr("asks for the microphone"),
            "pointer": App.tr("follows the pointer"),
            "clicks": App.tr("answers clicks"),
            "network": App.tr("uses the network"),
            "rgb": App.tr("lights up your devices"),
            "media": App.tr("reads what is playing")
        })
    readonly property var icons: ({
            "audio": "microphone",
            "pointer": "cursor",
            "clicks": "cursor-click",
            "network": "globe",
            "rgb": "lightbulb",
            "media": "music-note"
        })

    function asks(pkg) {
        return (pkg.asks ?? []).filter(a => phrases[a] !== undefined);
    }

    function newest(pkg) {
        const versions = pkg.versions ?? [];
        return versions.length > 0 ? versions[versions.length - 1] : pkg.version;
    }

    function here(pkg) {
        return (pkg.installed ?? []).includes(newest(pkg));
    }

    function older(pkg) {
        return (pkg.installed ?? []).length > 0 && !here(pkg) && !pkg.update && !pkg.own;
    }

    function author(pkg) {
        return (pkg.authors ?? []).length > 0 ? pkg.authors.join(", ") : pkg.author ?? "";
    }

    function caption(pkg) {
        const kind = pkg.kind ? Labels.value(pkg.kind) : "";
        if (pkg.own && pkg.own.update)
            return [kind, author(pkg), App.tr("you have %1, there is %2", pkg.own.version, pkg.own.update)].filter(t => t).join(" · ");
        if (pkg.own)
            return [author(pkg), pkg.own.version, pkg.repo_name ?? ""].filter(t => t).join(" · ");
        if (older(pkg))
            return [kind, author(pkg), App.tr("you have %1", pkg.installed[pkg.installed.length - 1])].filter(t => t).join(" · ");
        const wanted = asks(pkg);
        return [kind, author(pkg), App.bytes(pkg.size), wanted.length > 0 ? phrases[wanted[0]] : ""].filter(t => t).join(" · ");
    }

    function of(pkg) {
        if (pkg.own && pkg.own.revoked)
            return [App.tr("Version revoked"), "warning-circle", "coral"];
        if (pkg.own && (pkg.own.asks ?? []).length > 0)
            return [App.tr("The new version also %1", pkg.own.asks.map(a => phrases[a] ?? a).join(", ")), "warning", "ambar"];
        if (pkg.photosensitive === true && pkg.flash_disabled)
            return [App.tr("Has flashes, disabled"), "warning", "ambar"];
        if ((pkg.installed ?? []).length > 0 && !pkg.own)
            return [App.tr("Installed"), "check-circle", "brote"];
        const wanted = asks(pkg);
        if (wanted.includes("network"))
            return [App.tr("Uses the network"), "globe", ""];
        return [[author(pkg), App.bytes(pkg.size)].filter(t => t).join(" · "), "", ""];
    }
}
