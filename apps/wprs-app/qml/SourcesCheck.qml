import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page

    function fail(what) {
        throw new Error("sources: " + what);
    }

    function detail(id) {
        page.current = id;
        page.open("detail");
        if (!page.shown || page.shown.provider.id !== id)
            fail("the details of " + id + " did not open");
        return page.shown;
    }

    function clearing() {
        const shown = detail("daily");
        shown.clearing.open();
        if (!shown.clearing.visible || shown.clearing.title !== App.tr("Delete the stored photos?"))
            fail("the stored photos are not deleted through their confirmation");
        shown.clearing.close();
    }

    function more() {
        detail("art");
        page.deeper("");
        const shown = page.shown;
        if (!shown || shown.level !== "" || !shown.rows.visible)
            fail("the more options of a search source did not open");
        if (shown.words("purity") !== App.tr("For everyone") || shown.words("categories").split(" · ").length !== 2)
            fail("the more options do not name what the source brings");
        page.deeper("purity");
        if (page.shown.level !== "purity" || page.shown.listed.rows.length !== 3)
            fail("the content level does not list its three choices");
        page.deeper("colors");
        page.routeBack();
        if (page.shown.level !== "size")
            fail("going back from the colors does not return to the size");
        page.routeBack();
        page.routeBack();
        if (page.view !== "detail")
            fail("going back from the more options does not return to the details");
    }

    function exercise() {
        clearing();
        more();
        page.open("list");
        destroy();
    }
}
