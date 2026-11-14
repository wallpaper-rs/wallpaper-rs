import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.Button {
    id: more

    readonly property alias menu: menu
    property var publisher: null

    function pick(index) {
        if (index === 0)
            packages.open();
        else if (index === 1)
            applicationWindow().work(stall, {});
    }

    text: App.tr("More")
    icon.name: "dots-three"
    Accessible.name: App.tr("More: tools to publish wallpapers")
    onClicked: menu.open()

    ListaMenu {
        id: menu

        opener: more
        options: [{
                "text": App.tr("Upload a package…"),
                "icon": "upload-simple"
            }, {
                "text": App.tr("Static repository…"),
                "icon": "folder-simple"
            }]
        onChosen: index => more.pick(index)
    }

    FileDialog {
        id: packages

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: more.publisher && more.publisher.openWith(App.path(selectedFile))
    }

    Component {
        id: stall

        RepoStaticPage {
        }
    }
}
