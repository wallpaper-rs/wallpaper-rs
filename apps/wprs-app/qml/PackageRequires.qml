import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: requires

    required property var page
    property var installed: []
    readonly property var needed: page.manifest.requires?.packages ?? []
    readonly property var choices: installed.filter(item => item.id !== page.manifest.id && !needed.some(n => n.id === item.id))
    readonly property var found: choices.find(item => item.id === entry.editText.trim())
    readonly property int most: 16

    function load() {
        installed = [];
        for (const query of ["", "kind:assets"])
            App.query("Search", [query, 0, 500], "Library1", reply => {
                if (!requires)
                    return;
                const seen = requires.installed.map(item => item.id);
                const fresh = (reply?.items ?? []).filter(item => item.package?.id && !seen.includes(item.package.id));
                requires.installed = requires.installed.concat(fresh.map(item => ({
                            "id": item.package.id,
                            "version": item.package.version,
                            "title": item.title
                        })));
            }, () => {});
    }

    function add() {
        const id = entry.editText.trim();
        const range = version.text.trim();
        if (id === "" || (range === "" && !found))
            return;
        page.manifestEdit("requires", {
                "packages": [
                    {
                        "id": id,
                        "version": range !== "" ? range : found.version
                    }
                ],
                "add": true
            });
        entry.editText = "";
        version.text = "";
    }

    function drop(id) {
        page.manifestEdit("requires", {
                "packages": [
                    {
                        "id": id
                    }
                ],
                "remove": true
            });
    }

    Component.onCompleted: load()

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        text: requires.needed.length ? App.tr("These packages must be installed to use this one; installing it from a repository installs them too.") : App.tr("This package needs no other package. Add the ones whose effects, scenes or presets it uses.")
    }

    Repeater {
        model: requires.needed

        delegate: RowLayout {
            id: row

            required property var modelData

            Layout.fillWidth: true

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideMiddle
                textFormat: Text.PlainText
                text: row.modelData.id + " " + row.modelData.version
            }

            QQC2.ToolButton {
                icon.name: "edit-delete"
                text: App.tr("Remove")
                display: QQC2.AbstractButton.IconOnly
                QQC2.ToolTip.text: text
                QQC2.ToolTip.visible: hovered
                onClicked: requires.drop(row.modelData.id)
            }
        }
    }

    RowLayout {
        Layout.fillWidth: true
        enabled: requires.needed.length < requires.most

        QQC2.ComboBox {
            id: entry

            Layout.fillWidth: true
            editable: true
            model: requires.choices.map(item => item.id)
            currentIndex: -1
            Accessible.name: App.tr("Package")
            onActivated: version.text = ""
        }

        QQC2.TextField {
            id: version

            Layout.preferredWidth: Kirigami.Units.gridUnit * 8
            placeholderText: requires.found ? App.tr("Compatible with %1", requires.found.version) : App.tr("^1.2")
            Accessible.name: App.tr("Version range")
            onAccepted: requires.add()
        }

        QQC2.Button {
            text: App.tr("Add package")
            icon.name: "list-add"
            enabled: entry.editText.trim() !== "" && (!!requires.found || version.text.trim() !== "")
            onClicked: requires.add()
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("Pick an installed package: a version range left empty becomes the one compatible with its installed version, such as ^1.2. At most %1 packages.", requires.most)
    }
}
