import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var page
    property var pool: null
    readonly property var pictures: (page.view.assets ?? []).filter(asset => asset.kind === "image").map(asset => asset.id)

    function show(existing) {
        pool = existing;
        name.text = existing?.id ?? "";
        shapes.value = existing?.max ?? 16;
        vertices.value = existing?.max_vertices ?? 1024;
        picture.currentIndex = existing?.texture ? pictures.indexOf(existing.texture) + 1 : 0;
        own.checked = existing?.size !== null && existing?.size !== undefined;
        wide.value = existing?.size?.[0] ?? 256;
        tall.value = existing?.size?.[1] ?? 256;
        open();
    }

    title: pool ? App.tr("Pool %1", pool.id) : App.tr("New pool")
    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
    onAccepted: {
        const edit = {
            "id": name.text,
            "add": pool === null,
            "max": shapes.value,
            "max_vertices": vertices.value,
            "clear": [picture.currentIndex > 0 ? "" : "texture", own.checked ? "" : "size"].filter(key => key)
        };
        if (picture.currentIndex > 0)
            edit.texture = pictures[picture.currentIndex - 1];
        if (own.checked)
            edit.size = [wide.value, tall.value];
        page.run("pool", edit);
    }

    Kirigami.FormLayout {
        QQC2.TextField {
            id: name

            Kirigami.FormData.label: App.tr("Name:")
            enabled: dialog.pool === null
            validator: RegularExpressionValidator {
                regularExpression: /[a-z0-9_]{0,32}/
            }
        }

        QQC2.SpinBox {
            id: shapes

            Kirigami.FormData.label: App.tr("Shapes at once:")
            from: 1
            to: 256
            editable: true
        }

        QQC2.SpinBox {
            id: vertices

            Kirigami.FormData.label: App.tr("Vertices of each shape:")
            from: 3
            to: 65536
            editable: true
        }

        QQC2.ComboBox {
            id: picture

            Kirigami.FormData.label: App.tr("Picture:")
            model: [App.tr("None")].concat(dialog.pictures)
        }

        QQC2.CheckBox {
            id: own

            Kirigami.FormData.label: App.tr("Size:")
            text: App.tr("Own size in pixels")
        }

        RowLayout {
            Kirigami.FormData.label: App.tr("Width and height:")
            enabled: own.checked

            QQC2.SpinBox {
                id: wide

                from: 1
                to: 4096
                editable: true
            }

            QQC2.SpinBox {
                id: tall

                from: 1
                to: 4096
                editable: true
            }
        }
    }
}
