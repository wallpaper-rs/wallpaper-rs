import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: bursts

    required property var emission
    readonly property var list: emission.emitter.bursts ?? []

    function change(k, values, gesture) {
        const next = list.map(b => Object.assign({}, b));
        Object.keys(values).forEach(key => {
                if (values[key] === null)
                    delete next[k][key];
                else
                    next[k][key] = values[key];
            });
        emission.put({
                "bursts": next.length > 0 ? next : null
            }, gesture);
    }

    function drop(k) {
        const next = list.filter((b, i) => i !== k);
        emission.put({
                "bursts": next.length > 0 ? next : null
            });
    }

    RowLayout {
        Kirigami.Heading {
            Layout.fillWidth: true
            level: 4
            text: App.tr("Bursts")
        }

        QQC2.ToolButton {
            icon.name: "list-add"
            text: App.tr("Add a burst")
            display: QQC2.AbstractButton.IconOnly
            enabled: bursts.list.length < 8
            onClicked: emission.put({
                    "bursts": bursts.list.concat([{
                            "count": 20,
                            "every": 1
                        }])
                })
        }
    }

    SceneRepeater {
        id: listed

        rows: bursts.list

        delegate: GridLayout {
            id: row

            required property int index
            readonly property var modelData: listed.rows[index] ?? ({})
            readonly property bool timed: !modelData.on

            Layout.fillWidth: true
            columns: 4

            QQC2.SpinBox {
                from: 1
                to: 100000
                editable: true
                value: row.modelData.count ?? 1
                onValueModified: bursts.change(row.index, {
                        "count": value
                    })
            }

            QQC2.ComboBox {
                Layout.fillWidth: true
                model: [App.tr("on time"), App.tr("on the beat"), App.tr("on a click")]
                currentIndex: Math.max(0, emission.cues.indexOf(row.modelData.on ?? ""))
                onActivated: index => bursts.change(row.index, {
                        "on": index > 0 ? emission.cues[index] : null,
                        "at": index > 0 ? null : row.modelData.at ?? null,
                        "every": index > 0 ? null : row.modelData.every ?? null
                    })
            }

            QQC2.CheckBox {
                visible: row.timed
                text: App.tr("Repeat")
                checked: row.modelData.every !== undefined
                onToggled: bursts.change(row.index, {
                        "every": checked ? 1 : null
                    })
            }

            QQC2.ToolButton {
                Layout.column: 3
                icon.name: "edit-delete"
                text: App.tr("Remove this burst")
                display: QQC2.AbstractButton.IconOnly
                onClicked: bursts.drop(row.index)
            }

            QQC2.Label {
                visible: row.timed
                text: App.tr("After %1 s", Math.round((row.modelData.at ?? 0) * 100) / 100)
            }

            SceneSlider {
                Layout.fillWidth: true
                Layout.columnSpan: 3
                visible: row.timed
                from: 0
                to: Math.max(30, row.modelData.at ?? 0)
                value: row.modelData.at ?? 0
                onMoved: bursts.change(row.index, {
                        "at": value > 0 ? Math.round(value * 100) / 100 : null
                    }, true)
            }

            QQC2.Label {
                visible: row.timed && row.modelData.every !== undefined
                text: App.tr("Every %1 s", Math.round((row.modelData.every ?? 0) * 100) / 100)
            }

            SceneSlider {
                Layout.fillWidth: true
                Layout.columnSpan: 3
                visible: row.timed && row.modelData.every !== undefined
                from: 0.02
                to: Math.max(30, row.modelData.every ?? 0)
                value: row.modelData.every ?? 1
                onMoved: bursts.change(row.index, {
                        "every": Math.round(value * 100) / 100
                    }, true)
            }
        }
    }
}
