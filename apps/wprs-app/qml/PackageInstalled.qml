import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: installed

    property var list: []

    function load() {
        list = [];
        for (const query of ["", "kind:assets"])
            App.query("Search", [query, 0, 500], "Library1", reply => {
                    if (!installed)
                        return;
                    const seen = installed.list.map(item => item.id);
                    const fresh = (reply?.items ?? []).filter(item => item.package?.id && !seen.includes(item.package.id));
                    installed.list = installed.list.concat(fresh.map(item => ({
                                        "uid": item.uid,
                                        "id": item.package.id,
                                        "version": item.package.version,
                                        "title": item.title
                                    })));
                }, () => {});
    }

    Component.onCompleted: load()
}
