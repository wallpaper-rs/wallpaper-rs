import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: budget

    required property var tools
    readonly property var census: SceneSwarmCensus.census?.system === tools.system ? SceneSwarmCensus.census : null
    readonly property var glow: {
        const g = SceneSwarmGlow.info?.system === tools.system ? SceneSwarmGlow.info : null;
        return g && (g.lit || g.refract) ? g : null;
    }

    readonly property string output: tools.page.editor.output
    readonly property var here: (live?.systems ?? []).find(s => s.id === tools.system) ?? null
    readonly property var ways: ({
            "compute": App.tr("the graphics card runs compute shaders"),
            "software": App.tr("the graphics are drawn by software, without a graphics card"),
            "no_compute": App.tr("the graphics card has no compute shaders"),
            "chosen": App.tr("it was chosen so")
        })
    property var live: null

    function ask() {
        if (output === "") {
            live = null;
            return;
        }
        App.query("GetParticleStats", [output], "Daemon1", reply => budget && (budget.live = reply), () => {
            if (!budget)
                return;
            budget.live = null;
            if (budget.census === null)
                budget.measure();
        });
    }

    function measure() {
        SceneSwarmCensus.measure(tools.page, tools.system, null);
    }

    Connections {
        function onViewChanged() {
            if (SceneSwarmCensus.shown)
                again.restart();
        }

        target: budget.tools.page.editor
    }

    Component.onCompleted: ask()
    onOutputChanged: ask()

    Timer {
        interval: 2000
        repeat: true
        running: budget.output !== "" && budget.visible
        onTriggered: budget.ask()
    }

    Timer {
        id: again

        interval: 300
        onTriggered: budget.measure()
    }

    Kirigami.Heading {
        level: 4
        text: App.tr("Particle budget")
    }

    SceneQuality {
        page: budget.tools.page
        again: App.tr("Restart systems")
        onPicked: budget.measure()
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: budget.here !== null
        wrapMode: Text.Wrap
        text: budget.live ? App.tr("In the preview: %1 alive of %2 slots, simulated on %3 because %4", budget.here?.alive ?? 0, budget.live.slots, budget.live.path === "gpu" ? App.tr("the graphics card") : App.tr("the processor"), budget.ways[budget.live.reason] ?? budget.live.reason) : ""
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: budget.live !== null
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: budget.live ? App.tr("The preview draws at quality %1 on a graphics card of class %2", String(budget.live.quality).toUpperCase(), String(budget.live.tier).slice(1)) : ""
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: budget.live === null
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("No preview is running, so the counts below are an estimate made by the editor")
    }

    QQC2.CheckBox {
        text: App.tr("Simulate on the processor and show the particles over the preview")
        checked: SceneSwarmCensus.shown
        onToggled: {
            SceneSwarmCensus.shown = checked;
            if (checked)
                budget.measure();
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: budget.census !== null && (SceneSwarmCensus.shown || budget.live === null)
        wrapMode: Text.Wrap
        text: App.tr("Estimate of the editor, not of the preview: %1 alive of %2 at %3 s; %4 slots on the processor", budget.census?.alive ?? 0, budget.census?.capacity ?? 0, Math.round((budget.census?.time ?? 0) * 10) / 10, budget.census?.slots ?? 0)
    }

    SceneRepeater {
        id: listed3

        rows: budget.census?.levels ?? []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed3.rows[index]

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("%1: %2 % of births, %3 neighbours, %4 trail points, %5 rope points between particles", modelData.quality.toUpperCase(), Math.round(modelData.births * 100), modelData.neighbours, modelData.trail, modelData.subdivide)
        }
    }

    SceneRepeater {
        id: listed2

        rows: budget.census?.tiers ?? []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed2.rows[index]

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("%1: %2 slots on the GPU, %3 on the processor, %4 trail points, %5 neighbours, %6 rope segments", modelData.tier.toUpperCase(), modelData.gpu, modelData.cpu, modelData.trail, modelData.neighbours, modelData.rope)
        }
    }

    SceneRepeater {
        id: listed

        rows: glow ? glow.levels : []

        delegate: QQC2.Label {
            required property int index
            readonly property var modelData: listed.rows[index]

            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: App.tr("%1: lit by up to %2 lights; refraction %3", modelData.quality.toUpperCase(), modelData.lights, modelData.refract === null ? App.tr("off") : modelData.refract === 1 ? App.tr("at full size") : App.tr("at half size"))
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: budget.census !== null
        wrapMode: Text.Wrap
        font: Kirigami.Theme.smallFont
        text: App.tr("Phones and older computers fall on T0 or T1, which keep fewer slots, neighbours, trail points and rope segments, light particles with up to 8 lights and refract only from Q2; software rendering runs on the processor.")
    }
}
