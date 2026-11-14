import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    required property string folder
    property string starter
    property string heading
    property string source
    property alias editor: editor
    property alias builder: builder
    property alias use: use
    property alias tabs: side
    property string spot
    property string notice
    property real clock: 0
    readonly property var view: JSON.parse(editor.view || "{}")
    readonly property var fields: JSON.parse(editor.fields || "[]")
    readonly property var manifest: page.view.id === undefined ? ({}) : JSON.parse(editor.value("/manifest") || "{}")
    readonly property string kind: view.kind ?? ""
    readonly property bool viewable: ["web", "image", "video"].includes(kind)
    readonly property var packagePage: page
    property Component topBar: builder.shown ? null : packageTop
    property Component packageTop: Component {
        SceneTop {
            page: packagePage
        }
    }

    function run(action, args, gesture) {
        editor.run(action, JSON.stringify(args ?? {}), gesture === true);
    }

    function change(pointer, value, gesture) {
        run("set", {
            "pointer": pointer,
            "value": value
        }, gesture);
    }

    function manifestEdit(what, args, gesture) {
        const edit = {};
        edit[what] = args;
        run("manifest", edit, gesture);
    }

    function jump(pointer) {
        editor.select(pointer);
        spot = pointer;
    }

    title: builder.shown ? builder.title : view.title ?? ""
    padding: 0
    actions: builder.shown ? builder.actions : []
    Component.onCompleted: {
        if (starter) {
            editor.create(folder, starter, heading);
            const media = App.takeMedia();
            if (media)
                manifestEdit("media", {
                        "file": media
                    });
        } else
            editor.open(folder, source);
        editor.select("/manifest");
        if (App.checking)
            Qt.createComponent("PackageCheck.qml").createObject(page, {
                "page": page
            }).exercise();
    }

    SceneEditor {
        id: editor

        onFinished: message => use.placing ? use.placed(message) : page.notice = message
    }

    SceneUse {
        id: use

        page: page
    }

    Timer {
        interval: 30000
        running: editor.ready
        repeat: true
        onTriggered: editor.autosave()
    }

    header: SceneNotes {
        page: page
    }

    RowLayout {
        anchors.fill: parent
        enabled: editor.ready
        spacing: 0

        ColumnLayout {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 14
            Layout.fillWidth: false
            Layout.fillHeight: true

            QQC2.TabBar {
                id: side

                Layout.fillWidth: true

                QQC2.TabButton {
                    text: App.tr("Files")
                }

                QQC2.TabButton {
                    text: App.tr("Player")
                }
            }

            StackLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                currentIndex: side.currentIndex

                PackageFiles {
                    page: page
                }

                QQC2.ScrollView {
                    contentWidth: availableWidth
                    clip: true

                    PackageMedia {
                        width: parent.width
                        page: page
                    }
                }
            }
        }

        PackageMiddle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredWidth: Kirigami.Units.gridUnit
            page: page
        }

        ColumnLayout {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 16
            Layout.fillWidth: false
            Layout.fillHeight: true

            PackageInspector {
                Layout.fillWidth: true
                Layout.fillHeight: true
                page: page
            }

            SceneProblems {
                Layout.fillWidth: true
                Layout.fillHeight: false
                Layout.preferredHeight: Kirigami.Units.gridUnit * ((page.view.problems ?? []).length ? 7 : 2)
                page: page
            }
        }
    }

    Publish {
        id: builder

        page: page
    }
}
