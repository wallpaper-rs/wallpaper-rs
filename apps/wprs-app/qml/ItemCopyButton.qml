import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Button {
    id: button

    required property var drawer

    text: App.tr("Edit a copy…")
    icon.name: "edit-copy"
    visible: !!drawer.pkg && String(drawer.item.path ?? "").endsWith(".wpak") && !drawer.item.missing
    onClicked: {
        drawer.close();
        copying.start(drawer.item.path);
    }

    ItemCopy {
        id: copying
    }
}
