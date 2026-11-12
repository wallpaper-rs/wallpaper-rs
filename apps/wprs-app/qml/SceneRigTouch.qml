import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: touch

    required property var owner
    readonly property bool picking: owner.host.page.mesh.mode === "hit" && owner.host.page.mesh.rigging === owner.host.layerId

    QQC2.Label {
        text: App.tr("Bones that take the pointer")
        font.bold: true
    }

    SceneRepeater {
        id: listed

        rows: touch.owner.touched

        delegate: SceneRigRow {
            readonly property var modelData: listed.rows[index]
            required property int index

            text: modelData
            onRemoved: touch.owner.put("/hit", touch.owner.without(touch.owner.touched, index))
        }
    }

    QQC2.Button {
        text: App.tr("Pick them on the preview")
        icon.name: "input-mouse"
        checkable: true
        checked: touch.picking
        onToggled: touch.owner.host.page.mesh.use(touch.owner.host.layerId, checked ? "hit" : "")
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: touch.picking
        wrapMode: Text.Wrap
        text: App.tr("Click a bone on the preview to make it take the pointer, or click it again to let it go; the bones that take it have a thick ring.")
    }

    RowLayout {
        QQC2.ComboBox {
            id: hit

            Layout.fillWidth: true
            model: touch.owner.names.filter(n => !touch.owner.touched.includes(n))
        }

        QQC2.Button {
            text: App.tr("Add")
            enabled: hit.currentText !== ""
            onClicked: touch.owner.put("/hit", touch.owner.touched.concat([hit.currentText]))
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: touch.owner.touched.length > 0
        wrapMode: Text.Wrap
        text: App.tr("Scripts learn which of these bones the pointer touched, with the pointer permission.")
    }
}
