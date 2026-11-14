import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

ColumnLayout {
    id: picker

    required property var daemon
    property string output: ""
    property var info: ({})
    readonly property var transition: info.transition ?? null
    readonly property string general: String((daemon.state?.options ?? {})["transition.kind"] ?? "own")
    readonly property var words: ({
            "own": daemon.tr("The transition each wallpaper brings, or a fade"),
            "fade": daemon.tr("Always a plain fade"),
            "none": daemon.tr("None: change at once")
        })
    readonly property var reasons: ({
            "own": daemon.tr("Last change: the wallpaper's own transition."),
            "chosen": daemon.tr("Last change: what you chose."),
            "reduced": daemon.tr("Last change: a plain fade, because of less motion."),
            "fallen": daemon.tr("Last change: a plain fade, because the wallpaper's transition could not play."),
            "plain": daemon.tr("Last change: a plain fade, neither wallpaper brings a transition.")
        })
    readonly property var choices: [
        {
            "value": "general",
            "text": daemon.tr("Like every screen: %1", words[general] ?? general)
        }
    ].concat(Object.keys(words).map(value => ({
                "value": value,
                "text": words[value]
            })))

    QQC2.ComboBox {
        Layout.preferredWidth: Kirigami.Units.gridUnit * 24
        Accessible.name: picker.daemon.tr("Transition of %1", picker.output)
        model: picker.choices
        textRole: "text"
        valueRole: "value"
        currentIndex: Math.max(0, picker.choices.findIndex(c => c.value === (picker.transition?.own ?? "general")))
        onActivated: picker.daemon.act("SetOption", ["transition.kind." + picker.output, currentValue])
    }

    QQC2.Label {
        Layout.maximumWidth: Kirigami.Units.gridUnit * 30
        visible: text !== ""
        wrapMode: Text.Wrap
        textFormat: Text.PlainText
        font: Kirigami.Theme.smallFont
        text: picker.reasons[picker.transition?.why ?? ""] ?? ""
    }
}
