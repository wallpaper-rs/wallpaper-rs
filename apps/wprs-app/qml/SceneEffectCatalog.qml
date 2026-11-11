import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    required property var inspector
    readonly property var catalog: inspector.page.view.effect_catalog ?? []
    readonly property var filtered: {
        const needle = field.text.toLowerCase();
        const match = e => needle === "" || SceneWords.effect(e.id).toLowerCase().includes(needle) || (e.description ?? "").toLowerCase().includes(needle);
        const order = (a, b) => (a.category ?? "").localeCompare(b.category ?? "") || SceneWords.effect(a.id).localeCompare(SceneWords.effect(b.id));
        return dialog.catalog.filter(match).slice().sort(order);
    }

    title: App.tr("Add an effect")
    standardButtons: Kirigami.Dialog.Cancel
    preferredWidth: Kirigami.Units.gridUnit * 24
    preferredHeight: Kirigami.Units.gridUnit * 26

    onOpened: field.forceActiveFocus()

    ColumnLayout {
        anchors.fill: parent

        Kirigami.SearchField {
            id: field

            Layout.fillWidth: true
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true

            ListView {
                model: dialog.filtered
                clip: true

                delegate: ColumnLayout {
                    id: item

                    required property var modelData
                    required property int index
                    readonly property bool starts: item.index === 0 || item.modelData.category !== dialog.filtered[item.index - 1].category

                    width: ListView.view.width

                    Kirigami.Heading {
                        visible: item.starts
                        level: 5
                        text: SceneWords.category(item.modelData.category ?? "")
                    }

                    QQC2.ItemDelegate {
                        Layout.fillWidth: true
                        onClicked: {
                            dialog.inspector.addEffect(item.modelData.id);
                            dialog.close();
                        }

                        contentItem: ColumnLayout {
                            QQC2.Label {
                                Layout.fillWidth: true
                                text: SceneWords.effect(item.modelData.id)
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                visible: (item.modelData.description ?? "") !== ""
                                text: App.tr("%1 (cost %2)", item.modelData.description ?? "", (item.modelData.units ?? 0).toFixed(1))
                                wrapMode: Text.Wrap
                                font: Kirigami.Theme.smallFont
                                color: Kirigami.Theme.disabledTextColor
                            }
                        }
                    }
                }
            }
        }
    }
}
