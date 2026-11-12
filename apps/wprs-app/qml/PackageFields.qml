import QtQuick

Loader {
    id: fields

    required property var host

    sourceComponent: ({
            "package:credits": credits,
            "package:strings": strings,
            "package:flashes": flashes
        })[host.field.widget]

    Component {
        id: strings

        PackageStrings {
            host: fields.host
        }
    }

    Component {
        id: flashes

        PackageFlashes {
            host: fields.host
        }
    }

    Component {
        id: credits

        PackageCredits {
            host: fields.host
        }
    }
}
