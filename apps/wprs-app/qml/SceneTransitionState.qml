import QtQuick
import QtCore as Core
import io.github.wallpaper_rs.app

Item {
    id: tool

    required property var page
    property bool open: false
    property string way: "in"
    property real progress: 0.45
    property bool playing: false
    property var info: ({})
    property string before: ""
    property string shown: ""
    property bool busy: false
    property bool again: false
    property int turn: 0
    property bool stillReady: false
    property string said: ""
    property bool waiting: false
    readonly property var current: info[way] ?? null
    readonly property string folder: App.path(Core.StandardPaths.writableLocation(Core.StandardPaths.CacheLocation))
    readonly property string still: folder + "/transition-still.png"
    readonly property string beforeName: before === "" ? App.tr("Plain color") : before.split("/").pop()
    readonly property string look: SceneLook.effective + (SceneLook.calm ? "-calm" : "")

    function named(words, otherwise) {
        if (typeof words === "string")
            return words;
        if (!words)
            return otherwise ?? "";
        return words[Qt.locale().name.substring(0, 2)] ?? words.en ?? otherwise ?? "";
    }

    function load() {
        info = JSON.parse(page.editor.rig("", JSON.stringify({
            "transition": {}
        })) || "{}");
    }

    function act(args) {
        waiting = true;
        page.run("transition", Object.assign({
            "way": way
        }, args));
        said = waiting ? page.editor.error : "";
        waiting = false;
    }

    function start(which) {
        way = which;
        if (!(info[which]))
            page.run("transition", {
                "way": which,
                "make": true
            });
        open = true;
        refresh();
    }

    function close() {
        playing = false;
        open = false;
        shown = "";
    }

    function remove() {
        act({
            "remove": true
        });
        close();
    }

    function refresh() {
        stillReady = false;
        if (open)
            page.editor.render(still, JSON.stringify({
                "quality": SceneLook.effective,
                "size": 960,
                "time": page.clock
            }));
    }

    function draw() {
        if (!open || !stillReady || !current)
            return;
        if (busy) {
            again = true;
            return;
        }
        busy = true;
        turn = (turn + 1) % 2;
        page.editor.render(folder + "/transition-" + turn + ".png", JSON.stringify({
            "quality": SceneLook.effective,
            "calm": SceneLook.calm,
            "crossing": {
                "way": way,
                "progress": progress,
                "still": still,
                "before": before
            }
        }));
    }

    visible: false
    onProgressChanged: draw()
    onLookChanged: refresh()
    onBeforeChanged: draw()
    onWayChanged: draw()
    Component.onCompleted: load()

    NumberAnimation on progress {
        running: tool.open && tool.playing
        from: 0
        to: 1
        duration: tool.current ? tool.current.duration_ms : 1000
        onFinished: tool.playing = false
    }

    Connections {
        function onViewChanged() {
            tool.waiting = false;
            tool.load();
            tool.refresh();
        }

        function onBusyChanged() {
            if (!tool.page.editor.busy && tool.busy) {
                tool.busy = false;
                tool.again = false;
            }
        }

        function onRendered(files) {
            if (files === tool.still) {
                tool.stillReady = true;
                tool.draw();
            } else if (files.indexOf(tool.folder + "/transition-") === 0) {
                tool.busy = false;
                tool.shown = "file://" + files;
                if (tool.again) {
                    tool.again = false;
                    tool.draw();
                }
            }
        }

        target: tool.page.editor
    }
}
