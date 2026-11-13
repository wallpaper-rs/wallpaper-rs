import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: installed

    required property Installer installer
    property var packages: []
    property string removingId
    property string removingVersion
    readonly property var updates: packages.filter((p, i) => p.update && packages.findIndex(q => q.id === p.id) === i)
    readonly property var plain: updates.filter(p => !p.asks || p.asks.length === 0)

    function load() {
        App.query("ListInstalled", [], "Repos1", reply => installed && (installed.packages = reply.packages));
    }

    function confirmUninstall(id, version) {
        installed.removingId = id;
        installed.removingVersion = version;
        uninstaller.open();
    }

    Component.onCompleted: load()

    Connections {
        target: App.backend

        function onChanged(topic) {
            if (topic === "repos")
                installed.load();
        }
    }

    QQC2.Button {
        text: App.tr("Update all")
        icon.name: "system-software-update"
        enabled: installed.plain.length > 0
        onClicked: installed.plain.forEach(p => installed.installer.install(p.repo_id + "/" + p.id))
    }

    QQC2.ScrollView {
        Layout.fillWidth: true
        Layout.fillHeight: true

        FocusList {
            model: installed.packages

            delegate: RowLayout {
                id: row

                required property var modelData

                width: ListView.view.width

                ColumnLayout {
                    Layout.fillWidth: true

                    QQC2.Label {
                        Layout.fillWidth: true
                        text: row.modelData.id + " " + row.modelData.version + (row.modelData.revoked ? " · " + App.tr("revoked: %1", Labels.value(row.modelData.revoked)) : "")
                        textFormat: Text.PlainText
                        color: row.modelData.revoked ? Look.coral : Kirigami.Theme.textColor
                        elide: Text.ElideRight
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        visible: !!row.modelData.update && text !== ""
                        text: row.modelData.changes ? App.tr("What's new: %1", App.plain(row.modelData.changes)) : ""
                        textFormat: Text.PlainText
                        wrapMode: Text.Wrap
                        font: Kirigami.Theme.smallFont
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        visible: !!row.modelData.update && !!row.modelData.asks && row.modelData.asks.length > 0
                        text: App.tr("Not updated automatically: it now asks for %1", (row.modelData.asks ?? []).join(", "))
                        textFormat: Text.PlainText
                        wrapMode: Text.Wrap
                        color: Look.ambar
                        font: Kirigami.Theme.smallFont
                    }
                }

                QQC2.Button {
                    visible: !!row.modelData.update
                    text: App.tr("Update to %1", row.modelData.update ?? "")
                    onClicked: installed.installer.install(row.modelData.repo_id + "/" + row.modelData.id)
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    text: App.tr("Uninstall")
                    onClicked: installed.confirmUninstall(row.modelData.id, row.modelData.version)
                }

                QQC2.ToolButton {
                    icon.name: "edit-delete-remove"
                    text: App.tr("Uninstall all versions")
                    display: QQC2.AbstractButton.IconOnly
                    QQC2.ToolTip.text: text
                    QQC2.ToolTip.visible: hovered
                    onClicked: installed.confirmUninstall(row.modelData.id, "")
                }
            }
        }
    }

    Kirigami.Dialog {
        id: uninstaller

        title: App.tr("Uninstall package")
        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
        onAccepted: App.act("Uninstall", [installed.removingId, installed.removingVersion], "Repos1")

        QQC2.Label {
            Layout.preferredWidth: Kirigami.Units.gridUnit * 24
            wrapMode: Text.Wrap
            text: installed.removingVersion === "" ? App.tr("Uninstall every installed version of %1? You can install it again from Browse.", installed.removingId) : App.tr("Uninstall %1 %2? You can install it again from Browse.", installed.removingId, installed.removingVersion)
        }
    }
}
