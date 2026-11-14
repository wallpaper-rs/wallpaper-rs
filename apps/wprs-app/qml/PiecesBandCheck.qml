import QtQuick
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var host
    readonly property Component blank: Component {
        Kirigami.Page {}
    }

    function fail(what) {
        throw new Error("band: " + what);
    }

    function column(frame) {
        return Qt.rect(Look.margen + Look.menuAncho, frame.upper.y, frame.width - 2 * Look.margen - Look.menuAncho, 0);
    }

    function veilOf(frame) {
        for (let at = 0; at < frame.children.length; at++)
            if (typeof frame.children[at].shade === "function" && frame.children[at].centerX === undefined)
                return frame.children[at];
        return null;
    }

    function settle(sheet, high) {
        sheet.height = high;
        for (let round = 0; round < 3; round++) {
            sheet.side.forceLayout();
            sheet.page.layout();
            sheet.frame.measure();
            sheet.frame.place(true);
        }
    }

    function exercise() {
        const sheet = Qt.createComponent("PiecesBand.qml").createObject(host, {
                "width": 1600,
                "height": 900
            });
        if (!sheet)
            fail("the band page does not load");
        const frame = sheet.frame;
        const page = sheet.page;
        const steps = Qt.createComponent("LookWindow.qml").createObject(sheet, {
                "host": sheet
            });
        steps.tight = Qt.binding(() => frame.tight);
        if (!frame.banded)
            fail("a page that opts in does not get the band in a window of 1600×900");
        if (page.band.x !== Look.margen || page.band.width !== 1600 - 2 * Look.margen || page.band.y < frame.sideBottom)
            fail("the band is not under the menu from margin to margin: " + page.band);
        if (page.upper.x !== column(frame).x || page.upper.y + page.upper.height > page.band.y)
            fail("the upper block is not at the right of the menu above the band: " + page.upper);
        const shelf = page.shelf.mapToItem(sheet, 0, 0);
        if (shelf.x !== Look.margen || shelf.y < frame.sideBottom)
            fail("the shelf does not start at the left margin under the menu: " + shelf);
        const veil = veilOf(frame);
        if (!veil || veil.y > page.band.y || veil.width !== 1600)
            fail("the band veil does not cover the band at full width");
        for (const high of [860, 810]) {
            settle(sheet, high);
            if (!frame.banded || page.band.x !== Look.margen)
                fail("a window " + high + " high does not get the band: " + page.band);
        }
        settle(sheet, 900);
        if (frame.tight || Look.compacto)
            fail("a window 900 high keeps the smaller step");
        page.shelf.forceActiveFocus();
        sheet.height = 480;
        if (frame.banded)
            fail("the band stays under the menu in a window 480 high");
        if (page.band.x !== column(frame).x || page.upper.y > page.band.y)
            fail("in a low window the page is not in the column: " + page.band);
        if (!page.shelf.activeFocus)
            fail("the focus left the shelf when the window got lower");
        if (!veil || veil.y > page.band.y)
            fail("the band veil does not cover the band in the column");
        sheet.destroy();
        host.showPage(0);
        host.open(blank);
        const title = host.shell.titleFrame;
        if (title.bandLow !== 0 || title.tight || title.seat.x !== Look.margen + Look.menuAncho || host.pageStack.parent !== title.seat)
            fail("a page that does not opt in left the column");
        host.showPage(0);
        destroy();
    }
}
