import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host
    readonly property var shell: host.shell
    readonly property var allowed: ["left", "right", "up", "down", "Intro", "Esc", "Tab"]
    readonly property Component blank: Component {
        Kirigami.Page {}
    }

    function fail(what) {
        throw new Error("shell: " + what);
    }

    function words() {
        return host.shell.menu.all.map(entry => entry[1]).join(",");
    }

    function menu() {
        const wanted = ["Library", "Explore", "Lists", "Sources", "Create", "Settings", "Account"].map(word => App.tr(word)).join(",");
        if (words() !== wanted)
            fail("the menu reads " + words());
        for (let page = 0; page < host.pages.length; page++) {
            host.showPage(page);
            const marked = shell.menu.all.map((entry, at) => shell.menu.word(at)).filter(word => word.current);
            if (marked.length !== 1 || marked[0].page !== shell.menu.owner(page))
                fail("page " + page + " marks " + marked.length + " menu words");
        }
    }

    function hints(what) {
        const keys = [].concat(...shell.hints.model.map(hint => hint.keys));
        const stray = keys.filter(key => !allowed.includes(key));
        if (stray.length > 0)
            fail(what + " hints show the keys " + stray.join(","));
        return keys;
    }

    function keyboard() {
        host.showPage(0);
        shell.menu.forceActiveFocus(Qt.TabFocusReason);
        const onMenu = hints("menu");
        if (!onMenu.includes("up") || !onMenu.includes("Intro"))
            fail("the menu hints miss the arrows or Intro: " + onMenu.join(","));
        host.pageStack.currentItem.forceActiveFocus(Qt.TabFocusReason);
        const onPage = hints("page");
        if (!onPage.includes("Esc"))
            fail("the page hints miss Esc: " + onPage.join(","));
        if (!shell.hints.visible)
            fail("the hints line is hidden on a title screen");
    }

    function seated(kind, frame) {
        if (host.kind !== kind || !frame.visible)
            fail("the " + kind + " skeleton is not shown");
        if (host.pageStack.parent !== frame.seat)
            fail("the page does not sit in the " + kind + " skeleton");
        for (const other of [shell.titleFrame, shell.optionsFrame, shell.workFrame])
            if (other !== frame && other.visible)
                fail("two skeletons show at once");
    }

    function skeletons() {
        host.showPage(4);
        seated("title", shell.titleFrame);
        host.showPage(5);
        seated("options", shell.optionsFrame);
        if (shell.optionsFrame.sections.length !== 2)
            fail("the settings show no section words");
        host.showPage(0);
        host.work(blank);
        seated("work", shell.workFrame);
        if (shell.hints.visible || shell.menu.visible && shell.menu.parent.visible)
            fail("the work skeleton shows the menu or the hints");
        if (shell.workFrame.backButton.text !== App.tr("Library"))
            fail("the way back reads " + shell.workFrame.backButton.text);
        host.showPassiveNotification("probe");
        if (shell.notices.text !== "probe")
            fail("a notice does not reach the notice place");
        shell.notices.hide();
        shell.workFrame.backButton.clicked();
        if (host.current !== 0)
            fail("the way back opened page " + host.current);
    }

    function tones() {
        const fixed = JSON.parse(Look.bridge.derive(""));
        const keys = ["noche", "nocheAlta", "nocheLinea", "bruma", "brumaBaja", "luna", "lunaHonda", "velo"];
        const lost = keys.filter(key => fixed[key] === undefined);
        if (lost.length > 0)
            fail("the derived tones miss " + lost.join(","));
    }

    function exercise() {
        menu();
        keyboard();
        skeletons();
        tones();
        destroy();
    }
}
