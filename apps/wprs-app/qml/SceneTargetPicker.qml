import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: picker

    required property var info
    required property var value
    property var at: null
    readonly property var names: (info?.points ?? []).map(p => p.name).concat(["cursor"])
    readonly property bool fixed: Array.isArray(value)

    signal picked(var target)

    function start() {
        const point = (info?.points ?? []).find(p => p.name === value);
        if (point?.local)
            return point.local;
        const m = info?.inverse;
        if (Array.isArray(at) && m)
            return [Math.round((m[0] * at[0] + m[2] * at[1] + m[4]) * 10) / 10, Math.round((m[1] * at[0] + m[3] * at[1] + m[5]) * 10) / 10];
        return [0, 0];
    }

    function typed(coordinate, text) {
        const number = Number(text);
        if (text === "" || !isFinite(number) || number === value[coordinate])
            return;
        const next = value.slice();
        next[coordinate] = Math.round(number * 10) / 10;
        picked(next);
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: picker.names.concat([App.tr("a fixed point")])
        currentIndex: picker.fixed ? picker.names.length : picker.names.indexOf(picker.value)
        onActivated: index => {
            if (index < picker.names.length)
                picker.picked(picker.names[index]);
            else if (!picker.fixed)
                picker.picked(picker.start());
        }
    }

    SceneRepeater {
        id: fields

        rows: picker.fixed ? [[0, "X"], [1, "Y"]] : []

        delegate: QQC2.TextField {
            required property int index
            readonly property var modelData: fields.rows[index]

            Layout.preferredWidth: Kirigami.Units.gridUnit * 4
            text: String(picker.value?.[modelData[0]] ?? "")
            placeholderText: modelData[1]
            inputMethodHints: Qt.ImhFormattedNumbersOnly
            validator: DoubleValidator {
                locale: "C"
            }
            QQC2.ToolTip.visible: hovered
            QQC2.ToolTip.text: App.tr("Coordinate %1 of the fixed point, in pixels of the system", modelData[1])
            onEditingFinished: picker.typed(modelData[0], text)
        }
    }
}
