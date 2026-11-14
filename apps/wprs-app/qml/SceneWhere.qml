import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

PanelChico {
    id: ask

    property bool viewing: false
    property int chosen: 0
    property var then: null
    readonly property var screens: [""].concat(App.outputs)

    function word(output) {
        return output === "" ? App.tr("Every screen") : output;
    }

    function choose(opener, viewing, then) {
        ask.opener = opener;
        ask.viewing = viewing;
        ask.then = then;
        ask.chosen = 0;
        open();
    }

    title: viewing ? App.tr("On which screen do you view it as your wallpaper?") : App.tr("On which screen do you set it as your wallpaper?")
    text: viewing ? App.tr("The window is minimized and a notice brings you back.") : ""
    quiet: App.tr("Cancel")
    principal: viewing ? App.tr("View it on %1", word(screens[chosen])) : App.tr("Set it on %1", word(screens[chosen]))
    principalIcon: viewing ? "monitor-play" : "image"
    onAccepted: {
        const then = ask.then;
        close();
        then && then(screens[chosen]);
    }
    onRejected: close()

    ColumnLayout {
        width: parent ? parent.width : 0
        spacing: 0

        Repeater {
            model: ask.screens

            Opcion {
                required property string modelData
                required property int index

                Layout.fillWidth: true
                text: ask.word(modelData)
                icon: "monitor"
                lit: ask.chosen === index
                Accessible.role: Accessible.RadioButton
                Accessible.checked: lit
                onClicked: ask.chosen = index
                Keys.onUpPressed: ask.chosen = Math.max(0, ask.chosen - 1)
                Keys.onDownPressed: ask.chosen = Math.min(ask.screens.length - 1, ask.chosen + 1)
            }
        }
    }
}
