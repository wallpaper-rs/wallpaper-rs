import QtQuick
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

FileDialog {
    id: media

    required property var page
    property string starter

    function start(kind) {
        starter = kind;
        open();
    }

    nameFilters: starter === "video" ? [App.tr("Videos (*.webm *.mp4 *.mkv)")] : [App.tr("Pictures (*.png *.jpg *.jpeg *.webp *.gif *.ktx2)")]
    onAccepted: page.create(starter, App.path(selectedFile))
}
