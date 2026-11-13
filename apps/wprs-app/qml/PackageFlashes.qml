import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: flashes

    required property var host
    readonly property var page: host.page
    readonly property bool unset: host.field.required && !host.field.set
    readonly property var report: {
        page.view;
        return JSON.parse(page.editor.rig("", JSON.stringify({
                        "flashes": {}
                    })) || "null");
    }
    readonly property bool scene: (page.view.kind ?? "scene") === "scene"

    RowLayout {
        QQC2.Switch {
            visible: !flashes.unset
            checked: flashes.host.value === true
            onToggled: flashes.host.commit(checked)
        }

        QQC2.Label {
            visible: flashes.unset
            text: App.tr("Not set")
            color: Look.coral
        }

        Repeater {
            model: flashes.unset ? [false, true] : []

            delegate: QQC2.Button {
                required property bool modelData

                text: modelData ? App.tr("Yes") : App.tr("No")
                onClicked: flashes.host.commit(modelData)
            }
        }
    }

    QQC2.Button {
        visible: flashes.scene
        enabled: !flashes.page.editor.busy
        icon.name: "media-playback-start"
        text: App.tr("Measure the flashes")
        QQC2.ToolTip.text: App.tr("Draw 8 seconds of the scene and count how often it flashes; sound and the pointer are not part of the measure")
        QQC2.ToolTip.visible: hovered
        onClicked: flashes.page.run("shot", {
                "flashes": {}
            })
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: flashes.scene && flashes.report !== null
        wrapMode: Text.Wrap
        text: flashes.report?.flashing ? App.tr("It flashes more than three times a second near second %1.", (flashes.report.at ?? 0).toFixed(1)) : App.tr("No flashing found in the %1 seconds that were drawn.", flashes.report?.seconds ?? 0)
        color: flashes.report?.flashing ? Look.coral : Kirigami.Theme.textColor
    }
}
