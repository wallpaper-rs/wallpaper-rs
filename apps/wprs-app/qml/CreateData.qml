import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: facts

    required property var page
    readonly property var story: CreateStarters {
    }
    readonly property var recentModel: page.projects.map(entry => ({
                "folder": entry.folder,
                "kind": entry.kind,
                "version": entry.version ?? "",
                "edited": entry.edited,
                "missing": entry.missing,
                "name": entry.title,
                "picture": entry.poster ? App.url(entry.poster) : "",
                "icon": "stack",
                "off": entry.missing,
                "note": entry.missing ? App.tr("Its folder is gone") : App.tr("Edited %1", ago(entry.edited)),
                "noteIcon": entry.missing ? "warning-circle" : "",
                "noteTone": entry.missing ? "coral" : ""
            }))
    readonly property var sceneModel: story.scene
    readonly property var singleModel: story.single
    readonly property var moreModel: story.more
    readonly property var learnModel: page.scenes.map(item => ({
                "uid": item.uid,
                "name": item.title,
                "license": item.package.license ?? "",
                "example": (item.tags ?? []).includes("example"),
                "picture": item.thumb ? App.url(item.thumb) : ""
            })).sort((a, b) => b.example - a.example)
    readonly property string pie: !page.current ? "" : page.onRecent ? [SceneWords.kind(page.current.kind), page.current.version ? App.tr("version %1", page.current.version) : "", App.tr("edited %1", ago(page.current.edited))].filter(part => part !== "").join(" · ") : page.atExamples ? [App.tr("Scene"), page.current.example ? App.tr("Examples") : "", page.current.license].filter(part => part !== "").join(" · ") : ""

    function ago(seconds) {
        const minutes = Math.round((Date.now() / 1000 - seconds) / 60);
        if (minutes < 2)
            return App.tr("just now");
        if (minutes < 60)
            return App.tr("%1 minutes ago", minutes);
        const hours = Math.round(minutes / 60);
        if (hours < 24)
            return hours === 1 ? App.tr("an hour ago") : App.tr("%1 hours ago", hours);
        const days = Math.round(hours / 24);
        return days === 1 ? App.tr("yesterday") : App.tr("%1 days ago", days);
    }
}
