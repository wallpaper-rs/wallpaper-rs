import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

ColumnLayout {
    id: facts

    required property var review
    readonly property var publish: review.publish
    readonly property var platforms: review.platforms
    readonly property var playing: Object.keys(platforms).filter(name => !(platforms[name] ?? {}).refused)
    readonly property var notes: Object.keys(platforms).map(name => note(name)).filter(text => text !== "")

    function place(name) {
        return name === "desktop" ? App.tr("Desktop") : ["phone", "android"].includes(name) ? App.tr("Phone") : name;
    }

    function where() {
        const names = playing.map(place);
        return names.length === 0 ? App.tr("Nowhere yet") : names.length === 1 ? names[0] : App.tr("%1 and %2", names.slice(0, -1).join(", "), names[names.length - 1]);
    }

    function note(name) {
        const on = platforms[name] ?? {};
        if (on.refused)
            return App.tr("%1: refused, %2", place(name), on.refused);
        return (on.without ?? []).length > 0 ? App.tr("%1: plays without %2", place(name), on.without.join(", ")) : "";
    }

    function setVersion(text) {
        if (text !== "" && text !== publish.version) {
            publish.page.change("/manifest/version", text);
            review.refresh();
        }
    }

    spacing: Look.e2

    Filas {
        Layout.fillWidth: true

        Fila {
            text: App.tr("Where it plays")
            value: facts.where()
        }
    }

    Repeater {
        model: facts.notes.concat((facts.review.report.build_adds ?? []).length > 0 ? [App.tr("Building adds to the features it needs: %1", facts.review.report.build_adds.map(f => f.name).join(", "))] : [])

        delegate: Nota {
            required property string modelData

            Layout.fillWidth: true
            text: modelData
        }
    }

    RowLayout {
        Layout.fillWidth: true
        spacing: Look.e3

        Text {
            Layout.fillWidth: true
            text: App.tr("Version")
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }

        QQC2.TextField {
            Layout.preferredWidth: Look.e8 + Look.e6
            text: facts.publish.version
            onEditingFinished: facts.setVersion(text)
        }
    }

    RowLayout {
        Layout.fillWidth: true
        visible: facts.review.license !== "" && !licenses.picking
        spacing: Look.e3

        Text {
            Layout.fillWidth: true
            text: App.tr("License")
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.bruma
        }

        Text {
            text: Publishing.licenseName(facts.review.license)
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }

        QQC2.Button {
            flat: true
            text: App.tr("Change")
            onClicked: licenses.picking = true
        }
    }

    PublishLicenses {
        id: licenses

        Layout.fillWidth: true
        review: facts.review
    }

    Text {
        Layout.fillWidth: true
        Layout.topMargin: Look.e3
        text: App.tr("Say it yourself: what does “%1” have?", facts.publish.name)
        wrapMode: Text.WordWrap
        font.family: Look.letraTitulo
        font.pixelSize: Look.tAparte
        font.weight: Font.DemiBold
        color: Look.escarcha
    }

    PublishAnswer {
        Layout.fillWidth: true
        text: App.tr("Flashes: lights that blink fast")
        value: facts.publish.manifest.photosensitive ?? undefined
        onSaid: yes => {
            facts.publish.page.change("/manifest/photosensitive", yes);
            facts.review.refresh();
        }
    }

    PublishAnswer {
        Layout.fillWidth: true
        text: App.tr("Adult content")
        value: facts.publish.manifest.content ? facts.publish.manifest.content.rating === "mature" : undefined
        onSaid: yes => {
            facts.publish.page.change("/manifest/content", Object.assign({}, facts.publish.manifest.content ?? {}, {
                "rating": yes ? "mature" : "general"
            }));
            facts.review.refresh();
        }
    }

    Nota {
        Layout.fillWidth: true
        text: App.tr("Asked the first time you publish this wallpaper; whoever puts it on sees the notice first.")
    }
}
