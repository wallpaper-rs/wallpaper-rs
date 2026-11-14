import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: add

    required property var owner
    property var files: []
    property bool copy: false
    property bool typing: false
    property var jobs: []
    property var ended: []
    readonly property var typed: area.text.split("\n").map(line => line.trim()).filter(line => line !== "").map(line => line.startsWith("file://") ? App.path(line) : line)
    readonly property var paths: typing ? typed : files
    readonly property int inner: Look.panelChico - 2 * Look.e4

    function start(from) {
        opener = from;
        files = [];
        typing = false;
        area.text = "";
        open();
        chooser.open();
    }

    function bring(paths, copy) {
        if (paths.length > 0)
            App.act("Import", [paths, copy], "Library1", reply => add && add.follow(Number(Array.isArray(reply) ? reply[0] : reply)));
    }

    function follow(job) {
        if (ended.includes(job))
            return done(job);
        jobs = jobs.concat([job]);
    }

    function done(job) {
        App.query("GetJob", [job], "Daemon1", reply => {
            if (!add)
                return;
            const output = reply.job.output ?? {};
            const imported = (output.imported ?? []).length;
            const failed = output.failed ?? [];
            if (imported + failed.length === 0)
                return;
            if (imported === 0)
                return applicationWindow().showPassiveNotification(App.tr("The files could not be added: %1", failed[0].error), "long");
            const said = imported === 1 ? App.tr("1 wallpaper was added.") : App.tr("%1 wallpapers were added.", imported);
            const more = failed.length > 0 ? " " + App.tr("%1 could not be added.", failed.length) : "";
            applicationWindow().showPassiveNotification(said + more, "long", App.tr("See"), () => {
                if (!add)
                    return;
                App.navigate(0, "");
                add.owner.page.covers.newest();
            });
        });
    }

    title: paths.length > 1 ? App.tr("Add %1 wallpapers", paths.length) : paths.length === 1 ? App.tr("Add 1 wallpaper") : App.tr("Add wallpapers")
    text: App.tr("Copy them to your library or leave them where they are?")
    quiet: App.tr("Cancel")
    principal: App.tr("Add")
    first: copy ? copying : leaving
    onAccepted: paths.length > 0 ? bring(paths, copy) : typing || chooser.open()
    foot: [
        QQC2.Button {
            flat: true
            text: add.typing ? App.tr("Choose files…") : App.tr("Write the paths…")
            onClicked: {
                if (add.typing)
                    return chooser.open();
                area.text = add.files.join("\n");
                add.typing = true;
                area.forceActiveFocus();
            }
        }
    ]

    LibraryAddWay {
        id: copying

        width: add.inner
        text: App.tr("Copy them to your library")
        reason: App.tr("They stay here even if you move or delete the originals.")
        marked: add.copy
        onPicked: add.copy = true
    }

    LibraryAddWay {
        id: leaving

        width: add.inner
        text: App.tr("Leave them where they are")
        reason: App.tr("They take no extra space; if you move the folder, they stop showing.")
        marked: !add.copy
        onPicked: add.copy = false
    }

    Area {
        id: area

        visible: add.typing
        width: add.inner
        height: Look.e6 * 4
        placeholderText: App.tr("One path per line")
        Accessible.name: App.tr("Paths to import")
    }

    FileDialog {
        id: chooser

        fileMode: FileDialog.OpenFiles
        onAccepted: {
            add.files = Array.from(selectedFiles, file => App.path(file));
            add.typing = false;
            add.opened || add.open();
        }
    }

    Connections {
        target: App.backend

        function onJobFinished(job, state) {
            add.ended = add.ended.slice(-15).concat([job]);
            if (!add.jobs.includes(job))
                return;
            add.jobs = add.jobs.filter(other => other !== job);
            if (state === "done")
                add.done(job);
        }
    }
}
