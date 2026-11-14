import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Timer {
    id: probe

    required property var host
    required property KeyWalk walk
    property int stage: 0
    property int waited: 0
    property bool finished: !App.backend.claim("keys.menu")
    property var scratch: null
    property real top: 0
    property real width: 0

    readonly property var words: host.shell.menu

    function begin() {
        if (!finished)
            start();
    }

    function pick(at) {
        words.forceActiveFocus(Qt.TabFocusReason);
        for (let n = 0; n < words.all.length; n++)
            walk.press(Qt.Key_Up, 0);
        for (let n = 0; n < at; n++)
            walk.press(Qt.Key_Down, 0);
        walk.press(Qt.Key_Return, 0);
        return host.current;
    }

    function menu() {
        if (!walk.tabs(false).includes(words))
            throw new Error("the Tab key does not reach the menu");
        const pages = words.all.map((entry, at) => pick(at));
        host.showPage(5);
        const section = walk.gather(host.shell, []).find(item => item.label === App.tr("Tabs") && walk.shown(item));
        if (!section)
            throw new Error("the settings do not show their tabs");
        section.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Right, 0);
        walk.press(Qt.Key_Return, 0);
        const layouts = walk.gather(host.shell, []).find(item => item.kind === "pliega" && item.text === App.tr("Layouts") && walk.shown(item));
        if (!layouts)
            throw new Error("the screens tab does not show its layouts");
        layouts.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Return, 0);
        if (host.shell.optionsFrame.page.place !== "layouts")
            throw new Error("Enter on the layouts does not open them");
        pages.push(2);
        const all = pages.slice().sort((a, b) => a - b).join();
        if (all !== Array.from({
                "length": host.pages.length
            }, (v, at) => at).filter(at => host.names[at] !== "repositories").join())
            throw new Error("the arrow and Enter keys on the menu opened the pages " + pages.join(", "));
        walk.press(Qt.Key_Escape, 0);
        if (!layouts.activeFocus)
            throw new Error("the Escape key does not bring the focus back to the layouts");
        walk.press(Qt.Key_Escape, 0);
        if (!words.activeFocus)
            throw new Error("the Escape key does not bring the focus back to the menu");
        host.showPage(0);
    }

    function wheel(item) {
        const at = item.mapToItem(null, item.width / 2, item.height / 2);
        App.backend.wheel(at.x, at.y, -120);
    }

    function spin(reason, item, property) {
        scratch.forceActiveFocus();
        item.forceActiveFocus(reason);
        const old = item[property];
        wheel(item);
        return item[property] !== old;
    }

    function steps() {
        const pad = scratch.pad;
        const list = scratch.list;
        pad.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Right, 0);
        walk.press(Qt.Key_Return, 0);
        walk.press(Qt.Key_Down, 0);
        if (scratch.landed[0] !== 0.5 || scratch.landed[1] !== 0.5)
            throw new Error("the arrow keys do not move the alignment pad: " + scratch.landed.join(","));
        list.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Down, 0);
        walk.press(Qt.Key_Down, 0);
        if (list.currentIndex !== 2)
            throw new Error("the arrow keys do not move through a list: " + list.currentIndex);
    }

    function turns() {
        const combo = scratch.combo;
        const slider = scratch.slider;
        const button = scratch.button;
        top = scratch.contentY;
        scratch.forceActiveFocus();
        button.forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Return, 0);
        if (scratch.clicks !== 1)
            throw new Error("the Enter key does not press a focused button");
        if (spin(Qt.OtherFocusReason, combo, "currentIndex") || spin(Qt.TabFocusReason, combo, "currentIndex"))
            throw new Error("the mouse wheel changes a drop-down that was not clicked");
        if (spin(Qt.OtherFocusReason, slider, "value") || spin(Qt.TabFocusReason, slider, "value"))
            throw new Error("the mouse wheel changes a slider that was not clicked");
        if (!spin(Qt.MouseFocusReason, combo, "currentIndex"))
            throw new Error("the mouse wheel does not change a drop-down after a click");
        spin(Qt.OtherFocusReason, slider, "value");
    }

    function next() {
        if (stage === 0) {
            menu();
            width = host.width;
            host.width = width + Kirigami.Units.gridUnit * 80;
            return ++stage;
        }
        if (stage < 3)
            return ++stage;
        if (stage === 3) {
            menu();
            scratch = Qt.createComponent("KeyScratch.qml").createObject(host.overlay);
            return ++stage;
        }
        if (stage === 4) {
            turns();
            steps();
            waited = 0;
            return ++stage;
        }
        if (scratch.contentY <= top && ++waited < 20)
            return;
        if (scratch.contentY <= top)
            throw new Error("the mouse wheel does not scroll the page over a drop-down or a slider");
        finish();
    }

    function finish() {
        if (width > 0)
            host.width = width;
        scratch?.destroy();
        stop();
        finished = true;
    }

    interval: 250
    repeat: true
    onTriggered: {
        try {
            next();
        } catch (error) {
            finish();
            throw error;
        }
    }
}
