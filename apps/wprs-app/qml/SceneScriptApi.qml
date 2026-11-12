import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: api

    required property var page
    property var parts: []
    readonly property var headings: ({
            "scene": App.tr("Scene and layers"),
            "inputs": App.tr("What the scene gives a script"),
            "tools": App.tr("Timers, storage and other tools"),
            "events": App.tr("Functions a module exports"),
            "values": App.tr("Functions a value follows")
        })

    title: App.tr("Script API")
    standardButtons: Kirigami.Dialog.Close
    preferredWidth: Kirigami.Units.gridUnit * 40
    onAboutToShow: api.parts = JSON.parse(page.editor.code("reference", "{}") || "[]")

    QQC2.ScrollView {
        id: scroll

        implicitWidth: Kirigami.Units.gridUnit * 38
        implicitHeight: Kirigami.Units.gridUnit * 24

        ColumnLayout {
            width: scroll.availableWidth
            spacing: Kirigami.Units.smallSpacing

            Repeater {
                model: api.parts

                delegate: ColumnLayout {
                    id: part

                    required property var modelData

                    Layout.fillWidth: true

                    Kirigami.Heading {
                        level: 3
                        text: api.headings[part.modelData.part] ?? part.modelData.part
                    }

                    Repeater {
                        model: part.modelData.items

                        delegate: ColumnLayout {
                            id: item

                            required property var modelData

                            Layout.fillWidth: true
                            spacing: 0

                            QQC2.Label {
                                text: item.modelData.name
                                font.family: "monospace"
                                font.bold: true
                                wrapMode: Text.Wrap
                                Layout.fillWidth: true
                            }

                            QQC2.Label {
                                text: item.modelData.text
                                wrapMode: Text.Wrap
                                opacity: 0.8
                                Layout.fillWidth: true
                            }
                        }
                    }
                }
            }
        }
    }
}
