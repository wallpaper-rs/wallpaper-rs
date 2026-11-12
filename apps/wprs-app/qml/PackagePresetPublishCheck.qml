import QtQuick

QtObject {
    required property var page

    function steps(shown) {
        const done = {};
        shown.report.steps.forEach(step => {
            done[step.id] = step.done;
        });
        return done;
    }

    function exercise() {
        const shown = Qt.createComponent("PackagePresetPublish.qml").createObject(page, {
            "page": page
        });
        if (shown.report.steps.length !== 6 || shown.report.ready)
            throw new Error("a new preset is not listed with its steps to do: " + JSON.stringify(shown.report));
        page.manifestEdit("preset", {
            "base": {
                "id": "org.example.base",
                "version": "1.0.0"
            },
            "values": {
                "speed": 0.5
            }
        });
        shown.refresh();
        const done = steps(shown);
        if (!done.base || !done.values || done.preview || shown.report.ready)
            throw new Error("the steps did not follow the base and the values: " + JSON.stringify(shown.report));
        shown.destroy();
    }
}
