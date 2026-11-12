import QtQuick

Loader {
    id: fields

    required property var host

    sourceComponent: ({
            "package:credits": credits
        })[host.field.widget]

    Component {
        id: credits

        PackageCredits {
            host: fields.host
        }
    }
}
