import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property var scenes: []
    property var projects: JSON.parse(recents.projects() || "[]")
    property bool learn: false
    property bool more: false
    property bool changing: false
    property bool fixed: false
    property bool loaded: false
    property var focused: null
    property string named: App.tr("My scene %1", projects.length + 1)
    property string placed: ""
    readonly property var current: focused ? focused.current : null
    readonly property bool onRecent: focused === recent.row
    readonly property bool atExamples: learn && focused !== null && focused !== recent.row
    readonly property bool missing: onRecent && current !== null && current.missing === true
    readonly property string where: recents.shown(placed !== "" ? placed : helper.folderFor(named))
    readonly property real bandLow: footer.height + Look.e4 + rows.oneLine
    readonly property real bandHigh: bandLow
    readonly property real upperHeight: recent.implicitHeight
    readonly property real bandFrom: Math.min(upper.y, band.y)
    property rect band
    property rect upper
    readonly property rect low: band.width > 0 ? band : Qt.rect(0, upperHeight + Look.e5, width, Math.max(0, height - upperHeight - Look.e5))
    readonly property rect high: upper.width > 0 ? upper : Qt.rect(0, 0, width, upperHeight)
    readonly property alias facts: facts

    function reset() {
        learn = projects.length === 0 && scenes.length > 0;
        focused = projects.length > 0 ? recent.row : learn ? rows.learn : rows.scene;
    }

    function create(starter, media) {
        App.media = media;
        App.compose(placed !== "" ? placed : helper.folderFor(named), starter, named);
    }

    function make(starter) {
        if (starter === "image" || starter === "video")
            return media.start(starter);
        create(starter, "");
    }

    function open(folder) {
        if (!recents.isProject(folder))
            return refuse(folder);
        App.compose(folder, "", "");
    }

    function refuse(folder) {
        const name = folder.split("/").filter(part => part !== "").slice(-1).join("");
        applicationWindow().showPassiveNotification(App.tr("“%1” is not a wallpaper-rs project: it has no scene or package inside.", name), 12000, App.tr("Choose another"), () => page && page.pickFolder());
    }

    function pickFolder() {
        folders.open();
    }

    function act(row) {
        const entry = row.current;
        if (!entry)
            return;
        if (row === recent.row)
            return entry.missing ? pickFolder() : open(entry.folder);
        if (row === rows.learn)
            return page.copyOf(entry);
        make(entry.id);
    }

    function copyOf(entry) {
        const tile = rows.learn.view.currentItem;
        App.query("GetItem", [entry.uid], "Library1", reply => copy && copy.start(reply.item.path, tile || footer.principal, entry.picture));
    }

    function forget() {
        recents.forgetProject(current.folder);
        projects = JSON.parse(recents.projects() || "[]");
        reset();
    }

    function exercise() {
        if (page.scenes.length === 0)
            throw new Error("the create page offers no installed package to copy");
        for (const part of [["page", "web"], ["clip", "video"], ["still", "image"], ["preset", "preset"]])
            if (App.backend.claim("compose." + part[0]))
                App.compose(helper.folderFor(part[0]), part[1], "");
        App.compose(helper.folderFor("check"), "empty", "");
    }

    title: App.tr("Create")
    padding: 0
    background: null
    property var keyHints: [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Tab"],
            "text": App.tr("Change row")
        }]
    Component.onCompleted: fixed ? reset() : facts.load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "library" && !page.fixed)
                page.facts.load();
        }
    }

    SceneEditor {
        id: helper
    }

    Recents {
        id: recents
    }

    FolderDialog {
        id: folders

        onAccepted: {
            const folder = App.path(selectedFolder);
            if (page.missing)
                recents.forgetProject(page.current.folder);
            page.open(folder);
        }
    }

    CreateMedia {
        id: media

        page: page
    }

    CreateCopy {
        id: copy
    }

    CreateData {
        id: facts

        page: page
    }

    CreateRecent {
        id: recent

        page: page
        x: page.high.x
        y: page.high.y
        width: page.high.width
    }

    Flickable {
        id: flick

        x: page.low.x
        y: page.low.y
        width: page.low.width
        height: Math.max(0, page.low.height - footer.height - Look.e4)
        contentWidth: width
        contentHeight: Math.max(height, rows.height)
        clip: true
        boundsBehavior: Flickable.StopAtBounds

        CreateBand {
            id: rows

            page: page
            y: Math.max(0, flick.height - height)
            width: parent.width
        }
    }

    CreateFooter {
        id: footer

        page: page
        x: page.low.x
        y: page.low.y + page.low.height - height
        width: page.low.width - (page.low.x < page.high.x ? room.width + Look.e6 : 0)
    }

    Hints {
        id: room

        visible: false
        model: page.keyHints
    }
}
