import QtQuick

Loader {
    id: fields

    required property var host

    sourceComponent: ({
            "package:credits": credits,
            "package:parts": parts,
            "package:strings": strings,
            "package:flashes": flashes,
            "package:license": licensing
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
        id: licensing

        PackageLicense {
            host: fields.host
        }
    }

    Component {
        id: credits

        PackageCredits {
            host: fields.host
        }
    }

    Component {
        id: parts

        PackageParts {
            host: fields.host
        }
    }
}
