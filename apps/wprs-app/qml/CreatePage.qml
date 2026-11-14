import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property var scenes: []
    property var projects: JSON.parse(helper.projects() || "[]")
    property bool learn: false
    property bool more: false
    property bool changing: false
    property bool fixed: false
    property var focused: null
    property string named: App.tr("My scene %1", projects.length + 1)
    property string placed: ""
    readonly property var current: focused ? focused.current : null
    readonly property bool onRecent: focused === recent.row
    readonly property bool onLearn: focused === band.learn
    readonly property bool missing: onRecent && current !== null && current.missing === true
    readonly property string where: (placed !== "" ? placed : helper.folderFor(named)).split("/").filter(part => part !== "").slice(-2, -1).join("")
    readonly property alias band: band
    readonly property alias facts: facts

    function reset() {
        learn = projects.length === 0 && scenes.length > 0;
        focused = projects.length > 0 ? recent.row : learn ? band.learn : band.scene;
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
        if (!helper.isProject(folder))
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
        if (row === band.learn)
            return page.copyOf(entry);
        make(entry.id);
    }

    function copyOf(entry) {
        App.query("GetItem", [entry.uid], "Library1", reply => page && copy.start(reply.item.path, footer.principal, entry.picture));
    }

    function forget() {
        helper.forgetProject(current.folder);
        projects = JSON.parse(helper.projects() || "[]");
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
    property var keyHints: [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Tab"],
            "text": App.tr("Change row")
        }]
    Component.onCompleted: {
        if (fixed)
            return reset();
        App.query("Search", ["kind:package", 0, 500], "Library1", reply => {
            if (!page)
                return;
            page.scenes = (reply && reply.items ? reply.items : []).filter(item => item.package && !item.missing);
            page.reset();
        });
    }

    SceneEditor {
        id: helper
    }

    FolderDialog {
        id: folders

        onAccepted: {
            const folder = App.path(selectedFolder);
            if (page.missing)
                helper.forgetProject(page.current.folder);
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

    Flickable {
        id: flick

        anchors.fill: parent
        anchors.bottomMargin: footer.height + Look.e4
        contentWidth: width
        contentHeight: Math.max(height, column.height)
        clip: true
        boundsBehavior: Flickable.StopAtBounds

        Column {
            id: column

            y: Math.max(0, flick.height - height)
            width: parent.width
            spacing: Look.e5

            CreateRecent {
                id: recent

                page: page
                width: parent.width
            }

            CreateBand {
                id: band

                page: page
                width: parent.width
            }
        }
    }

    CreateFooter {
        id: footer

        page: page
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
    }
}
