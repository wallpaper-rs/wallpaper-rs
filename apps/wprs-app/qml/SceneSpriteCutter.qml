import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property string asset: ""
    property var sheet: null
    property int selected: -1
    readonly property var orders: [["rows", App.tr("Left to right, top to bottom")], ["columns", App.tr("Top to bottom, left to right")], ["rows_reversed", App.tr("Backwards, by rows")], ["columns_reversed", App.tr("Backwards, by columns")]]

    function show(id) {
        asset = id;
        selected = -1;
        load();
        open();
    }

    function load() {
        sheet = JSON.parse(page.editor.rig("", JSON.stringify({
                        "sprites": {
                            "asset": asset
                        }
                    })) || "null");
    }

    function edit(name, args) {
        page.run("sprites", {
                [name]: Object.assign({
                        "asset": asset
                    }, args)
            });
    }

    function cut() {
        edit("grid", {
                "rows": gridRows.value,
                "columns": gridColumns.value,
                "margin": gridMargin.value,
                "spacing": gridGap.value,
                "count": gridCount.value > 0 ? gridCount.value : null,
                "order": orders[order.currentIndex][0]
            });
    }

    title: App.tr("Cut the sprites of %1", asset)
    preferredWidth: Kirigami.Units.gridUnit * 48
    preferredHeight: Kirigami.Units.gridUnit * 36
    standardButtons: Kirigami.Dialog.Close

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        Connections {
            function onViewChanged() {
                if (dialog.opened)
                    dialog.load();
            }

            target: dialog.page.editor
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("Cut the picture in a grid, or drag over it to draw a frame. Drag a frame to move it and its corner to resize it. The picture is %1 × %2 px.", dialog.sheet?.size[0] ?? 0, dialog.sheet?.size[1] ?? 0)
        }

        Flow {
            Layout.fillWidth: true
            spacing: Kirigami.Units.largeSpacing

            SceneSpin {
                id: gridRows

                label: App.tr("Rows")
                from: 1
                to: 256
                value: 4
            }

            SceneSpin {
                id: gridColumns

                label: App.tr("Columns")
                from: 1
                to: 256
                value: 4
            }

            SceneSpin {
                id: gridMargin

                label: App.tr("Margin")
                from: 0
                to: 4096
            }

            SceneSpin {
                id: gridGap

                label: App.tr("Spacing")
                from: 0
                to: 4096
            }

            SceneSpin {
                id: gridCount

                label: App.tr("Frames (0 for all)")
                from: 0
                to: 256
            }

            QQC2.ComboBox {
                id: order

                model: dialog.orders.map(o => o[1])
            }

            QQC2.Button {
                icon.name: "view-grid"
                text: App.tr("Cut the grid")
                onClicked: dialog.cut()
            }

            QQC2.Button {
                icon.name: "edit-clear-all"
                text: App.tr("Remove all frames")
                enabled: (dialog.sheet?.rects.length ?? 0) > 0
                onClicked: dialog.edit("clear", {})
            }
        }

        SceneSpriteBoard {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredHeight: Kirigami.Units.gridUnit * 16
            sheet: dialog.sheet
            selected: dialog.selected
            onPick: index => dialog.selected = index
            onMoved: (index, rect) => dialog.edit("change", {
                        "index": index,
                        "rect": rect
                    })
            onDrawn: rect => {
                dialog.edit("add", {
                        "rect": rect
                    });
                dialog.selected = (dialog.sheet?.rects.length ?? 0);
            }
        }

        SceneSpriteStrip {
            Layout.fillWidth: true
            sheet: dialog.sheet
            selected: dialog.selected
            onPick: index => dialog.selected = index
            onReorder: (from, to) => {
                dialog.edit("reorder", {
                        "from": from,
                        "to": to
                    });
                dialog.selected = to;
            }
            onRemove: index => {
                dialog.edit("remove", {
                        "index": index
                    });
                dialog.selected = -1;
            }
            onPlay: (fps, frames) => dialog.edit("play", {
                        "fps": fps,
                        "frames": frames
                    })
        }
    }
}
