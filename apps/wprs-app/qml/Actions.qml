import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: tools

    required property ItemModel items
    property var page
    property string target
    property var uids: []
    property var playlists: []
    property string applied
    readonly property alias dialogs: dialogs
    readonly property bool several: App.outputs.length > 1

    function folder(uid) {
        App.query("GetItem", [uid], "Library1", reply => {
            const path = String(reply.item ? reply.item.path ?? "" : "");
            if (tools && path !== "")
                Qt.openUrlExternally("file://" + path.substring(0, path.lastIndexOf("/")));
        });
    }

    function take() {
        uids = JSON.parse(items.selection());
        return uids;
    }

    function chosen(open, from) {
        take();
        dialogs[open].opener = from;
        dialogs[open].open();
    }

    function put(uid) {
        if (!uid)
            return;
        const output = several ? target : "";
        const spot = output !== "" ? output : App.outputs.length > 0 ? App.outputs[0] : "";
        const screens = App.state.outputs ?? {};
        const rotating = screens[spot] ? screens[spot].playlist ?? null : null;
        App.query("GetItem", [uid], "Library1", reply => {
            if (!tools)
                return;
            const spread = several && !!(reply.about && reply.about.spread);
            App.act("Apply", [uid, spread ? "" : output, spread ? "span" : "", ""], "Library1", () => {
                if (!tools)
                    return;
                tools.applied = uid;
                const title = reply.item ? reply.item.title ?? "" : "";
                if (rotating)
                    return applicationWindow().showPassiveNotification(App.tr("The rotation of “%1” stopped because you set “%2”.", rotating.name, title), "long", App.tr("Resume"), () => App.act("Start", [rotating.uid, spot], "Playlists1"));
                applicationWindow().showPassiveNotification(output !== "" && !spread ? App.tr("%1 is now the wallpaper of %2", title, output) : App.tr("%1 is now your wallpaper", title));
            });
        });
    }

    function favorite(on) {
        App.act("SetFavorite", [take(), on], "Library1");
    }

    function share() {
        App.act("ShareFiles", [take(), true], "Sync1", () => applicationWindow().showPassiveNotification(App.tr("Sending the files, encrypted, to your server. Your other computers can then download them from the Library or from Account.")), () => applicationWindow().showPassiveNotification(App.tr("The files were not sent. Link this computer to your server with end-to-end encryption in Account first.")));
    }

    function renameTag(name, from) {
        dialogs.renamer.from = name;
        dialogs.renamer.opener = from;
        dialogs.renamer.open();
    }

    function list(text) {
        return text.split(",").map(tag => tag.trim()).filter(tag => tag);
    }

    function shown() {
        const make = (id, kind) => Qt.createComponent("ItemSettings.qml").createObject(tools, {
                "host": {
                    "uid": id,
                    "pkg": {
                        "kind": kind
                    }
                }
            });
        const on = make("01J9Z0X9Z6Q7X4F3K5E1V0R1SC", "scene");
        const off = make("01J9Z0X9Z6Q7X4F3K5E1V0R1ZZ", "scene");
        const image = make("01J9Z0X9Z6Q7X4F3K5E1V0R1AB", "image");
        const found = [on.screens.join(), off.screens.length, on.visible, image.visible];
        [on, off, image].forEach(item => item.destroy());
        if (found.join() !== "HDMI-A-1,0,true,false")
            throw new Error("the options of a wallpaper do not offer its settings on a screen: " + found.join());
    }

    SelectionDialogs {
        id: dialogs

        owner: tools
    }
}
