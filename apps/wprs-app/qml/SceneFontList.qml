import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

QQC2.Popup {
    id: list

    required property var listing
    required property string folder
    readonly property string needle: search.text.trim().toLowerCase()
    readonly property var rows: {
        const mark = (names, group) => names.map(n => ({
                            "name": typeof n === "string" ? n : n.name,
                            "path": typeof n === "string" ? "" : n.path,
                            "group": group
                        }));
        const all = mark(listing.package ?? [], "package").concat(mark(listing.generic ?? [], "generic"), mark(listing.system ?? [], "system"));
        return all.filter(row => needle === "" || row.name.toLowerCase().includes(needle));
    }

    signal picked(string name)

    width: Kirigami.Units.gridUnit * 22
    height: Math.min(Kirigami.Units.gridUnit * 24, parent ? parent.height : Kirigami.Units.gridUnit * 24)
    modal: true
    focus: true
    onOpened: search.forceActiveFocus()
    onClosed: search.clear()

    contentItem: ColumnLayout {
        QQC2.TextField {
            id: search

            Layout.fillWidth: true
            placeholderText: App.tr("Search fonts")
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: list.rows
            QQC2.ScrollBar.vertical: QQC2.ScrollBar {
            }

            delegate: QQC2.ItemDelegate {
                id: row

                required property var modelData

                width: ListView.view.width
                onClicked: {
                    list.picked(modelData.name);
                    list.close();
                }

                FontLoader {
                    id: loaded

                    source: row.modelData.path !== "" ? "file://" + list.folder + "/" + row.modelData.path : ""
                }

                contentItem: ColumnLayout {
                    spacing: 0

                    QQC2.Label {
                        Layout.fillWidth: true
                        text: row.modelData.name
                        elide: Text.ElideRight
                        font.family: loaded.status === FontLoader.Ready ? loaded.name : row.modelData.name
                        font.pointSize: Kirigami.Theme.defaultFont.pointSize * 1.3
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        text: ({
                                "package": App.tr("In this package"),
                                "generic": App.tr("Whatever the system uses for this style"),
                                "system": App.tr("Installed on this computer")
                            })[row.modelData.group]
                        font: Kirigami.Theme.smallFont
                        opacity: 0.7
                        elide: Text.ElideRight
                    }
                }
            }
        }
    }
}
