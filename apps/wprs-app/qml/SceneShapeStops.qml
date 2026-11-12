import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

SceneGradientBar {
    id: bar

    required property var page
    required property string layerId

    function stop(op) {
        page.run("layer", {
                "figure": {
                    "stop": {
                        "layer": layerId,
                        "stop": op
                    }
                }
            }, op.move !== undefined);
    }

    most: 4
    least: 2
    onStopAdded: at => stop({
                "add": {
                    "at": at
                }
            })
    onStopMoved: (index, at, gesture) => stop({
                "move": {
                    "index": index,
                    "at": at
                }
            })
    onStopColored: (index, hex) => stop({
                "color": {
                    "index": index,
                    "color": hex
                }
            })
    onStopRemoved: index => stop({
                "remove": {
                    "index": index
                }
            })
}
