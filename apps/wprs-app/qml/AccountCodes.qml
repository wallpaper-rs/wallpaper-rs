import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Item {
    id: codes

    property var account: null
    property var device: ({})
    property int lookup: 0
    property int comparing: 0
    readonly property var found: App.jobs.find(j => j.id === codes.lookup) || null
    readonly property bool known: !!found && found.state === "done"
    readonly property var compared: App.jobs.find(j => j.id === codes.comparing) || null
    readonly property var pairing: codes.account ? codes.account.pairing : null
    readonly property string shown: codes.pairing && codes.pairing.device === codes.device.id ? (codes.pairing.code || "") : ""
    readonly property string other: codes.device.label || codes.device.name || ""
    readonly property alias approver: approver
    readonly property alias sharer: sharer

    function approve(opener) {
        approver.opener = opener;
        approver.open();
    }

    function share(target, opener) {
        codes.device = target;
        App.query("ShareKey", [target.id], "Sync1", reply => codes && (codes.comparing = reply[0]));
        sharer.opener = opener;
        sharer.open();
    }

    function resume(opener) {
        const id = codes.pairing ? codes.pairing.device : "";
        const devices = codes.account ? codes.account.devices : [];
        codes.device = devices.find(d => d.id === id) || {
            "id": id,
            "name": ""
        };
        sharer.opener = opener;
        sharer.open();
    }

    PanelChico {
        id: approver

        title: App.tr("Approve another device")
        text: App.tr("Approve only a code that your own new device shows on its screen. Someone who sends you a code is trying to get into your account.")
        quiet: App.tr("Cancel")
        onClosed: {
            codes.lookup = 0;
            approval.text = "";
            password.text = "";
        }

        QQC2.TextField {
            id: approval

            width: approver.width - 2 * Look.e4
            readOnly: codes.lookup !== 0
            placeholderText: "BCDF-GHJK"
            Accessible.name: App.tr("Code shown by the new device")
        }

        Nota {
            width: approver.width - 2 * Look.e4
            visible: codes.known
            text: codes.known ? App.tr("A device on %1 running %2 asked for this code at %3.", codes.found.output.platform || "", codes.found.output.client || App.tr("unnamed software"), App.time(Math.floor((codes.found.output.requested_at || 0) / 1000))) : ""
        }

        Nota {
            width: approver.width - 2 * Look.e4
            visible: !!codes.found && codes.found.state === "failed"
            tone: "coral"
            text: codes.found && codes.found.error ? codes.found.error.message : ""
        }

        QQC2.TextField {
            id: password

            width: approver.width - 2 * Look.e4
            visible: codes.known
            echoMode: TextInput.Password
            placeholderText: App.tr("Password of the account")
            Accessible.name: placeholderText
        }

        QQC2.Button {
            highlighted: true
            visible: codes.lookup === 0
            enabled: approval.text.trim() !== ""
            text: App.tr("Look up")
            icon.name: "magnifying-glass"
            onClicked: App.query("PreviewCode", [approval.text.trim()], "Sync1", reply => codes && (codes.lookup = reply[0]))
        }

        QQC2.Button {
            highlighted: true
            visible: codes.known
            enabled: password.text.length > 0
            text: App.tr("This is my device: approve")
            icon.name: "check"
            onClicked: {
                App.passphrase("ApproveDevice", approval.text.trim() + "\n" + password.text);
                approver.close();
            }
        }
    }

    PanelChico {
        id: sharer

        title: App.tr("Let %1 sync", codes.other || App.tr("another device"))
        text: codes.shown ? App.tr("This computer shows %1. Check that both screens show the same number, then type the one that %2 shows: the key leaves only when both match.", codes.shown, codes.other) : codes.compared && codes.compared.state === "failed" ? codes.compared.error.message : App.tr("Waiting for %1: choose this computer there to get the key.", codes.other)
        quiet: App.tr("Cancel")
        onClosed: digits.text = ""
        onRejected: {
            if (codes.compared && !codes.compared.finished_at)
                App.act("CancelJob", [codes.comparing]);
        }

        QQC2.TextField {
            id: digits

            width: sharer.width - 2 * Look.e4
            visible: codes.shown !== ""
            placeholderText: App.tr("The 6 digits shown on that device")
            Accessible.name: placeholderText
        }

        QQC2.Button {
            highlighted: true
            visible: codes.shown !== ""
            enabled: digits.text.trim() !== ""
            text: codes.pairing && codes.pairing.sharing ? App.tr("Send the key") : App.tr("Take the key")
            icon.name: "check"
            onClicked: {
                App.act("ConfirmShare", [codes.device.id, digits.text], "Sync1");
                sharer.close();
            }
        }
    }
}
