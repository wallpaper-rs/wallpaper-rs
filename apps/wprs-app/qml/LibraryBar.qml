import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Flow {
    id: bar

    required property var tools
    required property int total
    property bool all: false
    readonly property bool tight: all || width < Look.tapaFoco * 2 + Look.tapa
    property int word: 0
    property string kind
    property string tag
    property var tags: []
    readonly property bool favorite: word === 4
    readonly property bool filtered: word !== 0 || kind !== "" || tag !== "" || find.text !== ""
    readonly property alias text: find.text
    readonly property var kinds: ["", "scene", "image", "video", ""]
    readonly property var extra: [{
            "text": App.tr("Packages"),
            "kind": "package"
        }, {
            "text": App.tr("Assets"),
            "kind": "assets"
        }, {
            "text": App.tr("Web"),
            "kind": "web"
        }]

    signal changed
    signal flipped

    function tagsLoad() {
        App.query("ListTags", [], "Library1", reply => bar && (bar.tags = reply.tags));
    }

    function clear() {
        find.text = "";
        word = 0;
        kind = "";
        tag = "";
        changed();
    }

    spacing: Look.e4
    width: parent.width
    Component.onCompleted: tagsLoad()

    Filtro {
        id: words

        label: App.tr("Show")
        current: bar.kind === "" && bar.tag === "" ? bar.word : -1
        words: [App.tr("All"), App.tr("Scenes"), App.tr("Pictures"), App.tr("Videos"), App.tr("Favorites")].map((text, index) => index === words.current ? {
                "text": text,
                "count": bar.total
            } : {
                "text": text
            })
        onPicked: index => {
            bar.word = index;
            bar.kind = "";
            bar.tag = "";
            bar.changed();
        }
    }

    QQC2.Button {
        id: more

        flat: true
        text: bar.kind !== "" ? bar.extra.find(entry => entry.kind === bar.kind).text : bar.tag !== "" ? Labels.tag(bar.tag) : App.tr("More")
        icon.name: "caret-down"
        LayoutMirroring.enabled: true
        onClicked: menu.open()
    }

    ListaMenu {
        id: menu

        readonly property var tagged: bar.tags.map(tag => ({
                    "text": Labels.tag(tag.name),
                    "note": String(tag.count),
                    "tag": tag.name
                }))

        opener: more
        title: App.tr("More")
        options: bar.extra.concat(bar.tags.length > 0 ? [{
                    "separator": true
                }] : []).concat(tagged).concat(bar.tag !== "" ? [{
                    "separator": true
                }, {
                    "text": App.tr("Rename tag…"),
                    "icon": "pencil-simple",
                    "rename": true
                }] : [])
        onChosen: index => {
            const option = options[index];
            if (option.rename)
                return bar.tools.renameTag(bar.tag, more);
            bar.word = 0;
            bar.kind = option.kind ?? "";
            bar.tag = option.tag ?? "";
            bar.changed();
        }
    }

    Buscar {
        id: find

        placeholder: bar.all ? App.tr("Search") : App.tr("Search your library")
        onTextChanged: wait.restart()
        onAccepted: {
            wait.stop();
            bar.changed();
        }
    }

    Timer {
        id: wait

        interval: 250
        onTriggered: bar.changed()
    }

    QQC2.Button {
        id: adder

        flat: true
        icon.name: "plus"
        text: App.tr("Add wallpapers…")
        display: bar.tight ? QQC2.AbstractButton.IconOnly : QQC2.AbstractButton.TextBesideIcon
        onClicked: bar.tools.dialogs.importer.start(adder)
        QQC2.ToolTip.text: text
        QQC2.ToolTip.visible: bar.tight && hovered
    }

    QQC2.Button {
        flat: true
        icon.name: bar.all ? "rows" : "squares-four"
        text: bar.all ? App.tr("Show shelf") : App.tr("See all")
        onClicked: bar.flipped()
    }

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "library")
                bar.tagsLoad();
        }
    }
}
