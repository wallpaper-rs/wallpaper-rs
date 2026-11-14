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
    property alias tabs: panel.tabs
    property alias builder: builder
    property alias use: use
    property alias settings: panel.settings
    property alias layers: panel.layers
    property string spot
    property var picked: []
    property bool moving
    property bool overwrite
    property bool magnets: true
    property string picking
    property var brush: null
    property var emitterEntry: ({})
    property bool lightBoxes
    readonly property SceneMeshState mesh: SceneMeshState {
        page: page
    }
    property string notice
    readonly property var scenePage: page
    property Component bar: Component {
        SceneTop {
            page: scenePage
        }
    }
    property real clock: editor.time
    property real heard: 0
    readonly property var view: JSON.parse(editor.view || "{}")
    onViewChanged: SceneWords.own = view.effect_names ?? ({})
    readonly property var fields: JSON.parse(editor.fields || "[]")

    function run(action, args, gesture) {
        editor.run(action, JSON.stringify(args ?? {}), gesture === true);
    }

    function change(pointer, value, gesture) {
        const keyed = (view.tracks ?? []).some(track => track.pointer === pointer);
        const args = keyed ? {
            "time": editor.time
        } : {};
        run(keyed ? "keyframe" : "set", Object.assign(args, {
                    "pointer": pointer,
                    "value": value
                }), gesture);
    }

    function jump(pointer) {
        const own = (view.layers ?? []).concat(view.properties ?? [], view.presets ?? []).some(item => item.pointer === pointer);
        const parent = pointer.split("/").slice(0, -1).join("/").replace(/\/(params|values)$/, "");
        editor.select(own || parent.split("/").length < 3 ? pointer : parent);
        spot = pointer;
    }

    title: view.title ?? ""
    padding: 0
    background: Rectangle {
        color: Look.noche
    }
    Component.onCompleted: {
        starter ? editor.create(folder, starter, heading) : editor.open(folder, source);
        if (App.checking)
            Qt.createComponent("SceneCheck.qml").createObject(page, {
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

    Connections {
        target: page.Window.window

        function onVisibilityChanged(visibility) {
            editor.cover(visibility === Window.Minimized || visibility === Window.Hidden);
        }
    }

    Item {
        anchors.fill: parent
        enabled: editor.ready

        SceneSide {
            id: panel

            width: Look.lateralIzq
            height: parent.height
            page: page
        }

        SceneStage {
            x: Look.lateralIzq
            width: parent.width - Look.lateralIzq - Look.lateralDer
            height: parent.height - foot.height
            page: page
        }

        SceneFoot {
            id: foot

            x: Look.lateralIzq
            y: parent.height - height
            width: parent.width - Look.lateralIzq - Look.lateralDer
            height: foot.wanted
            page: page
        }

        SceneFields {
            x: parent.width - width
            width: Look.lateralDer
            height: parent.height
            page: page
        }
    }

    SceneBuild {
        id: builder

        page: page
    }
}
