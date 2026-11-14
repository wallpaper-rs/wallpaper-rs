import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: main

    property var page: null
    readonly property var account: page ? page.account : null
    readonly property string phase: page ? page.phase : ""
    readonly property var pairing: account ? account.pairing : null
    readonly property alias sync: sync
    readonly property alias empty: empty
    readonly property alias waiting: waiting
    readonly property alias problem: problem
    readonly property alias line: line
    readonly property var rows: [server, sync, key, waiting, devices, repos, more, details]

    function host(url) {
        return (url || "").replace(/^[a-z]+:\/\//, "").replace(/\/.*$/, "");
    }

    function enter(from) {
        const back = {
            "server": server,
            "devices": devices,
            "repos": repos,
            "more": more,
            "details": details
        }[from || ""];
        if (!main.account)
            return empty.forceActiveFocus(Qt.TabFocusReason);
        const row = back && back.visible ? back : rows.find(row => row.visible);
        if (row)
            row.forceActiveFocus(Qt.TabFocusReason);
    }

    function choose(index) {
        if (index === 0)
            App.act("SetSyncing", [false, false], "Sync1");
        else if (main.phase === "connected")
            App.act("SetSyncing", [true, main.page.plain], "Sync1");
        else
            App.act("SetPaused", [index === 2], "Sync1");
    }

    width: parent ? parent.width : 0
    height: column.height

    Column {
        id: column

        width: main.width
        spacing: Look.e5

        Fallo {
            width: Math.min(main.width, Look.filasAncho)
            visible: !main.account && !!main.page && main.page.error !== ""
            big: true
            title: App.tr("Could not read the account")
            text: App.tr("The wallpaper service did not answer. It may still be starting.")
            details: main.page ? main.page.error : ""
            action: App.tr("Try again")
            onActed: main.page.load()
        }

        Vacio {
            id: empty

            width: Math.min(main.width, Look.filasAncho)
            visible: !main.account && !!main.page && main.page.error === ""
            title: App.tr("No server yet")
            text: App.tr("A wallpaper-rs server gives you its repositories to browse, install and vote, and can keep your library, playlists and settings the same on your devices, encrypted end to end.")
            action: App.tr("Add a server…")
            actionIcon: "plus"
            onActed: main.page.panels.add(empty)
        }

        Estado {
            id: line

            visible: !!main.account
            text: AccountWords.line(main.account, main.page ? main.page.code : "", main.page ? main.page.address : "")
            tone: AccountWords.tone(main.account)
        }

        AccountProblem {
            id: problem

            width: Math.min(main.width, Look.filasAncho)
            page: main.page
        }

        Filas {
            visible: !!main.account

            Fila {
                id: server

                text: App.tr("Server")
                note: App.tr("Repositories, browse, install and vote")
                value: main.account ? (main.phase === "unlinked" || main.phase === "linking" ? App.tr("not connected") : main.host(main.account.url)) : ""
                kind: "pliega"
                onOpened: main.page.open("server")
            }

            Fila {
                id: sync

                text: App.tr("Sync my library")
                note: App.tr("Wallpapers, playlists and settings, between your devices")
                kind: off ? "texto" : "elige"
                off: !(main.page && main.page.linked)
                options: [App.tr("Off"), App.trc("sync of the library", "On"), App.tr("Paused")]
                current: main.phase === "connected" ? 0 : main.phase === "paused" ? 2 : 1
                value: off ? App.tr("connect first") : ""
                explain: AccountWords.explain(main.account, off)
                onChosen: index => main.choose(index)
            }

            Fila {
                id: key

                visible: main.phase === "needs_key"
                text: App.tr("Get the key from another device")
                note: App.tr("This device needs the key of a device that already syncs")
                kind: "pliega"
                value: App.tr("choose the device")
                onOpened: main.page.open("devices")
            }

            Fila {
                id: waiting

                visible: !!main.pairing && !!main.pairing.code
                text: App.tr("Another device is waiting")
                note: App.tr("Check that both screens show the same number")
                value: main.pairing && main.pairing.code ? main.pairing.code : ""
                kind: "pliega"
                onOpened: main.page.codes.resume(waiting)
            }

            Fila {
                id: devices

                visible: !!main.page && main.page.linked
                text: App.tr("Devices of the account")
                value: main.account && main.account.devices.length === 1 ? App.tr("one device") : App.tr("%1 devices", main.account ? main.account.devices.length : 0)
                kind: "pliega"
                onOpened: main.page.open("devices")
            }

            Fila {
                id: repos

                visible: !!main.page && main.page.linked
                text: App.tr("Repositories of your other devices")
                value: main.account && main.account.offers.length > 0 ? App.tr("%1 to add", main.account.offers.length) : App.tr("none to add")
                kind: "pliega"
                onOpened: main.page.open("repos")
            }

            Fila {
                id: more

                text: App.tr("More")
                value: App.tr("Another device, recovery passphrase…")
                kind: "pliega"
                onOpened: main.page.open("more")
            }

            Fila {
                id: details

                text: App.tr("Details")
                value: App.tr("Server, certificate, storage")
                kind: "pliega"
                onOpened: main.page.open("details")
            }
        }
    }
}
