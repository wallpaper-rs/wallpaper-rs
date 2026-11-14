import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtCore as Core
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Rectangle {
    id: publish

    property var page: null
    property string file: ""
    property bool shown: false
    property int at: 1
    property string key: ""
    property bool unsigned: false
    property string place: memory.place
    property var account: null
    property var result: null
    property var fault: null
    property var manifest: ({})
    property int shots: 0
    readonly property bool editing: page !== null && page.editor !== undefined
    readonly property string name: editing && (page.view.title ?? "") !== "" ? page.view.title : Publishing.title(manifest, file)
    readonly property string version: manifest.version ?? ""
    readonly property string author: (manifest.authors ?? [])[0] ?? ""
    readonly property alias memory: memory
    readonly property alias work: work
    readonly property alias revealing: revealing
    readonly property alias review: review
    readonly property alias sign: sign
    readonly property alias where: where
    readonly property alias done: done
    readonly property bool linked: ["ready", "paused", "error"].includes(account ? account.state : "")
    readonly property string title: App.tr("Publish “%1”", name)
    readonly property list<QtObject> actions: [
        Kirigami.Action {
            text: publish.editing ? App.tr("Back to the editor") : App.tr("Close")
            icon.name: "arrow-left-bold"
            onTriggered: publish.close()
        }
    ]

    function reload() {
        manifest = editing ? JSON.parse(page.editor.value("/manifest") || "{}") : ({});
    }

    function open() {
        if (!shown && result !== null)
            restart();
        reload();
        App.query("GetStatus", [], "Sync1", reply => publish && (publish.account = reply.account ?? null), () => publish && (publish.account = null));
        sign.refresh();
        shown = true;
        review.refresh();
        Qt.callLater(() => publish && publish.focusStep());
    }

    function openWith(path) {
        file = path;
        at = 3;
        open();
    }

    function close() {
        shown = false;
    }

    function restart() {
        result = null;
        fault = null;
        at = file !== "" && !editing ? 3 : 1;
    }

    function advance(step) {
        fault = null;
        at = step === 2 && sign.automatic ? 3 : step;
        Qt.callLater(() => publish && publish.focusStep());
    }

    function focusStep() {
        const target = at === 1 ? review : at === 2 ? sign : at === 3 ? where : done;
        target.forceActiveFocus(Qt.TabFocusReason);
    }

    anchors.fill: parent
    color: Look.noche
    onPlaceChanged: revealing.ask()
    onFaultChanged: revealing.ask()
    onAtChanged: revealing.ask()
    onShownChanged: shown && revealing.ask()
    visible: shown
    z: 20

    Core.Settings {
        id: memory

        property string place: "file"
        property string repos: "[]"
        property string copied: "[]"

        location: Core.StandardPaths.writableLocation(Core.StandardPaths.ConfigLocation) + "/wallpaper-rs/publish.conf"
    }

    PublishReveal {
        id: revealing

        publish: publish
        flick: scroll.contentItem
    }

    PublishWork {
        id: work

        publish: publish
    }

    PublishPicture {
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.right: panel.left
        publish: publish
    }

    Rectangle {
        id: panel

        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        width: Math.min(PublishLook.publicarAncho, parent.width * 0.6)
        color: Look.noche

        QQC2.ScrollView {
            id: scroll

            anchors.fill: parent
            contentWidth: availableWidth

            ColumnLayout {
                width: scroll.availableWidth - 2 * Look.e5
                x: Look.e5
                spacing: Look.e4

                Item {
                    implicitHeight: Look.e5 - Look.e4
                }

                Text {
                    Layout.fillWidth: true
                    visible: publish.at === 1 && publish.editing && !publish.sign.made && publish.sign.found.length === 0
                    text: App.tr("Three steps on this same page. At the end we tell you what to give the other person.")
                    wrapMode: Text.WordWrap
                    font.family: Look.letraTexto
                    font.pixelSize: Look.tExplica
                    color: Look.bruma
                }

                PublishReview {
                    id: review

                    publish: publish
                }

                PublishSign {
                    id: sign

                    publish: publish
                }

                PublishWhere {
                    id: where

                    publish: publish
                }

                PublishDone {
                    id: done

                    publish: publish
                }

                Item {
                    implicitHeight: Look.e6 - Look.e4
                }
            }
        }
    }
}
