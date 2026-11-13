import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function depth() {
        return page.edit.depth ?? {};
    }

    function expect(ok, what) {
        if (!ok)
            throw new Error("regions: " + what + ": " + JSON.stringify(depth()));
    }

    function exercise() {
        const zones = [{
                "order": 1,
                "depth": 0.9,
                "share": 0.3,
                "at": [0.25, 0.5],
                "colour": "#ef4444"
            }, {
                "order": 2,
                "depth": 0.5,
                "share": 0.2,
                "at": [0.7, 0.5],
                "colour": "#facc15"
            }, {
                "order": 3,
                "depth": 0.1,
                "share": 0.5,
                "at": [0.1, 0.9],
                "colour": "#4287f5"
            }];
        const tool = Qt.createComponent("EditRegions.qml").createObject(page, {
                "page": page,
                "map": "",
                "zones": zones
            });
        const canvas = Qt.createComponent("EditRegionsCanvas.qml").createObject(page, {
                "page": page,
                "width": 200,
                "height": 100,
                "zones": zones,
                "grid": {
                    "w": 2,
                    "h": 1,
                    "cells": [0, 2]
                }
            });
        page.regionPicks = [];
        tool.reset();
        canvas.toggle(canvas.pickAt(10, 50));
        canvas.toggle(canvas.pickAt(190, 50));
        expect(page.regionPicks.join() === "1,3", "clicking the picture did not pick the regions");
        canvas.toggle(canvas.pickAt(190, 50));
        expect(page.regionPicks.join() === "1", "a second click did not drop the region");
        canvas.toggle(canvas.pickAt(190, 50));
        tool.join();
        expect(depth().joins?.length === 1 && page.regionPicks.length === 0, "two picked regions were not joined");
        page.regionPicks = [2];
        tool.nudge(-0.2);
        expect(depth().moves?.length === 1 && depth().moves[0].by === -0.2, "a picked region was not moved");
        canvas.add(20, 10);
        canvas.add(100, 50);
        canvas.add(100.2, 50.1);
        canvas.lay();
        expect(depth().splits?.length === 1 && depth().splits[0].points.length === 2, "a stroke did not split");
        page.editor.undo();
        expect((depth().splits ?? []).length === 0 && depth().moves.length === 1, "undo did not take the split back");
        page.editor.redo();
        expect(depth().splits.length === 1, "redo did not bring the split back");
        tool.reset();
        expect(!depth().joins && !depth().moves && !depth().splits, "the regions were not put back");
        page.regionPicks = [];
        tool.destroy();
        canvas.destroy();
    }
}
