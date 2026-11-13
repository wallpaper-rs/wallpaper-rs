import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: moves

    required property var owner

    RowLayout {
        Layout.fillWidth: true

        QQC2.Button {
            icon.name: "edit-link"
            text: App.tr("Join")
            enabled: moves.owner.picks.length >= 2
            onClicked: moves.owner.join()
        }

        QQC2.Button {
            icon.name: "arrow-up"
            text: App.tr("Nearer")
            enabled: moves.owner.picks.length >= 1
            onClicked: moves.owner.nudge(moves.owner.step)
        }

        QQC2.Button {
            icon.name: "arrow-down"
            text: App.tr("Farther")
            enabled: moves.owner.picks.length >= 1
            onClicked: moves.owner.nudge(-moves.owner.step)
        }
    }

    EditSlider {
        text: App.tr("Move by")
        from: 0.01
        to: 0.5
        stepSize: 0.01
        value: moves.owner.step
        onMoved: value => moves.owner.step = value
    }

    QQC2.Button {
        Layout.fillWidth: true
        checkable: true
        checked: moves.owner.page.regionSplit
        icon.name: "edit-cut"
        text: App.tr("Split along a stroke")
        onClicked: moves.owner.page.regionSplit = checked
    }
}
