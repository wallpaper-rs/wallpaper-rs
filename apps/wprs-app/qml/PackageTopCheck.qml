import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function gather(item, out) {
        out.push(item);
        for (const child of item.children)
            gather(child, out);
        return out;
    }

    function words(item) {
        return gather(item, []).filter(at => at.checkable !== undefined && at.visible).map(at => at.text);
    }

    function exercise() {
        const bar = page.bar.createObject(applicationWindow().contentItem);
        const shown = words(bar);
        const wanted = [App.tr("Undo"), App.tr("Redo"), App.tr("Set as wallpaper"), App.tr("Publish…")].concat(page.viewable ? [App.tr("View as wallpaper")] : []);
        const missing = wanted.filter(word => !shown.includes(word));
        if (missing.length > 0 || page.viewable !== shown.includes(App.tr("View as wallpaper")))
            throw new Error(page.kind + ": the top bar shows " + shown.join(", "));
        const publish = gather(bar, []).find(at => at.text === App.tr("Publish…"));
        publish.clicked();
        if (!page.builder.visible)
            throw new Error(page.kind + ": Publish… does not open publishing");
        page.builder.close();
        bar.destroy();
        const named = gather(page, []).filter(at => at.checkable !== undefined).map(at => at.text);
        const tabs = {
            "web": [App.tr("Options"), App.tr("See as")],
            "image": [App.tr("See as")],
            "video": [App.tr("See as")]
        }[page.kind] ?? [];
        const lost = tabs.concat([App.tr("Wallpaper details…")]).filter(word => !named.includes(word));
        if (lost.length > 0 || named.includes(App.tr("Preview")) || named.includes(App.tr("Properties")))
            throw new Error(page.kind + ": the package page lacks " + lost.join(", ") + " among " + named.join(", "));
    }
}
