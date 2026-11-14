import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

FocusScope {
    id: shelf

    required property var page
    readonly property bool empty: page.loaded && page.playlists.length === 0
    readonly property real low: empty ? nothing.y + nothing.implicitHeight : bar.height + Look.e5 + covers.implicitHeight
    readonly property alias covers: covers
    readonly property alias filter: filter
    readonly property var entries: page.shown.map(p => ({
                "uid": p.uid,
                "name": p.name,
                "caption": shelf.caption(p),
                "note": shelf.note(p),
                "noteIcon": "clock",
                "pictures": page.pictures[p.uid] ?? [],
                "picture": (page.pictures[p.uid] ?? [])[0] ?? ""
            }))
    readonly property var entry: page.shown[covers.currentIndex] ?? null
    readonly property var hints: empty ? [] : [{
            "keys": ["left", "right"],
            "text": App.tr("Move around")
        }, {
            "keys": ["Tab"],
            "text": App.tr("Next block")
        }]

    function every(text) {
        const found = /^(\d+)\s*([smhd]?)$/.exec(String(text ?? ""));
        if (!found)
            return String(text ?? "");
        const amount = Number(found[1]);
        const unit = found[2] || "s";
        if (unit === "d")
            return amount === 1 ? App.tr("every day") : App.tr("every %1 days", amount);
        if (unit === "h")
            return App.tr("every %1 h", amount);
        if (unit === "m")
            return amount % 60 === 0 ? App.tr("every %1 h", amount / 60) : App.tr("every %1 min", amount);
        return App.tr("every %1 s", amount);
    }

    function caption(p) {
        const parts = [p.count === 1 ? App.tr("1 wallpaper") : App.tr("%1 wallpapers", p.count), every(p.rules.interval)];
        if (p.outputs.length === 0)
            parts.push(App.tr("stopped"));
        return parts.join(" · ");
    }

    function note(p) {
        const at = page.next(p);
        if (at <= 0)
            return "";
        const minutes = Math.max(1, Math.ceil((at - Date.now() / 1000) / 60));
        return minutes < 90 ? App.tr("Changes in %1 min", minutes) : App.tr("Changes in %1 h", Math.round(minutes / 60));
    }

    Item {
        id: bar

        width: parent.width
        height: filter.implicitHeight

        Filtro {
            id: filter

            visible: !shelf.empty
            label: App.tr("Show")
            current: shelf.page.kind
            words: [{
                    "text": App.trc("playlists", "All"),
                    "count": shelf.page.playlists.length
                }, {
                    "text": App.trc("playlists", "Running"),
                    "count": shelf.page.running.length
                }]
            onPicked: index => shelf.page.kind = index
        }

        QQC2.Button {
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            icon.name: "plus"
            text: App.tr("New playlist…")
            onClicked: shelf.page.creator.open()
        }
    }

    Estante {
        id: covers

        y: bar.height + Look.e5
        width: parent.width
        visible: !shelf.empty
        focus: !shelf.empty
        label: App.tr("Your playlists")
        model: shelf.entries
        onActivated: shelf.entry && shelf.page.start(shelf.entry)
        actions: Row {
            spacing: Look.e3

            QQC2.Button {
                highlighted: true
                icon.name: "play-fill"
                text: App.outputs.length > 1 ? App.tr("Start on %1", shelf.page.target) : App.trc("playlists", "Start")
                onClicked: shelf.page.start(shelf.entry)
            }

            PiecesTecla {
                anchors.verticalCenter: parent.verticalCenter
                text: App.trc("key", "Enter")
            }

            QQC2.Button {
                flat: true
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("Edit")
                icon.name: "caret-right"
                LayoutMirroring.enabled: true
                onClicked: shelf.page.opened = shelf.entry.uid
            }
        }
    }

    Column {
        id: nothing

        y: bar.height + Look.e7
        width: parent.width
        visible: shelf.empty
        spacing: Look.e4

        Vacio {
            title: App.tr("You have no playlists yet")
            text: App.tr("A playlist rotates its wallpapers on your desktop. The quickest way: choose several wallpapers in your library and press “Rotate these every…”. Here you name them and change them.")
        }

        Nota {
            icon: "heart"
            text: App.tr("“Favorites” already exists and fills itself: mark a wallpaper with the heart.")
        }

        Row {
            spacing: Look.e3

            QQC2.Button {
                id: choose

                focus: shelf.empty
                highlighted: true
                icon.name: "images"
                text: App.tr("Choose wallpapers in the library")
                Keys.onReturnPressed: clicked()
                Keys.onEnterPressed: clicked()
                onClicked: applicationWindow().showPage(0)
            }

            PiecesTecla {
                anchors.verticalCenter: parent.verticalCenter
                text: App.trc("key", "Enter")
            }

            QQC2.Button {
                flat: true
                anchors.verticalCenter: parent.verticalCenter
                text: App.tr("New playlist…")
                onClicked: shelf.page.creator.open()
            }
        }
    }

    QQC2.Label {
        x: Look.margen
        visible: shelf.page.error !== ""
        text: shelf.page.error
        color: Look.coral
    }
}
