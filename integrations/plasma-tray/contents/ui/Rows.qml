import QtQuick
import QtQuick.Layouts
import "../look"

ColumnLayout {
    id: rows

    required property var daemon
    required property var names
    signal more(string name)

    function rowOf(name) {
        for (let i = 0; i < list.count; i++) {
            if (list.itemAt(i).name === name)
                return list.itemAt(i);
        }
        return null;
    }

    spacing: Look.e3

    Repeater {
        id: list

        model: rows.names

        delegate: ScreenRow {
            required property string modelData

            Layout.fillWidth: true
            daemon: rows.daemon
            name: modelData
            single: rows.names.length === 1
            info: (rows.daemon.current.outputs ?? {})[modelData] ?? {}
            onMore: name => rows.more(name)
        }
    }
}
