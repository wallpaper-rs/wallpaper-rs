import QtQuick

QtObject {
    id: bench

    property var findings: []
    property string kind: "scene"
    property string notice: ""
    readonly property QtObject editor: QtObject {
        property bool busy: false
        property bool dirty: false
        property string error: ""
        property string folder: ""

        signal finished(string message)

        function value(pointer) {
            return '{"id": "org.example.check", "version": "1.2.0", "license": "MIT", "photosensitive": false, "content": {}}';
        }

        function rig(path, query) {
            return '{"findings": ' + JSON.stringify(bench.findings) + '}';
        }

        function save() {
        }

        function build(out, secret, word) {
        }

        function apply(output) {
        }
    }

    function change(pointer, value) {
    }

    function jump(pointer) {
    }
}
