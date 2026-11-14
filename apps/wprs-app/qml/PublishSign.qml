import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import io.github.wallpaper_rs.app

PublishStep {
    id: sign

    required property var publish
    property var found: []
    property string chosen: ""
    property string made: ""
    property bool wrong: false
    readonly property var current: found.find(k => k.secret === chosen) ?? (chosen !== "" ? picked(chosen) : found[0] ?? null)
    readonly property bool first: found.length === 0 && chosen === ""
    readonly property string keyName: current ? current.name : ""
    readonly property bool automatic: current !== null && current.encrypted === false && !publish.unsigned
    readonly property string secret: publish.unsigned || current === null ? "" : current.secret
    readonly property string word: current !== null && current.encrypted !== false ? unlock.pass.text : ""
    readonly property bool copied: current !== null && JSON.parse(publish.memory.copied || "[]").includes(current.fingerprint)
    readonly property alias backend: backend
    readonly property alias unlock: unlock

    function picked(path) {
        return {
            "name": Publishing.base(path).replace(/\.key$/, ""),
            "secret": path,
            "encrypted": true,
            "fingerprint": ""
        };
    }

    function refresh() {
        found = JSON.parse(backend.keys(backend.folder()) || "[]");
    }

    function confirm() {
        wrong = current.encrypted !== false && !backend.unlocks(current.secret, unlock.pass.text);
        if (wrong)
            return;
        publish.unsigned = false;
        publish.advance(3);
    }

    function skip() {
        wrong = false;
        publish.unsigned = true;
        publish.advance(3);
    }

    function keep(folder) {
        const reply = JSON.parse(backend.keep(current.secret, folder) || "{}");
        if (reply.error !== undefined) {
            applicationWindow().showPassiveNotification(reply.error, "long");
            return;
        }
        const list = JSON.parse(publish.memory.copied || "[]");
        publish.memory.copied = JSON.stringify(list.concat([current.fingerprint]));
        applicationWindow().showPassiveNotification(App.tr("A copy of your signature “%1” is in “%2”. Keep it somewhere safe.", current.name, Publishing.base(folder)), "long");
    }

    function create(name, password) {
        made = name;
        unlock.pass.text = password;
        backend.create(backend.folder(), name, password, false);
    }

    function manage() {
        keys.open();
    }

    function copyAway() {
        copies.open();
    }

    function choose() {
        files.open();
    }

    function take(operation, answer) {
        const reply = JSON.parse(answer);
        if (reply.error !== undefined) {
            applicationWindow().showPassiveNotification(reply.error, "long");
            return;
        }
        if (operation !== "create")
            return;
        made = reply.name;
        refresh();
        chosen = reply.secret;
        applicationWindow().showPassiveNotification(App.tr("Your signature “%1” was created. Save a copy: without it you cannot publish as %1 from another computer.", reply.name), "long", App.tr("Save a copy…"), () => sign && copies.open());
        publish.unsigned = false;
        publish.advance(3);
    }

    number: 2
    title: first ? App.tr("Your signature") : App.tr("Sign it")
    done: publish.unsigned ? App.tr("Published <b>without a signature</b>") : automatic ? App.tr("Signed only with <b>%1</b>: it has no password", Publishing.escaped(keyName)) : App.tr("Signed with your key <b>%1</b>", Publishing.escaped(keyName))
    mode: publish.at === 2 ? "abierto" : publish.at > 2 ? "hecho" : "cerrado"
    changeable: publish.result === null
    onChange: publish.advance(2)

    after: PublishCopy {
        Layout.fillWidth: true
        visible: sign.mode === "hecho" && sign.current !== null && !sign.publish.unsigned && !sign.copied
        onSave: copies.open()
    }

    Signing {
        id: backend

        onAnswered: (operation, answer) => sign.take(operation, answer)
    }

    PublishFirst {
        Layout.fillWidth: true
        visible: sign.first
        sign: sign
    }

    PublishUnlock {
        id: unlock

        Layout.fillWidth: true
        visible: !sign.first
        sign: sign
    }

    SceneKeys {
        id: keys

        onClosed: sign.refresh()
        onPicked: secret => sign.chosen = secret
    }

    FolderDialog {
        id: copies

        title: App.tr("Where to keep a copy of your signature")
        onAccepted: sign.keep(App.path(selectedFolder))
    }

    FileDialog {
        id: files

        nameFilters: [App.tr("Author keys") + " (*.key)"]
        onAccepted: sign.chosen = App.path(selectedFile)
    }
}
