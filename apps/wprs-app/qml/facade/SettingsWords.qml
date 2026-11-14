pragma Singleton
import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    readonly property var passages: ["own", "fade", "none"]
    readonly property var levels: ["auto", "min-q2", "min-q1", "q3", "q2", "q1", "q0"]
    readonly property var reasons: ({
            "cut": App.tr("Last change: at once, without a transition."),
            "own": App.tr("Last change: the wallpaper's own transition."),
            "chosen": App.tr("Last change: what you chose."),
            "reduced": App.tr("Last change: a plain fade, because of less motion."),
            "fallen": App.tr("Last change: a plain fade, because the wallpaper's transition could not play."),
            "plain": App.tr("Last change: a plain fade, neither wallpaper brings a transition.")
        })

    function passage(value) {
        return ({
                "general": App.tr("Like every screen"),
                "own": App.tr("Each wallpaper's own"),
                "fade": App.tr("Fade"),
                "none": App.trc("transition", "None")
            })[value] ?? value;
    }

    function level(value) {
        return ({
                "none": App.tr("Like every screen"),
                "auto": App.trc("scene quality", "Automatic"),
                "min-q2": App.tr("Automatic, never below high"),
                "min-q1": App.tr("Automatic, never below reduced"),
                "q3": App.trc("scene quality", "Highest"),
                "q2": App.tr("Always high"),
                "q1": App.tr("Always reduced"),
                "q0": App.tr("Save battery")
            })[value] ?? value;
    }

    function quality(several, output, info, options) {
        const general = String(options["playback.quality"] ?? "auto");
        const chosen = several ? String(info.quality_choice ?? "none") : general;
        const values = (several ? ["none"] : []).concat(["auto", "q0", "q3"]);
        if (!values.includes(chosen))
            values.push(chosen);
        const fixed = chosen.length === 2 && chosen[0] === "q";
        const lower = fixed && info.quality_auto === "lower";
        return {
            "key": several ? "playback.quality." + output : "playback.quality",
            "values": values,
            "words": values.map(value => level(value)),
            "current": values.indexOf(chosen),
            "note": fixed ? App.tr("You set it: it stays on battery") + (lower ? App.tr(", even though the automatic one would lower it now") : "") : ""
        };
    }

    function transition(several, output, info, options) {
        const general = String(options["transition.kind"] ?? "own");
        const state = info.transition ?? {};
        const chosen = several ? String(state.own ?? "general") : general;
        const values = (several ? ["general"] : []).concat(passages);
        return {
            "key": several ? "transition.kind." + output : "transition.kind",
            "values": values,
            "words": values.map(value => passage(value)),
            "current": Math.max(0, values.indexOf(chosen)),
            "why": reasons[state.why ?? ""] ?? ""
        };
    }
}
