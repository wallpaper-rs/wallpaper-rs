import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: use

    required property var page
    property bool placing: false
    property bool saved: false
    property bool kept: false
    property var before: ({})
    property string screen
    property string viewing
    property bool stopping: false
    property int was: Window.Windowed
    readonly property string name: page.view.title ?? ""
    readonly property var window: page.Window.window
    readonly property alias note: note

    function store() {
        if (!page.editor.dirty)
            return false;
        page.editor.save();
        return !page.editor.dirty;
    }

    function ask(opener, viewing, then) {
        if (App.outputs.length < 2)
            then("");
        else
            screens.choose(opener, viewing, then);
    }

    function place(opener) {
        ask(opener, false, output => use.apply(output));
    }

    function apply(output) {
        saved = store();
        const outputs = output === "" ? App.outputs : [output];
        const shown = App.state.outputs ?? {};
        const items = {};
        for (const name of outputs)
            items[name] = (shown[name] ?? {}).item ?? "";
        before = items;
        screen = output;
        placing = true;
        page.editor.apply(output);
    }

    function placed(message) {
        placing = false;
        const told = [saved ? App.tr("Saved before setting it as the wallpaper.") : "", App.tr("“%1” is your wallpaper.", name)].concat(message.split("\n").slice(1));
        if (kept) {
            kept = false;
            note.show(App.tr("“%1” is your wallpaper.", name), "", JSON.stringify([["default", ""], ["back", App.tr("Back to the editor")]]));
        }
        applicationWindow().showPassiveNotification(told.filter(line => line !== "").join(" "), "long", App.tr("Undo"), () => use.undo());
    }

    function undo() {
        for (const output of Object.keys(before))
            if (before[output] !== "")
                App.act("Apply", [before[output], output, "", ""], "Library1", () => {});
    }

    function publish() {
        if (store())
            applicationWindow().showPassiveNotification(App.tr("Saved before publishing."), "short");
        page.builder.open();
    }

    function view(opener) {
        ask(opener, true, output => use.show(output));
    }

    function show(output) {
        screen = output;
        stopping = false;
        viewing = "on";
        page.editor.preview(true, output);
        note.show(App.tr("You are viewing “%1” as your wallpaper", name), App.tr("It is the scene as it is in the editor. Your usual wallpaper comes back when you finish."), JSON.stringify([["default", ""], ["back", App.tr("Back to the editor")], ["keep", App.tr("Keep it")], ["stop", App.tr("Stop the preview")]]));
        if (!App.checking && window.visibility !== Window.Minimized) {
            was = window.visibility;
            window.showMinimized();
        }
    }

    function stop() {
        stopping = true;
        viewing = "";
        page.editor.preview(false, screen);
        note.close();
    }

    function back() {
        if (viewing === "on")
            stop();
        note.close();
        viewing = "";
        restore();
    }

    function restore() {
        if (App.checking)
            return;
        window.visibility = was === Window.Hidden || was === Window.Minimized ? Window.Windowed : was;
        window.raise();
        window.requestActivate();
    }

    function ended() {
        viewing = "ended";
        note.show(App.tr("The wallpaper view ended"), App.tr("“%1” is no longer shown on the desktop.", name), JSON.stringify([["default", ""], ["again", App.tr("View it again")], ["back", App.tr("Back to the editor")]]));
    }

    DeskNote {
        id: note

        onActed: key => {
            if (key === "keep") {
                use.stop();
                use.kept = true;
                use.apply(use.screen);
            } else if (key === "stop") {
                use.stop();
            } else if (key === "again") {
                use.show(use.screen);
            } else {
                use.back();
            }
        }
        onFailed: {
            use.restore();
            applicationWindow().showPassiveNotification(App.tr("“%1” is shown on the desktop as a preview.", use.name), "long", App.tr("Stop the preview"), () => use.stop());
        }
    }

    Connections {
        target: use.page.editor

        function onLiveChanged() {
            if (!use.page.editor.live && use.viewing === "on" && !use.stopping)
                use.ended();
        }

        function onErrorChanged() {
            if (use.page.editor.error !== "")
                use.placing = use.kept = false;
        }
    }

    SceneWhere {
        id: screens
    }
}
