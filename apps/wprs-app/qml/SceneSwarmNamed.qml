import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.InlineMessage {
    id: named

    required property var page
    property var renamed: null
    readonly property var where: (renamed?.uses ?? []).map(use => use.path + ":" + use.line)

    signal done

    visible: renamed !== null
    type: Kirigami.MessageType.Warning
    showCloseButton: true
    text: renamed ? App.tr("These lines of the scripts still name %1: %2", renamed.from, where.join(", ")) : ""
    actions: [
        Kirigami.Action {
            icon.name: "edit-find-replace"
            text: App.tr("Rewrite the exact names")
            onTriggered: {
                named.page.run("particles", {
                        "scripts": {
                            "from": named.renamed.from,
                            "to": named.renamed.to
                        }
                    });
                named.done();
            }
        }
    ]
    onVisibleChanged: {
        if (!visible)
            done();
    }
}
