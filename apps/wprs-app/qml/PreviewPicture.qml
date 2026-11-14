import QtQuick
import io.github.wallpaper_rs.app

Foto {
    id: picture

    property var item: null
    property string file: ""
    property bool whole: false
    property string found: ""
    readonly property string uid: item ? item.uid : ""
    readonly property string shown: whole && item && item.kind === "image" && file !== "" ? file : found

    function fetch() {
        found = "";
        const wanted = uid;
        if (wanted !== "")
            App.query("Thumbnail", [wanted, 1024], "Library1", reply => picture && picture.uid === wanted && (picture.found = reply[0] ?? ""), () => {});
    }

    source: App.url(shown)
    sourceSize.width: whole ? width : PreviewLook.tiraFoco * 2
    onUidChanged: fetch()
    Component.onCompleted: fetch()
}
