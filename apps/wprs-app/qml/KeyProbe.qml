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

    readonly property var drawer: host.globalDrawer

    function begin() {
        if (!finished)
            start();
    }

    function entries() {
        return walk.gather(drawer.contentItem, []).filter(item => item.tAction !== undefined && item.down !== undefined && walk.shown(item));
    }

    function menu(mode) {
        const seen = walk.tabs(drawer.modal);
        const found = entries();
        const actions = Array.from(new Set(found.map(item => item.tAction)));
        const lost = actions.filter(action => !found.some(item => item.tAction === action && seen.includes(item)));
        if (actions.length !== drawer.actions.length)
            throw new Error("the " + mode + " side menu shows " + actions.length + " of its " + drawer.actions.length + " entries");
        if (lost.length > 0)
            throw new Error("the Tab key does not reach the " + mode + " side menu entries " + lost.map(action => action.text).join(", "));
        const pages = [];
        for (const action of actions) {
            found.find(item => item.tAction === action && seen.includes(item)).forceActiveFocus(Qt.TabFocusReason);
            walk.press(Qt.Key_Return, 0);
            pages.push(host.current);
        }
        const all = pages.slice().sort((a, b) => a - b).join();
        if (all !== Array.from({
                "length": host.pages.length
            }, (v, at) => at).join())
            throw new Error("the Enter key on the " + mode + " side menu entries opened the pages " + pages.join(", "));
        found[0].forceActiveFocus(Qt.TabFocusReason);
        walk.press(Qt.Key_Escape, 0);
        if (drawer.opened && !drawer.collapsed)
            throw new Error("the Escape key does not close the " + mode + " side menu");
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
            drawer.open();
            return ++stage;
        }
        if (stage === 1) {
            if (!drawer.opened && ++waited < 20)
                return;
            menu("narrow");
            width = host.width;
            host.width = width + Kirigami.Units.gridUnit * 80;
            waited = 0;
            return ++stage;
        }
        if (stage === 2) {
            if (drawer.modal && ++waited < 20)
                return;
            drawer.collapsed = false;
            drawer.open();
            waited = 0;
            return ++stage;
        }
        if (stage === 3) {
            if (!drawer.opened && ++waited < 20)
                return;
            menu("wide");
            scratch = Qt.createComponent("KeyScratch.qml").createObject(host.overlay);
            return ++stage;
        }
        if (stage === 4) {
            turns();
            return ++stage;
        }
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
