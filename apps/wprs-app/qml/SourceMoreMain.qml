import QtQuick
import io.github.wallpaper_rs.app

Filas {
    id: main

    required property var more
    readonly property var sorting: more.field("sorting").values || []
    readonly property bool random: more.shown.sorting === "random"
    readonly property var size: ["atleast", "resolutions", "ratios", "colors"].filter(key => String(more.shown[key] ?? "") !== "")

    function enter() {
        first.forceActiveFocus(Qt.TabFocusReason);
    }

    Seccion {
        text: App.tr("What photos")
    }

    Fila {
        id: first

        kind: "pliega"
        text: App.tr("Categories")
        value: main.more.words("categories")
        onOpened: main.more.deeper("categories")
    }

    Fila {
        kind: "pliega"
        text: App.tr("Content")
        note: App.tr("The general filter of Settings rules")
        value: main.more.words("purity")
        explain: App.tr("Choose between for everyone, a bit daring and for adults; you can mark several. If adult content is hidden in Settings, those photos do not arrive even if you mark them here.")
        onOpened: main.more.deeper("purity")
    }

    Fila {
        kind: "pliega"
        text: App.tr("Size and colors")
        note: App.tr("Minimum resolution, proportions and colors")
        value: main.size.length === 0 ? App.tr("Any") : main.size.map(key => key === "atleast" ? App.tr("%1 or more", SourceMoreWords.choice(key, main.more.shown.atleast)) : main.more.words(key)).join(" · ")
        onOpened: main.more.deeper("size")
    }

    Seccion {
        text: App.tr("How to find them")
    }

    Fila {
        kind: "elige"
        text: App.tr("Order")
        options: main.sorting.map(v => SourceMoreWords.choice("sorting", v))
        current: Math.max(0, main.sorting.indexOf(main.more.shown.sorting))
        words: main.random ? [{
                "text": App.tr("Shuffle again"),
                "icon": "shuffle"
            }] : []
        onChosen: index => main.more.set({
                "sorting": main.sorting[index]
            })
        onWorded: App.act("Reseed", [main.more.provider.id], "Providers1")
    }

    Fila {
        kind: "campo"
        text: App.tr("Collections of")
        placeholder: App.tr("User")
        fieldText: main.more.owner
        note: main.more.shown.collection ? App.tr("Showing the collection %1", main.more.shown.collection) : ""
        words: [{
                "text": App.tr("List")
            }].concat(main.more.shown.collection ? [{
                    "text": App.tr("Search instead")
                }] : [])
        onEdited: text => main.more.collections(text.trim())
        onWorded: index => index === 0 ? main.more.collections(main.more.owner) : main.more.set({
                "collection": ""
            })
    }

    Fila {
        kind: "campo"
        text: App.tr("Similar to")
        note: App.tr("With the code of a Wallhaven photo, brings photos like that one")
        placeholder: App.tr("Photo code")
        fieldText: main.more.shown.similar || ""
        onEdited: text => main.more.set({
                "similar": text.trim()
            })
    }

    Fila {
        kind: "pliega"
        text: App.tr("Other search options")
        value: App.tr("Search words, order, pages and more")
        onOpened: main.more.deeper("other")
    }
}
