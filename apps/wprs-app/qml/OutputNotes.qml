import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: notes

    required property var info
    required property var daemon
    readonly property var errors: notes.info.page_errors ?? []
    readonly property string errorText: {
        const last = notes.errors.slice(-1)[0] ?? "";
        if (notes.info.kind === "scene")
            return notes.daemon.tr("A script of the scene reported an error: %1", last);
        return notes.daemon.tr("The page reported an error and may not be drawing: %1", last);
    }
    readonly property var stills: ({
            "software": notes.daemon.tr("The picture stays still: this computer draws with its processor, so depth movement is off. Turn it on with Depth parallax with software rendering in the settings; it then moves in 8 steps at up to 15 frames per second."),
            "building_simple": notes.daemon.tr("The picture stays still until its depth layers are built. They are built with the simple fill because LaMa, which fills the hidden parts better, is not installed. You can download it from the depth tool of the editor; nothing is downloaded until you ask.")
        })

    spacing: 0

    QQC2.Label {
        Layout.fillWidth: true
        visible: notes.errors.length > 0
        wrapMode: Text.Wrap
        elide: Text.ElideRight
        maximumLineCount: 3
        text: notes.errorText
        textFormat: Text.PlainText
        color: Look.coral
        font: Kirigami.Theme.smallFont
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: text !== ""
        wrapMode: Text.Wrap
        text: notes.stills[notes.info.still ?? ""] ?? ""
        textFormat: Text.PlainText
        color: Kirigami.Theme.disabledTextColor
        font: Kirigami.Theme.smallFont
    }
}
