import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    required property var page

    function exercise() {
        const quiet = Qt.createComponent("SceneProblems.qml").createObject(page, {
                "page": page,
                "scriptError": "",
                "scriptLines": []
            });
        const loud = Qt.createComponent("SceneProblems.qml").createObject(page, {
                "page": page,
                "scriptError": "line 3: boom",
                "scriptLines": ["hello"]
            });
        if (!(loud.wanted > quiet.wanted))
            throw new Error("problems: a script error gets " + loud.wanted + " against " + quiet.wanted);
        if (loud.wanted < 200)
            throw new Error("problems: a script error and console get only " + loud.wanted);
        quiet.destroy();
        loud.destroy();
    }
}
