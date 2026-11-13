import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: file

    required property var page
    readonly property bool video: page.kind === "video"
    property var info: null

    function load() {
        info = JSON.parse(page.editor.rig("", JSON.stringify({
                        "media": {}
                    })) || "null");
    }

    function size(bytes) {
        return bytes >= 1048576 ? (bytes / 1048576).toFixed(1) + " MiB" : Math.max(1, Math.round(bytes / 1024)) + " KiB";
    }

    Component.onCompleted: load()

    Connections {
        function onViewChanged() {
            file.load();
        }

        target: file.page.editor
    }

    RowLayout {
        Layout.fillWidth: true

        QQC2.Label {
            Layout.fillWidth: true
            elide: Text.ElideMiddle
            textFormat: Text.PlainText
            text: file.info?.present ? file.info.entry : App.tr("Nothing chosen yet")
        }

        QQC2.Button {
            text: file.video ? App.tr("Choose the video…") : App.tr("Choose the picture…")
            icon.name: "document-open"
            onClicked: chooser.open()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: !!file.info?.present
        color: Kirigami.Theme.disabledTextColor
        text: (file.info?.size ? file.info.size[0] + " × " + file.info.size[1] + " · " : "") + (file.info?.length ? App.tr("%1 s", file.info.length.toFixed(1)) + " · " : "") + file.size(file.info?.bytes ?? 0)
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: !file.info?.present
        color: Look.ambar
        text: file.video ? App.tr("Choose a video (webm, mp4 or mkv): it is copied into the package.") : App.tr("Choose a picture (png, jpeg, webp, gif or ktx2): it is copied into the package.")
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        visible: !file.video
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("This package shows one still picture. To animate it or add effects, make a scene and add the picture as a layer.")
    }

    FileDialog {
        id: chooser

        nameFilters: file.video ? [App.tr("Videos (*.webm *.mp4 *.mkv)")] : [App.tr("Pictures (*.png *.jpg *.jpeg *.webp *.gif *.ktx2)")]
        onAccepted: file.page.manifestEdit("media", {
                "file": App.path(selectedFile)
            })
    }
}
