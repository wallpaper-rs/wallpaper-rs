import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Page {
    id: page

    property var account: null
    property string error
    property string code
    property string address
    property string failure
    property string failureDetail
    property string lastName
    property bool lastPlain
    property bool plain: false
    property string level
    readonly property alias filer: filer
    readonly property alias panels: panels
    readonly property alias secrets: secrets
    readonly property alias codes: codes
    readonly property alias revoker: revoker
    readonly property alias shown: shown
    readonly property string phase: account ? account.state : ""
    readonly property bool keyed: ["ready", "paused", "error"].includes(phase)
    readonly property bool linked: !!account && !!account.device
    readonly property var levels: ({
            "server": App.tr("Server"),
            "devices": App.tr("Devices of the account"),
            "repos": App.tr("Repositories of your other devices"),
            "more": App.tr("More"),
            "details": App.tr("Details")
        })
    property var route: level === "" ? [] : [App.tr("Account"), levels[level]]
    property var keyHints: [{
            "keys": ["up", "down"],
            "text": App.tr("Move")
        }, {
            "keys": ["left", "right"],
            "text": App.tr("Change")
        }, {
            "keys": ["Intro"],
            "text": App.tr("Open")
        }]

    function load() {
        App.query("GetStatus", [], "Sync1", reply => {
            if (!page)
                return;
            page.account = reply.account;
            page.error = "";
        }, error => page && (page.error = error.message));
    }

    function call(member, args) {
        App.query(member, args, "Sync1", reply => {
            if (!page)
                return;
            page.failure = "";
            page.code = reply[0];
            page.address = reply[1];
        }, error => {
            if (!page)
                return;
            page.failure = error.name.endsWith("KeyringUnavailable") ? "keyring" : "other";
            page.failureDetail = error.message;
        });
    }

    function link(name, plaintext) {
        lastName = name;
        lastPlain = plaintext;
        call("Connect", [name]);
    }

    function relink() {
        page.link(page.lastName, page.lastPlain);
    }

    function open(name) {
        level = name;
        Qt.callLater(() => page && shown.item && shown.item.enter && shown.item.enter());
    }

    function enter() {
        if (shown.item && shown.item.enter)
            shown.item.enter();
    }

    function routeBack() {
        const from = level;
        level = "";
        Qt.callLater(() => page && shown.item && shown.item.enter && shown.item.enter(from));
    }

    function exercise() {
        Qt.createComponent("AccountCheck.qml").createObject(page, {
            "page": page
        }).run();
    }

    title: App.tr("Account")
    padding: 0
    background: null
    actions: [
        Kirigami.Action {
            text: App.tr("Sync now")
            icon.name: "arrows-clockwise"
            visible: page.keyed && !(page.account && page.account.paused)
            onTriggered: App.act("SyncNow", [], "Sync1")
        }
    ]
    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "account" || topic === "jobs")
                page.load();
        }
    }

    Connections {
        target: page.Window.window

        function onActiveFocusItemChanged() {
            const item = page.Window.activeFocusItem;
            if (!item || !page.contains(item.mapToItem(page, 0, 0)))
                return;
            const top = item.mapToItem(flick.contentItem, 0, 0).y;
            if (top < flick.contentY)
                flick.contentY = Math.max(0, top - Look.e4);
            else if (top + item.height > flick.contentY + flick.height)
                flick.contentY = Math.min(flick.contentHeight - flick.height, top + item.height - flick.height + Look.e4);
        }
    }

    Flickable {
        id: flick

        anchors.fill: parent
        clip: true
        contentWidth: width
        contentHeight: shown.height + Look.e4
        boundsBehavior: Flickable.StopAtBounds
        QQC2.ScrollBar.vertical: QQC2.ScrollBar {}

        Loader {
            id: shown

            width: flick.width
            source: ({
                    "": "AccountMain.qml",
                    "server": "AccountServer.qml",
                    "devices": "AccountDevices.qml",
                    "repos": "AccountShared.qml",
                    "more": "AccountMore.qml",
                    "details": "AccountSummary.qml"
                })[page.level]
            onLoaded: item.page = page
        }
    }

    AccountPanels {
        id: panels

        page: page
    }

    AccountSecrets {
        id: secrets

        page: page
    }

    SecretFiler {
        id: filer

        onChosen: page.relink()
    }

    AccountRevoke {
        id: revoker
    }

    AccountCodes {
        id: codes

        account: page.account
    }
}
