import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host
    property var page: null

    function fail(what) {
        throw new Error("pieces: " + what);
    }

    function press(key, modifiers) {
        App.backend.press(key, modifiers ?? 0);
    }

    function named(item, what) {
        if (!item.Accessible.name)
            fail(what + " has no name for assistive tools");
    }

    function rows() {
        const part = page.parts[0];
        page.show(0);
        if (!part.quality.lit)
            fail("the first row does not light up when it takes the focus");
        let chosen = -1;
        part.quality.chosen.connect(index => chosen = index);
        press(Qt.Key_Left);
        if (chosen !== 1)
            fail("the left arrow does not choose the previous value of a row");
        press(Qt.Key_Down);
        const rate = part.quality.nextItemInFocusChain(true);
        press(Qt.Key_Right);
        if (rate.at !== 35 || rate.number !== "35")
            fail("the right arrow does not move the rail of a row");
        press(Qt.Key_Down);
        const pause = rate.nextItemInFocusChain(true);
        press(Qt.Key_Space);
        if (pause.checked || !pause.lit)
            fail("the space key does not turn off the switch of a row");
        for (let i = 0; i < 3; i++)
            press(Qt.Key_Down);
        if (!part.folders.lit)
            fail("the down arrow does not walk the rows");
        press(Qt.Key_Return);
        if (part.folders.open || part.list.visible)
            fail("a fold does not close its sublist");
        press(Qt.Key_Return);
        press(Qt.Key_Down);
        press(Qt.Key_Delete);
        if (part.list.model.length !== 1 || part.folders.number !== "1")
            fail("the delete key does not remove an item of a sublist");
        [part.quality, rate, pause, part.folders].forEach(row => named(row, "a row"));
    }

    function more() {
        const part = page.parts[1];
        page.show(1);
        press(Qt.Key_Return);
        press(Qt.Key_K, Qt.ControlModifier);
        if (part.shortcut.keys.join("+") !== "Ctrl+K" || !part.shortcut.lit)
            fail("a key capture does not keep the combination pressed");
        press(Qt.Key_Down);
        press(Qt.Key_Return);
        if (!part.named.lit || host.activeFocusItem === part.named)
            fail("a text row does not give the focus to its field: " + host.activeFocusItem);
        press(Qt.Key_Escape);
        if (host.activeFocusItem !== part.named)
            fail("the escape key does not return from a text field to its row");
        part.battery.forceActiveFocus();
        press(Qt.Key_Space);
        if (part.battery.checked)
            fail("a row that is off reacts to the keys");
    }

    function said() {
        const notices = page.parts[2].notices;
        page.show(2);
        if (notices.aviso.tone !== "brote" || notices.queue.length !== 1)
            fail("notices are not shown one at a time");
        let acted = null;
        notices.acted.connect(notice => acted = notice);
        notices.aviso.acted();
        if (acted?.tone !== "brote" || notices.aviso.tone !== "ambar")
            fail("the action of a notice does not reach its owner or the next notice does not follow");
        notices.aviso.closed();
        if (notices.shown !== null || notices.aviso.visible)
            fail("closing the last notice leaves it on screen");
    }

    function panels() {
        const part = page.parts[2 + 1];
        page.show(3);
        const menu = part.menu;
        let chosen = -1;
        menu.chosen.connect(index => chosen = index);
        menu.move(1);
        menu.move(1);
        if (menu.current !== 0)
            fail("the menu of a control stops on a separator or on an option that is off");
        menu.choose(4);
        menu.choose(menu.current);
        if (chosen !== 0 || menu.visible)
            fail("choosing an option does not close the menu with that option");
        let accepted = false;
        part.confirm.accepted.connect(() => accepted = true);
        part.confirm.first.clicked();
        if (!accepted || !Qt.colorEqual(part.confirm.first.palette.highlight, Look.coral))
            fail("the confirmation does not have a coral main button that accepts");
        page.show(0);
        if (part.question.visible)
            fail("a small panel stays open when its part is left");
    }

    function exercise() {
        page = Qt.createComponent("PiecesAdjust.qml").createObject(host.contentItem, {
            "width": host.width,
            "height": host.height
        });
        rows();
        more();
        said();
        panels();
        page.destroy();
    }
}
