import QtQuick
import io.github.wallpaper_rs.app

Column {
    id: notices

    required property var host
    property string text: ""
    property string tone: ""
    property string action: ""
    property var callback: null
    property string dismissed: ""
    readonly property string banner: App.backend.banner
    readonly property bool failing: banner !== "" && banner !== dismissed
    readonly property bool busy: failing || account.visible

    function show(message, timeout, actionText, onAction) {
        const notice = message !== null && typeof message === "object" ? message : {
            "text": message
        };
        text = notice.text ?? "";
        tone = notice.tone ?? "";
        action = actionText ?? notice.action ?? "";
        callback = onAction ?? notice.undo ?? null;
        later.interval = typeof timeout === "number" && timeout > 0 ? timeout : 7000;
        later.restart();
    }

    function hide() {
        later.stop();
        text = "";
        callback = null;
    }

    width: Look.avisoAncho
    spacing: Look.e2
    Accessible.role: Accessible.StaticText
    onBannerChanged: dismissed = banner === "" ? "" : dismissed

    Timer {
        id: later

        onTriggered: notices.hide()
    }

    Aviso {
        visible: notices.failing
        width: Math.min(notices.width, implicitWidth)
        text: notices.banner
        tone: "coral"
        onClosed: notices.dismissed = notices.banner
    }

    AccountNotice {
        id: account

        width: visible ? parent.width : Look.avisoAncho
        host: notices.host
        visible: phase === "signed_out" && !notices.failing
    }

    Aviso {
        id: passive

        visible: notices.text !== "" && !notices.busy
        width: Math.min(notices.width, implicitWidth)
        text: notices.text
        tone: notices.tone
        action: notices.action
        onHeldChanged: held ? later.stop() : notices.text !== "" && later.restart()
        onActed: {
            const run = notices.callback;
            notices.hide();
            if (run)
                run();
        }
        onClosed: notices.hide()
    }
}
