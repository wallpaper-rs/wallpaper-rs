import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.ScrollablePage {
    id: page

    property var scenes: []
    property var projects: JSON.parse(helper.projects() || "[]")
    readonly property var hints: ({
            "image": App.tr("One still picture as the wallpaper: you choose it after creating the package. To animate or add effects, start from an empty scene and add the picture as a layer."),
            "video": App.tr("One looping video as the wallpaper: you choose it after creating the package.")
        })
    readonly property var starters: [["empty", "document-new"], ["particles", "weather-snow"], ["parallax", "view-list-details"], ["clock", "clock"], ["web", "text-html"], ["preset", "color-management"], ["image", "image-x-generic"], ["video", "video-x-generic"], ["assets", "package-x-generic"]]

    function exercise() {
        if (page.scenes.length === 0)
            throw new Error("the scenes page offers no installed package to copy");
        for (const part of [["page", "web"], ["clip", "video"], ["still", "image"], ["preset", "preset"]])
            if (App.backend.claim("compose." + part[0]))
                App.compose(helper.folderFor(part[0]), part[1], "");
        App.compose(helper.folderFor("check"), "empty", "");
    }

    function make(starter) {
        App.compose(folder.typed ? folder.text : helper.folderFor(name.text), starter, name.text);
    }

    function edit(path) {
        copying.start(path);
    }

    function copy(uid) {
        App.query("GetItem", [uid], "Library1", reply => page && page.edit(reply.item.path));
    }

    title: App.tr("Scenes")
    Component.onCompleted: {
        App.query("Search", ["kind:package", 0, 500], "Library1", reply => {
            if (!page)
                return;
            page.scenes = (reply?.items ?? []).filter(item => item.package && !item.missing);
        });
    }

    SceneEditor {
        id: helper
    }

    ItemCopy {
        id: copying
    }

    FolderDialog {
        id: folders

        onAccepted: App.compose(App.path(selectedFolder), "", "")
    }

    ColumnLayout {
        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: name

                Kirigami.FormData.label: App.tr("Title:")
                text: App.tr("My scene")
            }

            QQC2.TextField {
                id: folder

                property bool typed: false

                Kirigami.FormData.label: App.tr("Folder:")
                Layout.fillWidth: true
                text: helper.folderFor(name.text)
                onTextEdited: typed = true
            }
        }

        Flow {
            Layout.fillWidth: true
            spacing: Kirigami.Units.largeSpacing

            SceneRepeater {
                id: listed3

                rows: page.starters

                delegate: QQC2.Button {
                    required property int index
                    readonly property var modelData: listed3.rows[index]

                    text: SceneWords.kind(modelData[0])
                    icon.name: modelData[1]
                    display: QQC2.AbstractButton.TextUnderIcon
                    implicitWidth: Kirigami.Units.gridUnit * 9
                    implicitHeight: Kirigami.Units.gridUnit * 5
                    QQC2.ToolTip.visible: hovered && (page.hints[modelData[0]] ?? "") !== ""
                    QQC2.ToolTip.text: page.hints[modelData[0]] ?? ""
                    onClicked: page.make(modelData[0])
                }
            }
        }

        Kirigami.Heading {
            level: 3
            text: App.tr("Open")
        }

        QQC2.Button {
            text: App.tr("Open a scene or package folder…")
            icon.name: "document-open-folder"
            onClicked: folders.open()
        }

        SceneRepeater {
            id: listed2

            rows: page.projects

            delegate: QQC2.ItemDelegate {
                required property int index
                readonly property var modelData: listed2.rows[index]

                Layout.fillWidth: true
                focusPolicy: Qt.StrongFocus
                text: (modelData.title || "") + " — " + modelData.folder
                icon.name: "folder"
                onClicked: App.compose(modelData.folder, "", "")
            }
        }

        Kirigami.Heading {
            level: 3
            text: App.tr("Edit a copy of a package from the library")
            visible: page.scenes.length > 0
        }

        SceneRepeater {
            id: listed

            rows: page.scenes

            delegate: QQC2.ItemDelegate {
                required property int index
                readonly property var modelData: listed.rows[index]

                Layout.fillWidth: true
                focusPolicy: Qt.StrongFocus
                text: modelData.title
                icon.name: "edit-copy"
                onClicked: page.copy(modelData.uid)
            }
        }
    }
}
