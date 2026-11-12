import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: made

    required property var page
    readonly property var templates: ["color", "distort", "passes", "material", "particle"]

    QQC2.TextField {
        id: name

        Layout.fillWidth: true
        placeholderText: App.tr("Id of a new effect")
        validator: RegularExpressionValidator {
            regularExpression: /[a-z0-9_]{1,32}/
        }
    }

    QQC2.ComboBox {
        id: template

        model: [App.tr("A change of color"), App.tr("A bend around a point"), App.tr("Two passes"), App.tr("A material shader"), App.tr("A particle shader")]
    }

    QQC2.Button {
        icon.name: "document-new"
        text: App.tr("New effect")
        enabled: name.acceptableInput
        onClicked: {
            made.page.run("shader", {
                    "new": {
                        "name": name.text,
                        "template": made.templates[template.currentIndex]
                    }
                });
            name.text = "";
        }
    }
}
