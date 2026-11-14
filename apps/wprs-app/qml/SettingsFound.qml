import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: found

    required property var page
    readonly property var hits: visible ? page.entries.filter(entry => SettingsReach.matches(entry.text, page.query)) : []
    readonly property var groups: [0, 1, 2].map(at => hits.filter(entry => entry.tab === at)).map((list, at) => ({
                "tab": at,
                "list": list
            })).filter(group => group.list.length > 0)
    readonly property var themes: ["tema", "claro", "oscuro", "theme", "light", "dark"]
    readonly property var tries: [App.tr("battery"), App.tr("quality"), App.tr("full screen"), App.tr("folders"), App.tr("lights")]
    readonly property alias tried: tried

    function enter() {
        const first = SettingsReach.rows(found)[0];
        if (first)
            first.forceActiveFocus(Qt.TabFocusReason);
        else if (empty.visible)
            tried.forceActiveFocus(Qt.TabFocusReason);
    }

    implicitHeight: hits.length > 0 ? list.y + list.height : empty.y + empty.height

    Row {
        id: told

        visible: found.hits.length > 0
        spacing: Look.e1

        Text {
            text: found.hits.length === 1 ? App.tr("1 setting") : App.tr("%1 settings", found.hits.length)
            font.family: Look.letraTitulo
            font.pixelSize: Look.tSeccion
            font.weight: Font.Bold
            color: Look.escarcha
        }

        Text {
            text: App.tr("across the three tabs")
            font.family: Look.letraTitulo
            font.pixelSize: Look.tSeccion
            color: Look.bruma
        }
    }

    Filas {
        id: list

        y: told.height + Look.e5
        visible: found.hits.length > 0

        Repeater {
            model: found.groups

            Column {
                id: group

                required property var modelData

                width: parent.width
                spacing: Look.e2

                Seccion {
                    text: found.page.words[group.modelData.tab]
                }

                Repeater {
                    model: group.modelData.list

                    Fila {
                        required property var modelData

                        width: parent.width
                        text: modelData.text
                        note: [found.page.words[modelData.tab]].concat(modelData.place ? [SettingsPlaces.word(modelData.place, found.page.output)] : []).join(" · ")
                        value: modelData.value ?? ""
                        explain: modelData.explain ?? ""
                        off: modelData.off === true
                        kind: "pliega"
                        onOpened: found.page.reach(modelData)
                    }
                }
            }
        }
    }

    Column {
        id: empty

        y: Look.e7
        visible: found.hits.length === 0
        spacing: Look.e3

        Vacio {
            title: App.tr("No setting speaks of “%1”", found.page.query)
            text: [found.themes.some(word => SettingsReach.matches(found.page.query, word)) ? App.tr("The colors of the app come from the wallpaper you have on, so there is no theme to choose.") : "", App.tr("Search with words for what you want to happen, for example:")].filter(part => part !== "").join(" ")
        }

        Filtro {
            id: tried

            label: App.tr("Try with")
            words: found.tries.map(text => ({
                        "text": text
                    }))
            current: -1
            onPicked: index => found.page.query = found.tries[index]
        }
    }
}
