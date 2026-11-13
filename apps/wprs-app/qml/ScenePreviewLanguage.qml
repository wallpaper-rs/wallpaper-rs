import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

RowLayout {
    id: language

    required property var page
    readonly property var languages: {
        language.page.view;
        return (JSON.parse(language.page.editor.rig("", JSON.stringify({
                        "strings": {}
                    })) || "{}").languages ?? []);
    }

    visible: languages.length > 1

    QQC2.Label {
        text: App.tr("Language of the preview")
    }

    QQC2.ComboBox {
        Layout.fillWidth: true
        model: [App.tr("Default language")].concat(language.languages.map(l => ManifestWords.language(l)))
        currentIndex: Math.max(0, language.languages.indexOf(SceneLook.language) + 1)
        Accessible.name: App.tr("Language of the preview")
        onActivated: index => {
            SceneLook.language = index === 0 ? "" : language.languages[index - 1];
            SceneLook.preview(language.page);
        }
    }
}
