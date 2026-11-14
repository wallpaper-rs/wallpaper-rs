import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Rectangle {
    id: more

    required property var pkg
    required property var catalog
    readonly property string key: pkg ? pkg.repo_id + "/" + pkg.id : ""
    readonly property alias version: version

    implicitWidth: Look.masFicha
    implicitHeight: lines.implicitHeight + 2 * Look.e4
    radius: Look.radio
    color: Look.noche
    border.width: Look.linea
    border.color: Look.nocheLinea
    Accessible.role: Accessible.Grouping
    Accessible.name: App.tr("Details of “%1”", pkg ? pkg.title : "")

    BrowseNotes {
        id: notes
    }

    ColumnLayout {
        id: lines

        x: Look.e4
        y: Look.e4
        width: parent.width - 2 * Look.e4
        spacing: Look.e3

        Text {
            Layout.fillWidth: true
            visible: text !== ""
            text: more.pkg ? (more.pkg.content ? App.plain(more.pkg.description) : more.pkg.description) : ""
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            lineHeight: 1.3
            font.family: Look.letraTexto
            font.pixelSize: Look.tCuerpo
            color: Look.escarcha
        }

        GridLayout {
            columns: 2
            columnSpacing: Look.e4
            rowSpacing: Look.e2

            BrowseTerm {
                text: App.tr("Authors")
            }

            BrowseTerm {
                text: more.pkg ? notes.author(more.pkg) + (more.pkg.license ? " · " + more.pkg.license : "") : ""
                lit: true
            }

            BrowseTerm {
                text: App.tr("Version")
            }

            QQC2.ComboBox {
                id: version

                readonly property var versions: more.pkg ? (more.pkg.versions ?? []).slice().reverse() : []

                model: versions.map((v, i) => i === 0 ? App.tr("%1, the newest", v) : v)
                currentIndex: Math.max(0, versions.indexOf(more.catalog.versions[more.key] ?? ""))
                onActivated: index => more.catalog.versions = Object.assign({}, more.catalog.versions, {
                        [more.key]: index === 0 ? "" : versions[index]
                    })
            }

            BrowseTerm {
                visible: votes.visible
                text: App.tr("Your vote")
            }

            BrowseVotes {
                id: votes

                visible: more.pkg !== null && more.catalog.voteRepo !== "" && more.pkg.repo_id === more.catalog.voteRepo
                counts: more.catalog.votes[more.key] ?? null
                shown: more.catalog.counted
                packageId: more.key
                onVoted: found => more.catalog.votes = Object.assign({}, more.catalog.votes, Object.fromEntries([[more.key, found]]))
            }
        }

        BrowseTerm {
            Layout.fillWidth: true
            visible: votes.visible
            text: App.tr("Votes go to your server with your account; it keeps only the counts and your own vote.")
            small: true
        }

        Estado {
            Layout.fillWidth: true
            visible: more.pkg !== null && more.pkg.photosensitive === true
            tone: "ambar"
            icon: "warning"
            text: more.pkg && more.pkg.flash_disabled ? App.tr("Has flashes, disabled: allow wallpapers that flash in the settings.") : App.tr("Has flashes or fast patterns that can trigger seizures in people with photosensitive epilepsy.")
        }

        Estado {
            Layout.fillWidth: true
            visible: text !== ""
            icon: "info"
            text: more.pkg && notes.asks(more.pkg).length > 0 ? App.tr("It %1; it asks you before it starts.", notes.asks(more.pkg).map(a => notes.phrases[a]).join(", ")) : ""
        }

        PrivacyNotice {
            Layout.fillWidth: true
            exposes: more.pkg && more.pkg.kind === "web" ? ["zone", "language", "gpu"] : []
        }
    }
}
