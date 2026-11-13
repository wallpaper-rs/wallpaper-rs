import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: note

    required property var survey
    required property string last
    property var notes: []
    property real known
    property var drop: null
    readonly property var names: ({
            "depth_join": App.tr("joining regions"),
            "depth_split": App.tr("splitting a region"),
            "depth_move": App.tr("moving a region"),
            "": App.tr("the last change")
        })

    function follow() {
        const reach = survey.reach;
        if (reach === undefined || reach === null)
            return;
        if (known > 0 && reach < known * 0.97)
            drop = {
                "from": known,
                "to": reach,
                "by": names[last] ?? names[""]
            };
        else if (drop && reach >= drop.from * 0.97)
            drop = null;
        known = reach;
    }

    onSurveyChanged: follow()

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        visible: note.drop !== null
        text: note.drop ? App.tr("The movement limit dropped after %1: the picture can now move up to %2 % of its width, it was %3 %. Less movement keeps the picture from stretching.", note.drop.by, (note.drop.to * 100).toFixed(1), (note.drop.from * 100).toFixed(1)) : ""
        showCloseButton: false
        actions: [
            Kirigami.Action {
                text: App.tr("Dismiss")
                icon.name: "dialog-ok"
                onTriggered: note.drop = null
            }
        ]
    }

    Repeater {
        model: note.notes

        delegate: QQC2.Label {
            required property var modelData

            Layout.fillWidth: true
            color: Kirigami.Theme.neutralTextColor
            wrapMode: Text.Wrap
            text: modelData.kind === "join" ? App.tr("Join %1 does not apply: both its points are in one region. Put the regions back, or undo, and redo it.", modelData.index) : modelData.kind === "split" ? App.tr("Split %1 does not apply: its stroke crosses no region. Put the regions back, or undo, and redo it.", modelData.index) : App.tr("Move %1 does not apply: its region was already moved or joined with another. Put the regions back, or undo, and redo it.", modelData.index)
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: note.survey.reach !== undefined && note.survey.reach !== null
        text: App.tr("Movement limit: %1 % of the width", ((note.survey.reach ?? 0) * 100).toFixed(1))
        font: Kirigami.Theme.smallFont
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: (note.survey.still ?? "") !== ""
        text: note.survey.still ?? ""
        color: Kirigami.Theme.negativeTextColor
        wrapMode: Text.Wrap
    }
}
