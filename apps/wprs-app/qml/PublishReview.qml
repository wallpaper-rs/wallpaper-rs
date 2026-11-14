import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PublishStep {
    id: review

    required property var publish
    property var report: ({})
    readonly property var findings: (report.findings ?? []).slice().sort((a, b) => rank(a) - rank(b))
    readonly property int blocking: count("error")
    readonly property int warnings: count("warning")
    readonly property var platforms: report.platforms ?? ({})
    readonly property var manifest: publish.manifest
    readonly property bool flashesAnswered: manifest.photosensitive !== undefined && manifest.photosensitive !== null
    readonly property bool adultAnswered: manifest.content !== undefined && manifest.content !== null
    readonly property string license: manifest.license ?? ""
    readonly property bool ready: blocking === 0 && flashesAnswered && adultAnswered && license !== "" && lineage.confirmed
    readonly property alias lineage: lineage
    readonly property alias go: go

    function rank(finding) {
        return ["error", "warning", "note"].indexOf(finding.level);
    }

    function count(level) {
        return (report.findings ?? []).filter(f => f.level === level).length;
    }

    function refresh() {
        if (!publish.editing)
            return;
        publish.reload();
        report = JSON.parse(publish.page.editor.rig("", JSON.stringify({
            "review": {}
        })) || "{}");
        lineage.inspect(publish.page.editor.folder);
    }

    function summary() {
        if (blocking > 0)
            return (blocking === 1 ? App.tr("One problem keeps it from being published") : App.tr("%1 problems keep it from being published", blocking)) + (warnings === 0 ? "" : warnings === 1 ? App.tr("; there is one warning") : App.tr("; there are %1 warnings", warnings));
        if (warnings > 0)
            return warnings === 1 ? App.tr("One warning; nothing keeps it from being published") : App.tr("%1 warnings; nothing keeps it from being published", warnings);
        return App.tr("Nothing to report: the package passes every check.");
    }

    function why() {
        const parts = [[blocking > 0, App.tr("Fix the problems"), App.tr("fix the problems")], [!flashesAnswered || !adultAnswered, App.tr("Answer the two questions"), App.tr("answer the two questions")], [license === "", App.tr("Choose a license"), App.tr("choose a license")], [!lineage.confirmed, App.tr("Confirm the copy"), App.tr("confirm the copy")]].filter(part => part[0]).map((part, index) => part[index === 0 ? 1 : 2]);
        return parts.length === 0 ? "" : App.tr("%1 to go on", parts.join(App.tr(" and ")));
    }

    function goTo(pointer) {
        publish.close();
        publish.page.jump(pointer);
    }

    number: 1
    title: App.tr("Review")
    done: publish.editing ? App.tr("Reviewed: <b>%1, version %2</b>", Publishing.licenseName(license), Publishing.escaped(publish.version)) : App.tr("A finished package: <b>%1</b>", Publishing.escaped(Publishing.base(publish.file)))
    mode: publish.at === 1 ? "abierto" : "hecho"
    changeable: publish.editing && publish.result === null
    onChange: publish.advance(1)

    Estado {
        Layout.fillWidth: true
        text: review.summary()
        tone: review.blocking > 0 ? "coral" : review.warnings > 0 ? "ambar" : "brote"
    }

    Repeater {
        model: review.findings

        delegate: PublishFinding {
            required property var modelData
            required property int index

            Layout.fillWidth: true
            finding: modelData
            first: index === 0
            onGo: review.goTo(modelData.pointer)
        }
    }

    Nota {
        Layout.fillWidth: true
        visible: review.blocking > 0
        text: App.tr("“Go to the problem” takes you to the editor; when you come back, this page is as you left it.")
    }

    PublishFacts {
        Layout.fillWidth: true
        review: review
    }


    SceneLineage {
        id: lineage

        Layout.fillWidth: true
    }

    PublishMore {
        PublishVerify {
            Layout.fillWidth: true
            review: review
        }
    }

    RowLayout {
        spacing: Look.e3

        QQC2.Button {
            id: go

            highlighted: true
            text: App.tr("Go on")
            enabled: review.ready
            Keys.onReturnPressed: clicked()
            onClicked: review.publish.advance(2)
        }

        Text {
            Layout.fillWidth: true
            visible: !review.ready
            text: review.why()
            wrapMode: Text.WordWrap
            font.family: Look.letraTexto
            font.pixelSize: Look.tPie
            color: Look.bruma
        }

        KeyCap {
            visible: review.ready
            key: "Intro"
        }
    }
}
