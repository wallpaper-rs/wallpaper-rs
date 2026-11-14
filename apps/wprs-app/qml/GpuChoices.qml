import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: picker

    readonly property var adapters: App.state.adapters ?? []
    readonly property string chosen: String((App.state.options ?? {})["render.gpu"] ?? "auto")
    readonly property var kinds: ({
            "integrated": App.tr("integrated"),
            "dedicated": App.tr("dedicated"),
            "virtual": App.tr("virtual"),
            "software": App.tr("drawn by the processor, slow"),
            "other": App.tr("other kind")
        })
    readonly property var words: ({
            "auto": App.tr("Automatic"),
            "integrated": App.tr("Any integrated card"),
            "dedicated": App.tr("Any dedicated card"),
            "software": App.tr("Drawn by the processor instead of a graphics card, for tests")
        })
    readonly property var choices: {
        const types = ["integrated", "dedicated"].filter(kind => adapters.some(a => a.kind === kind) || chosen === kind);
        const found = ["auto"].concat(types, chosen === "software" ? ["software"] : []).map(value => ({
                    "value": value,
                    "text": words[value]
                }));
        for (const card of adapters) {
            const twins = adapters.some(a => a.name === card.name && a.bus && card.bus && a.bus !== card.bus);
            const value = twins ? "pci:" + card.bus : card.name;
            if (card.kind === "software" || found.some(c => c.value === value))
                continue;
            const same = adapters.filter(a => a.name === card.name && (!twins || a.bus === card.bus));
            found.splice(found.length, 0, {
                "value": value,
                "text": twins ? App.tr("%1 · on PCI bus %2", label(card.name, same), card.bus) : label(card.name, same)
            });
        }
        if (!found.some(c => c.value.toLowerCase() === chosen.toLowerCase()))
            found.splice(found.length, 0, {
                "value": chosen,
                "text": chosen.toLowerCase().startsWith("pci:") ? App.tr("The card on PCI bus %1, which is not connected now", chosen.slice(4)) : adapters.some(a => a.name.toLowerCase().includes(chosen.toLowerCase())) ? App.tr("The first card whose name has “%1”", chosen) : App.tr("“%1”, which no card matches", chosen)
            });
        return found;
    }
    readonly property int current: Math.max(0, choices.findIndex(c => c.value.toLowerCase() === chosen.toLowerCase()))
    readonly property var used: adapters.find(a => a.used) ?? null
    readonly property string now: [used ? (used.kind === "integrated" ? App.tr("Now the integrated card draws") : used.kind === "dedicated" ? App.tr("Now the dedicated card draws") : App.tr("Now %1 draws", used.name)) : "", adapters.some(a => a.kind === "dedicated") ? "" : App.tr("dedicated card: not detected")].filter(t => t).join(" · ")
    readonly property string said: [App.state.gpu_restart === true ? App.tr("The new choice takes effect the next time the daemon starts.") : "", App.state.gpu_note ?? "", App.state.gpu_warning ?? ""].filter(t => t).join(" ")

    function backend(name) {
        return ({
                "vulkan": "Vulkan",
                "gl": "OpenGL ES"
            })[name] ?? name;
    }

    function label(name, same) {
        const used = same.some(a => a.used) ? App.tr("in use") : "";
        const desktop = same.some(a => a.compositor) ? App.tr("draws the desktop") : "";
        const ways = same.map(a => backend(a.backend)).join(", ");
        return [name, kinds[same[0].kind] ?? same[0].kind, ways, used, desktop].filter(t => t).join(" · ");
    }
}
