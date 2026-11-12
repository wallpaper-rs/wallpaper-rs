import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: publish

    required property var page
    property string basePath: ""
    property var report: ({
            "ready": false,
            "steps": []
        })
    readonly property var names: ({
            "base": App.tr("The base package is chosen"),
            "values": App.tr("The preset sets at least one value or file"),
            "title": App.tr("It has a title"),
            "license": App.tr("It has a license"),
            "preview": App.tr("It has a preview picture"),
            "check": App.tr("The review finds no problem")
        })

    function refresh() {
        report = JSON.parse(page.editor.rig("", JSON.stringify({
                    "preset_steps": {}
                })) || "null") ?? {
            "ready": false,
            "steps": []
        };
    }

    function shot(animated) {
        const base = basePath === "" ? ({}) : {
            "base": basePath
        };
        page.run("shot", animated ? {
                "recording": Object.assign({
                    "from": 0,
                    "seconds": 4,
                    "fps": 10,
                    "format": asWebp.checked ? "webp" : "gif"
                }, base)
            } : {
                "still": base
            });
    }

    spacing: Kirigami.Units.smallSpacing
    Component.onCompleted: refresh()

    Connections {
        function onViewChanged() {
            publish.refresh();
        }

        target: publish.page.editor
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        level: 4
        text: App.tr("Publish as a preset")
    }

    Repeater {
        model: publish.report.steps

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true

            Kirigami.Icon {
                source: row.modelData.done ? "dialog-ok-apply" : "dialog-warning"
                Layout.preferredWidth: Kirigami.Units.iconSizes.small
                Layout.preferredHeight: Kirigami.Units.iconSizes.small
            }

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: publish.names[row.modelData.id] ?? row.modelData.id
            }

            QQC2.Button {
                visible: row.modelData.id === "title" || row.modelData.id === "license"
                text: App.tr("Edit…")
                icon.name: "document-edit"
                onClicked: publish.page.editor.select("/manifest")
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true
        visible: publish.report.steps.length > 0

        QQC2.Button {
            text: App.tr("Capture the thumbnail")
            icon.name: "camera-photo"
            enabled: !publish.page.editor.busy && publish.basePath !== ""
            onClicked: publish.shot(false)
            QQC2.ToolTip.text: publish.basePath === "" ? App.tr("Install the base package to draw the preset over it.") : App.tr("Draw the base package with the values of this preset and keep that frame")
            QQC2.ToolTip.visible: hovered
        }

        QQC2.Button {
            text: App.tr("Record a preview")
            icon.name: "media-record"
            enabled: !publish.page.editor.busy && publish.basePath !== ""
            onClicked: publish.shot(true)
        }

        QQC2.CheckBox {
            id: asWebp

            text: App.tr("WebP")
            Accessible.name: App.tr("Record as WebP instead of GIF")
        }

        QQC2.Button {
            text: App.tr("Choose a picture…")
            icon.name: "document-open"
            onClicked: pictures.open()
        }
    }

    QQC2.Button {
        text: App.tr("Publish…")
        icon.name: "document-send"
        enabled: publish.report.ready && !publish.page.editor.busy
        onClicked: publish.page.builder.open()
        QQC2.ToolTip.text: publish.report.ready ? App.tr("Build the package, then add it to a repository and publish it") : App.tr("Finish the steps above first")
        QQC2.ToolTip.visible: hovered
    }

    FileDialog {
        id: pictures

        nameFilters: [App.tr("Pictures") + " (*.png *.jpg *.jpeg *.webp *.gif)"]
        onAccepted: publish.page.run("thumbnail", {
            "file": App.path(selectedFile),
            "animated": false
        })
    }
}
