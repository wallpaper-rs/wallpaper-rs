import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host
    property Timer later: Timer {
        interval: 200
        onTriggered: check.changed()
    }

    function same(found, wanted, what) {
        if (!Qt.colorEqual(found, wanted))
            throw new Error("look: " + what + " is " + found + " instead of " + wanted);
    }

    function tones(what) {
        same(host.palette.window, Look.noche, what + " window");
        same(host.palette.windowText, Look.escarcha, what + " text");
        same(host.palette.button, Look.nocheAlta, what + " button");
        same(host.palette.highlight, Look.luna, what + " highlight");
        same(host.Kirigami.Theme.backgroundColor, Look.noche, what + " Kirigami background");
        same(host.Kirigami.Theme.textColor, Look.escarcha, what + " Kirigami text");
        same(host.Kirigami.Theme.negativeTextColor, Look.coral, what + " Kirigami negative text");
    }

    function controls() {
        const made = Qt.createQmlObject("import QtQuick\nimport QtQuick.Controls\nItem { Button { text: \"a\" } Button { text: \"b\"; highlighted: true } Button { text: \"c\"; flat: true } Switch { text: \"d\" } }", host, "LookControls");
        const quiet = made.children[0];
        const principal = made.children[1];
        const word = made.children[2];
        const key = made.children[3];
        same(quiet.background.color, Look.nocheAlta, "quiet button");
        same(principal.background.color, Look.luna, "principal button");
        if (principal.font.family !== Look.letraTitulo || word.background.color.a !== 0)
            throw new Error("look: the principal button is not in the title font or the word button has a surface");
        if (key.indicator.implicitWidth !== Look.llaveAncho)
            throw new Error("look: the switch is not drawn by the own style");
        made.destroy();
    }

    function exercise() {
        if (Look.bridge.style() !== "io.github.wallpaper_rs.style")
            throw new Error("look: the controls style is " + Look.bridge.style());
        if (Look.bridge.family() !== Look.letraTexto || !Look.bridge.hasFamily(Look.letraTitulo))
            throw new Error("look: the text font is " + Look.bridge.family());
        const missing = Look.bridge.iconNames().split(",").filter(name => !Look.bridge.hasIcon(name));
        if (missing.length > 0)
            throw new Error("look: icons the own set does not resolve: " + missing.join(" "));
        if (Look.bridge.hasIcon("go-home") || Look.bridge.hasIcon("user-trash"))
            throw new Error("look: an icon outside the own set resolved from the system");
        tones("start");
        controls();
        Look.tone({
            "noche": "#1c1003",
            "nocheAlta": "#2a1b09",
            "luna": "#f7afa7"
        });
        later.start();
    }

    function changed() {
        tones("after a change");
        Look.tone({
            "noche": "#0a1322",
            "nocheAlta": "#121e33",
            "luna": "#9cc3f5"
        });
    }
}
