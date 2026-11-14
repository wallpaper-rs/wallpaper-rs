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

    function failing(offline) {
        const art = page.providers.find(p => p.id === "art");
        const broken = Object.assign({}, art, {
            "state": "error",
            "offline": offline,
            "last_error": offline ? "no connection: the server could not be reached" : "the server answered 500"
        });
        page.apply(page.providers.map(p => p.id === "art" ? broken : p));
        const card = page.cards.card("art");
        if (page.cards.offline !== offline)
            fail(offline ? "a source without connection does not show the no connection notice" : "a source in error shows the no connection notice");
        if (!card || card.problem !== (offline ? "" : broken.last_error))
            fail(offline ? "a source without connection shows a problem of its own" : "a source in error does not say its problem");
    }

    function exercise() {
        failing(false);
        failing(true);
        page.load();
        clearing();
        more();
        page.open("list");
        destroy();
    }
}
