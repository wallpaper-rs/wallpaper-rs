import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: notes

    required property var page

    property bool outside: false

    spacing: 0

    Timer {
        interval: 2000
        running: notes.page.editor.ready && notes.page.visible && !notes.page.editor.busy
        repeat: true
        onTriggered: notes.page.editor.followFolder()
    }

    Connections {
        target: notes.page.editor

        function onErrorChanged() {
            notes.outside = notes.page.editor.error !== "" && notes.page.editor.diverged();
        }
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Error
        text: notes.page.editor.error
        visible: text !== ""
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        text: App.tr("Another program changed this folder while you were editing. Your changes are not lost: choose what to keep.")
        visible: notes.outside && notes.page.editor.error !== ""
        actions: [
            Kirigami.Action {
                text: App.tr("Keep mine")
                onTriggered: notes.page.editor.settleOutside("mine", "")
            },
            Kirigami.Action {
                text: App.tr("Take theirs")
                onTriggered: notes.page.editor.settleOutside("theirs", "")
            },
            Kirigami.Action {
                text: App.tr("Save elsewhere…")
                onTriggered: elsewhere.open()
            }
        ]
    }

    FolderDialog {
        id: elsewhere

        onAccepted: notes.page.editor.settleOutside("elsewhere", App.path(selectedFolder))
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Positive
        text: notes.page.notice
        visible: text !== ""
        showCloseButton: true
    }

    Kirigami.InlineMessage {
        Layout.fillWidth: true
        type: Kirigami.MessageType.Warning
        text: App.tr("This folder has unsaved changes from an earlier session.")
        visible: notes.page.view.recoverable === true
        actions: [
            Kirigami.Action {
                text: App.tr("Recover")
                onTriggered: notes.page.editor.recover(true)
            },
            Kirigami.Action {
                text: App.tr("Discard")
                onTriggered: notes.page.editor.recover(false)
            }
        ]
    }
}
