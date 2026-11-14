import QtQuick
import io.github.wallpaper_rs.app

FocusScope {
    id: shared

    property var page: null
    property var offer: ({})
    readonly property var offers: page && page.account ? page.account.offers : []
    readonly property alias menu: menu

    function enter() {
        const first = rows.itemAt(0);
        (first || empty).forceActiveFocus(Qt.TabFocusReason);
    }

    width: parent ? parent.width : 0
    height: list.height

    Filas {
        id: list

        Vacio {
            id: empty

            width: parent.width
            visible: shared.offers.length === 0
            small: true
            title: App.tr("Nothing to add")
            text: App.tr("Repositories that you add on your other devices wait here until you choose whether to add them on this computer.")
        }

        Repeater {
            id: rows

            model: shared.offers

            delegate: Fila {
                id: row

                required property var modelData

                text: modelData.name
                note: modelData.url
                kind: "pliega"
                value: App.tr("add it here?")
                explain: App.tr("Its key starts with %1. Add it only if you added it yourself on another device.", (modelData.fingerprints[0] || "").slice(0, 16))
                onOpened: {
                    shared.offer = modelData;
                    menu.opener = row;
                    menu.open();
                }
            }
        }
    }

    ListaMenu {
        id: menu

        options: [{
                "text": App.tr("Add it here"),
                "icon": "plus"
            }, {
                "text": App.tr("Not here"),
                "icon": "x"
            }]
        onChosen: index => App.act("ConfirmRepo", [shared.offer.id, index === 0], "Sync1")
    }
}
