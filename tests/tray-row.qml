import QtQuick
import QtTest
import "plasmoid/contents/ui"
import "app" as App

Item {
    width: 400
    height: 320

    ScreenRow {
        id: row

        property var decided: []
        property var asked: []
        property var acts: []
        property var opened: []

        function i18n(text, first, second, third) {
            return [text, first, second, third].filter(part => part !== undefined).join(" ");
        }

        width: 400
        index: 0
        name: "X-1"
        info: ({
                "item": "A",
                "playlist": null,
                "still_reason": "battery"
            })
        daemon: QtObject {
            property var current: ({})
            property var state: ({})

            signal updated(string topic, var ids)
            signal choose(string name)

            function query(member, args, iface, onReply) {
                row.asked.push(member);
                if (member === "Thumbnail")
                    return onReply([Qt.resolvedUrl("green.png").toString()]);
                onReply({
                    "item": {
                        "title": "Storm",
                        "package": {
                            "content": "mature",
                            "photosensitive": true,
                            "confirmed": row.decided
                        }
                    }
                });
            }

            function act(member, args, iface) {
                row.acts.push([iface, member].concat(args));
            }

            function url(path) {
                return path;
            }
        }

        onMore: name => opened.push(name)
    }

    App.Thumbnail {
        id: shown

        property var acts: []

        y: 220
        width: 120
        height: 80
        item: "B"
        source: Qt.resolvedUrl("green.png")
        mature: acts.length === 0
        scene: false
        daemon: QtObject {
            function tr(text) {
                return text;
            }

            function act(member, args, iface) {
                shown.acts = shown.acts.concat([[iface, member].concat(args)]);
            }
        }
    }

    TestCase {
        function find(item, test) {
            return test(item) ? item : Array.from(item.data ?? []).map(child => find(child, test)).find(found => found) ?? null;
        }

        function test_row_shows_the_wallpaper_and_its_actions() {
            tryVerify(() => row.thumb !== "");
            compare(row.asked, ["GetItem", "Thumbnail"]);
            tryCompare(row, "title", "Storm");
            const next = find(row, item => String(item.text ?? "") === "Next");
            const choose = find(row, item => String(item.text ?? "") === "Choose…");
            const more = find(row, item => item.glyph === "dots-three");
            const where = find(row, item => String(item.text ?? "").startsWith("X-1 · "));
            compare(next.enabled, false);
            compare(where.text, "X-1 · no playlist running");
            compare(find(row, item => String(item.text ?? "").startsWith("Mature content")), null);
            row.info = {
                "item": "A",
                "playlist": {
                    "name": "Calm",
                    "position": 0,
                    "count": 3
                }
            };
            compare(next.enabled, true);
            mouseClick(next);
            compare(row.acts, [["Playlists1", "Next", "X-1"]]);
            let chosen = [];
            row.daemon.choose.connect(name => chosen.push(name));
            mouseClick(choose);
            compare(chosen, ["X-1"]);
            mouseClick(more);
            compare(row.opened, ["X-1"]);
        }

        function test_the_app_reveals_with_a_key() {
            const image = find(shown, item => item instanceof Image);
            const reveal = find(shown, item => String(item.text ?? "").startsWith("Mature content"));
            tryCompare(image, "status", Image.Ready);
            compare([image.implicitHeight, reveal.visible, reveal.activeFocusOnTab], [12, true, true]);
            reveal.forceActiveFocus();
            keyClick(Qt.Key_Space);
            compare(shown.acts, [["Library1", "Confirm", ["B"], "mature"]]);
            tryCompare(image, "implicitHeight", 32);
            mouseClick(image);
            compare([reveal.visible, shown.acts.length], [false, 1]);
        }

        when: windowShown
    }
}
