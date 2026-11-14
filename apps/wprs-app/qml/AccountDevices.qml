import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: devices

    property var page: null
    property var device: ({})
    property var choices: []
    readonly property var account: page ? page.account : null
    readonly property var items: account ? account.devices : []
    readonly property alias menu: menu

    function reachable(device) {
        return !device.current && !!device.seen_at;
    }

    function offers(device) {
        const out = [];
        if (page.keyed && reachable(device))
            out.push({
                "text": App.tr("Let it sync…"),
                "icon": "share-network",
                "act": "share"
            });
        if (page.phase === "needs_key" && reachable(device))
            out.push({
                "text": App.tr("Get the key from it"),
                "icon": "key",
                "act": "join"
            });
        if (!device.current)
            out.push({
                "text": App.tr("Remove device…"),
                "icon": "trash",
                "tone": "coral",
                "act": "revoke"
            });
        return out;
    }

    function note(device) {
        const seen = device.current ? App.tr("this computer") : device.seen_at ? App.tr("seen %1", App.time(device.seen_at)) : App.tr("never connected");
        return [device.label ? device.name : "", seen, App.tr("added %1", App.time(device.created_at) || "–")].filter(t => t).join(" · ");
    }

    function choose(row, device) {
        devices.device = device;
        devices.choices = offers(device);
        menu.opener = row;
        menu.open();
    }

    function act(index) {
        const which = choices[index] ? choices[index].act : "";
        if (which === "share")
            page.codes.share(device, menu.opener);
        else if (which === "join")
            App.act("Join", [device.id], "Sync1");
        else if (which === "revoke")
            page.revoker.ask(device, menu.opener);
    }

    function enter() {
        const first = rows.itemAt(0);
        if (first)
            first.forceActiveFocus(Qt.TabFocusReason);
    }

    width: parent ? parent.width : 0
    height: list.height

    Filas {
        id: list

        Repeater {
            id: rows

            model: devices.items

            delegate: Fila {
                id: row

                required property var modelData

                text: modelData.label || modelData.name
                note: devices.note(modelData)
                kind: modelData.current ? "texto" : "pliega"
                value: modelData.current ? App.tr("this computer") : ""
                explain: modelData.current || modelData.seen_at ? "" : App.tr("This device never finished connecting, so it cannot receive the key. Remove it if you no longer use it.")
                onOpened: devices.choose(row, modelData)
            }
        }
    }

    ListaMenu {
        id: menu

        options: devices.choices.map(c => ({
                    "text": c.text,
                    "icon": c.icon,
                    "tone": c.tone || ""
                }))
        onChosen: index => devices.act(index)
    }
}
