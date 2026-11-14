pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function ago(seconds) {
        if (!seconds)
            return App.tr("never synced");
        const minutes = Math.max(0, Math.floor((Date.now() / 1000 - seconds) / 60));
        if (minutes < 1)
            return App.tr("synced just now");
        if (minutes < 60)
            return App.tr("synced %1 min ago", minutes);
        if (minutes < 1440)
            return App.tr("synced %1 h ago", Math.floor(minutes / 60));
        return App.tr("synced %1", App.time(seconds));
    }

    function line(account, code, address) {
        if (!account)
            return "";
        const name = account.name || account.url;
        const where = App.tr("Connected to %1", name);
        switch (account.state) {
        case "unlinked":
            return App.tr("Server added · this computer is not connected yet");
        case "linking":
            return code ? App.tr("Open %1 and enter the code %2", address, code) : App.tr("Waiting for the approval of the code");
        case "connected":
            return App.tr("Connected · not syncing");
        case "needs_key":
            return [where, App.tr("waiting for the key")].join(" · ");
        case "paused":
            return [where, App.tr("sync paused")].join(" · ");
        case "error":
            return [where, App.tr("the last sync failed")].join(" · ");
        case "signed_out":
            return App.tr("This computer can no longer sync");
        }
        return [where, ago(account.last_sync), account.encrypted === false ? App.tr("not encrypted") : App.tr("encrypted")].join(" · ");
    }

    function tone(account) {
        const state = account ? account.state : "";
        if (state === "ready")
            return account.encrypted === false ? "ambar" : "brote";
        if (["error", "signed_out"].includes(state))
            return "coral";
        if (["needs_key", "paused", "linking"].includes(state))
            return "ambar";
        return "";
    }

    function explain(account, off) {
        if (off)
            return App.tr("Connect this computer to the server first. Connecting alone sends nothing of your library.");
        if (account && account.state === "connected")
            return App.tr("Connected, not syncing: you browse and install from the server, and nothing of your library leaves this computer. Turn it on to keep your devices the same.");
        if (account && account.encrypted === false)
            return App.tr("Sync is not encrypted: the operator of the server can read your library, playlists and settings.");
        return App.tr("Your devices keep the same things, encrypted: the server cannot read them. Pausing deletes nothing; resume whenever you like.");
    }
}
