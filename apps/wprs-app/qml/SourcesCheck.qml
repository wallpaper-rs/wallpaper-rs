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

    function exercise() {
        clearing();
        page.open("list");
        destroy();
    }
}
