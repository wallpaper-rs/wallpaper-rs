import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.ScrollablePage {
    id: page

    property var account: null
    property string error
    property string code
    property string address
    property string failure
    property string failureDetail
    property string lastName
    property bool lastPlain
    readonly property alias filer: filer
    readonly property alias dialogs: dialogs
    readonly property string phase: account?.state ?? ""
    readonly property bool keyed: ["ready", "paused", "error"].includes(phase)

    function load() {
        App.query("GetStatus", [], "Sync1", reply => {
            if (!page)
                return;
            page.account = reply.account;
            page.error = "";
        }, error => page && (page.error = error.message));
    }

    function link(name, plaintext) {
        lastName = name;
        lastPlain = plaintext;
        App.query("Link", [name, plaintext], "Sync1", reply => {
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

    function exercise() {
        dialogs.linker.open();
        page.failure = "keyring";
        if (!problem.visible)
            throw new Error("a link that failed for lack of a keyring shows no notice on the account page");
        filer.open();
        filer.close();
        page.failure = "";
        const kept = page.account;
        page.account = Object.assign({}, kept, {
            "state": "signed_out"
        });
        if (!problem.signedOut)
            throw new Error("a computer the server no longer accepts shows no notice on the account page");
        page.account = kept;
        revoker.device = {
            "id": "0",
            "name": "Device 2"
        };
        revoker.open();
        revoker.close();
    }

    title: App.tr("Account")
    Component.onCompleted: load()
    actions: tools.list

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "account" || topic === "jobs")
                page.load();
        }
    }

    ColumnLayout {
        Kirigami.PlaceholderMessage {
            Layout.fillWidth: true
            visible: !page.account
            icon.name: page.error ? "data-error" : "folder-cloud"
            text: page.error || App.tr("No server yet")
            explanation: page.error ? "" : App.tr("A wallpaper-rs server keeps your library, playlists and settings in sync between your computers, encrypted end to end.")
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: page.code !== "" && page.phase === "linking"
            type: Kirigami.MessageType.Information
            text: App.tr("Open %1 and enter the code %2, or approve it on a linked computer.", page.address, page.code)
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: page.account?.encrypted === false
            type: Kirigami.MessageType.Warning
            text: App.tr("Sync is not encrypted: the operator of the server can read your library, playlists and settings.")
        }

        AccountProblem {
            id: problem

            Layout.fillWidth: true
            page: page
        }

        AccountSummary {
            Layout.fillWidth: true
            visible: !!page.account
            account: page.account ?? {}
        }

        Repeater {
            model: page.account?.devices ?? []

            delegate: AccountDevice {
                required property var modelData

                Layout.fillWidth: true
                device: modelData
                keyed: page.keyed
                joining: page.phase === "needs_key"
                pairing: page.account?.pairing ?? null
                onShare: codes.share(modelData)
                onRevoke: {
                    revoker.device = modelData;
                    revoker.open();
                }
            }
        }

        AccountShared {
            Layout.fillWidth: true
            visible: !!page.account?.device
            account: page.account
            dialogs: dialogs
        }

        AccountFiles {
            Layout.fillWidth: true
            account: page.account
        }

        AccountUploads {
            Layout.fillWidth: true
            visible: !!page.account && items.length > 0
            account: page.account
        }

        AccountSessions {
            Layout.fillWidth: true
            account: page.account
        }

        AccountKeys {
            Layout.fillWidth: true
            visible: !!page.account
            account: page.account
        }
    }

    AccountDialogs {
        id: dialogs

        page: page
    }

    AccountActions {
        id: tools

        page: page
        dialogs: dialogs
        codes: codes
    }

    SecretFiler {
        id: filer

        onChosen: page.link(page.lastName, page.lastPlain)
    }

    AccountRevoke {
        id: revoker
    }

    AccountCodes {
        id: codes

        account: page.account
    }
}
