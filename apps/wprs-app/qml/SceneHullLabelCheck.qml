import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function find(item, test) {
        if (!item)
            return null;
        if (test(item))
            return item;
        for (const child of item.children ?? []) {
            const found = find(child, test);
            if (found)
                return found;
        }
        return null;
    }

    function exercise() {
        const motion = {
            "page": page,
            "layerId": "",
            "bone": "",
            "bones": ["spine"],
            "info": {
                "model": true,
                "colliders": [
                    {
                        "bone": "spine",
                        "shape": "capsule",
                        "radius": 0.2
                    }
                ]
            }
        };
        const hulls = Qt.createComponent("SceneModelColliders.qml").createObject(page, {
                "motion": motion
            });
        const slider = find(hulls, item => item.objectName === "hullRadius");
        if (!slider || slider.Accessible.name !== App.tr("Radius"))
            throw new Error("hulls: the collider radius slider has no name");
        hulls.destroy();
    }
}
