import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

QQC2.MenuItem {
    id: item

    required property var owner
    required property string kind

    enabled: owner.usable(kind)
    text: SceneWords.kind(kind) + (enabled ? "" : " (" + (kind === "shapes" ? App.tr("add a pool first") : App.tr("import a file of this kind first")) + ")")
    icon.name: owner.owner.icon(kind)
    onTriggered: owner.offer(kind)
}
