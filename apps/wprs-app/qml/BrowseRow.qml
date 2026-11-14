import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: row

    required property var pkg
    required property var catalog
    required property Installer installer
    property bool open
    readonly property var p: pkg ?? {
        "versions": []
    }
    property bool folds: true
    readonly property var job: pkg ? installer.running(pkg) : null
    readonly property bool here: pkg ? notes.here(pkg) : false
    readonly property bool older: pkg ? notes.older(pkg) : false
    readonly property bool own: pkg ? pkg.own !== undefined : false
    readonly property alias main: main
    readonly property alias second: second
    readonly property alias more: more

    signal details

    function act() {
        if (row.catalog.veiled(row.p))
            return row.catalog.reveal(row.pkg);
        if (row.own || row.p.update)
            return row.installer.update(row.pkg);
        if (row.older)
            return row.installer.install(row.pkg, row.p.versions[row.p.versions.length - 1], false, main, false, false);
        if (row.here)
            return row.installer.apply({
                "repo": row.p.repo_id,
                "id": row.p.id,
                "title": row.p.title
            });
        row.installer.install(row.pkg, row.catalog.versions[row.p.repo_id + "/" + row.p.id] ?? "", row.catalog.fresh, main, false, false);
    }

    spacing: Look.e3

    BrowseNotes {
        id: notes
    }

    Estado {
        visible: row.older && !row.job
        text: App.tr("Installed · the repository offers an older version")
        tone: "brote"
        icon: "check-circle"
    }

    Progreso {
        visible: row.job !== null
        width: Look.progresoInstala
        text: App.tr("Installing “%1”…", row.pkg ? row.p.title : "")
        count: row.job && row.job.total > 0 ? App.tr("%1 of %2", App.bytes(row.job.done), App.bytes(row.job.total)) : ""
        value: row.job && row.job.total > 0 ? row.job.done / row.job.total : 0
        indeterminate: !row.job || row.job.total === 0
    }

    Row {
        visible: row.job === null
        spacing: Look.e3

        QQC2.Button {
            id: main

            highlighted: !row.older
            text: row.catalog.veiled(row.p) ? App.tr("Show the picture") : row.own || row.p.update ? App.tr("Update to %1", row.own ? row.p.own.update : notes.newest(row.p)) : row.older ? App.tr("Install the older one…") : row.here || row.catalog.fresh ? App.tr("Set as wallpaper") : App.tr("Install")
            icon.name: row.catalog.veiled(row.p) ? "eye" : row.own || row.p.update ? "arrow-circle-up" : row.older ? "clock-counter-clockwise" : row.here || row.catalog.fresh ? "image" : "download-simple"
            visible: !(row.own && !row.p.own.update)
            onClicked: row.act()
        }

        PiecesTecla {
            anchors.verticalCenter: parent.verticalCenter
            visible: main.visible && main.highlighted
            text: App.trc("key", "Enter")
        }

        QQC2.Button {
            id: second

            visible: !row.here && !row.older && !row.own && !row.p.update && !row.catalog.fresh && !row.catalog.veiled(row.p)
            text: App.tr("Install and set as wallpaper")
            icon.name: "image"
            onClicked: row.installer.install(row.pkg, row.catalog.versions[row.p.repo_id + "/" + row.p.id] ?? "", true, second, false, false)
        }

        Estado {
            anchors.verticalCenter: parent.verticalCenter
            visible: row.here
            text: App.tr("Installed")
            tone: "brote"
            icon: "check-circle"
        }

        BrowseOwn {
            id: more

            anchors.verticalCenter: parent.verticalCenter
            visible: row.own
            pkg: row.pkg
            installer: row.installer
        }

        QQC2.Button {
            visible: !row.own && row.folds
            flat: true
            text: App.tr("Details")
            icon.name: row.open ? "caret-down" : "caret-right"
            LayoutMirroring.enabled: true
            Accessible.name: App.tr("Details of “%1”", row.pkg ? row.p.title : "")
            onClicked: row.details()
        }
    }
}
