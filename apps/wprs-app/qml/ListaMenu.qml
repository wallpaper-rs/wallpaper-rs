import QtQuick
import io.github.wallpaper_rs.app

PanelChico {
    id: menu

    property var options: []
    property int current: -1

    signal chosen(int index)

    function usable(index) {
        const option = options[index];
        return option !== undefined && !option.separator && !option.off;
    }

    function move(by) {
        let next = current;
        for (let tries = 0; tries < options.length; tries++) {
            next = (next + by + options.length) % options.length;
            if (usable(next))
                return current = next;
        }
    }

    function choose(index) {
        if (!usable(index))
            return;
        chosen(index);
        close();
    }

    width: Look.menuChico
    padded: false
    first: list
    onAboutToShow: usable(current) || move(1)

    FocusScope {
        id: list

        width: menu.width
        implicitHeight: column.implicitHeight + 2 * Look.listaHueco
        Accessible.role: Accessible.PopupMenu
        Keys.onUpPressed: menu.move(-1)
        Keys.onDownPressed: menu.move(1)
        Keys.onReturnPressed: menu.choose(menu.current)
        Keys.onEnterPressed: menu.choose(menu.current)
        Keys.onSpacePressed: menu.choose(menu.current)

        Column {
            id: column

            x: Look.listaHueco
            y: Look.listaHueco
            width: parent.width - 2 * Look.listaHueco

            Repeater {
                model: menu.options

                Loader {
                    id: entry

                    required property var modelData
                    required property int index

                    width: column.width
                    sourceComponent: modelData.separator ? line : item

                    Component {
                        id: line

                        Item {
                            implicitHeight: Look.linea + 2 * Look.listaHueco

                            Rectangle {
                                y: Look.listaHueco
                                width: parent.width
                                height: Look.linea
                                color: Look.nocheLinea
                            }
                        }
                    }

                    Component {
                        id: item

                        Opcion {
                            text: entry.modelData.text ?? ""
                            icon: entry.modelData.icon ?? ""
                            note: entry.modelData.note ?? ""
                            tone: entry.modelData.tone ?? ""
                            off: entry.modelData.off ?? false
                            lit: list.activeFocus && menu.current === entry.index
                            onClicked: menu.choose(entry.index)
                        }
                    }
                }
            }
        }
    }
}
