import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: head

    required property var page

    Opcion {
        Layout.fillWidth: true
        lit: head.page.editor.selected === "/manifest"
        icon: "info"
        text: App.tr("Wallpaper details…")
        note: App.tr("Title, author, license and tags")
        onClicked: head.page.editor.select("/manifest")
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        visible: head.page.view.v2 === false
        text: App.tr("This package uses the first version of the manifest. Upgrade it to give it a rating, credits and texts in several languages; what it cannot tell, such as the rating, the flashes and the origin, stays among the problems until you set it.")
        actions: [
            Kirigami.Action {
                text: App.tr("Upgrade")
                icon.name: "go-up"
                onTriggered: head.page.run("upgrade")
            }
        ]
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        level: 4
        text: App.tr("Options you offer to whoever uses it")
    }
}
