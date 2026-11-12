import QtQml

QtObject {
    function text(line) {
        const head = "the effect ", tail = " was left out: ";
        const at = line.indexOf(tail);
        if (!line.startsWith(head) || at < 0)
            return line;
        const why = line.slice(at + tail.length);
        const reasons = [["it is too slow for this graphics card", i18n("it is too slow for this graphics card")], ["it stopped the graphics card or the program twice", i18n("it stopped the graphics card or the program twice")], ["the graphics card or the program stopped twice", i18n("the graphics card or the program stopped twice while it built or drew this effect")], ["the graphics card or the program stopped", i18n("the graphics card or the program stopped while it drew this effect")], ["it kept the graphics card busy", i18n("it kept the graphics card busy for too long")], ["its shader could not be checked", i18n("its shader could not be checked")], ["the effects of the scene would do more than", i18n("the effects of the scene would do more work than this graphics card allows")], ["the scene has more than", i18n("the scene has more than 16 effects that keep a state")], ["its state textures would take", i18n("its state textures would take too much memory")], ["its textures would take more than", i18n("its textures would take too much memory")]];
        const found = reasons.find(r => why.startsWith(r[0]));
        if (!found || found[1].startsWith(found[0]))
            return line;
        return i18n("The effect %1 was left out: %2", line.slice(head.length, at), found[1]);
    }

    function privacy(exposes, sandbox) {
        const words = {
            "zone": i18n("your time zone"),
            "language": i18n("your language"),
            "gpu": i18n("what your graphics card can do"),
            "storage": i18n("data it keeps on this computer"),
            "network": i18n("the network, to reach the sites it names")
        };
        const used = exposes.length ? i18n("This web wallpaper uses %1. You can restrict each of them in its settings in the app.", exposes.map(name => words[name] ?? name).join(", ")) : "";
        return [used, sandbox === "reduced" ? i18n("Its web engine runs without part of its sandbox on this computer.") : ""].filter(t => t).join(" ");
    }

    function level(info, state) {
        const reasons = {
            "device": i18n("the most this graphics card allows"),
            "cost": i18n("lowered because the scene is heavy for this graphics card"),
            "battery": i18n("lowered on battery"),
            "saver": i18n("lowered while saving power"),
            "heat": i18n("lowered while the computer is hot"),
            "slow": i18n("lowered because its frames took too long on the graphics card"),
            "user": i18n("chosen by you")
        };
        const quality = info.quality_pace > 1 ? i18n("Quality %1 of 3 at %2 frames a second", info.quality.slice(1), info.fps) : i18n("Quality %1 of 3", info.quality.slice(1));
        const tier = String(state?.tier ?? "").slice(1);
        const card = !tier ? "" : state.tier_measured ? i18n("graphics card class %1 of 3, measured", tier) : i18n("graphics card class %1 of 3, from its model", tier);
        return [[reasons[info.quality_reason] ? quality + ": " + reasons[info.quality_reason] : quality, card].filter(t => t).join(" · "), timing(info), note(info)].filter(t => t).join("\n");
    }

    function timing(info) {
        const steps = {
            "model": i18n("The level comes from the estimate until frames are measured."),
            "steady": i18n("The level holds: frames fit, without enough room for the next level."),
            "climbed": i18n("The level went up: frames had time to spare."),
            "dropped": i18n("The level went down: frames took longer than the target."),
            "top": i18n("This is the highest level allowed now."),
            "held": i18n("The next level was measured and does not fit."),
            "still": i18n("Even the lowest level is far too slow, so the scene stays still at its poster."),
            "halved": i18n("The frame rate was halved: frames took longer than the target and the level stays."),
            "doubled": i18n("The frame rate went back up: frames had time to spare.")
        };
        const step = steps[info.quality_step ?? ""];
        if (!step)
            return "";
        if (info.quality_frame_ms === null || info.quality_frame_ms === undefined)
            return i18n("GPU time not measured yet. %1", step);
        return i18n("Measured GPU time: %1 ms a frame, for a target of %2 ms. %3", Number(info.quality_frame_ms).toFixed(1), Number(info.quality_period_ms).toFixed(1), step);
    }

    function note(info) {
        const over = info.quality_budget > 0 && info.quality_cost > info.quality_budget;
        const auto = info.quality_auto ?? "";
        const left = (info.left_out ?? []).length;
        const kept = left ? i18n("Effects left out to protect the graphics card stay out at every level: %1", left) : "";
        const cost = i18n("This scene costs %1 units a frame at this level, more than the %2 this device is estimated to hold: frames may drop and battery use rise.", Number(info.quality_cost).toFixed(1), Number(info.quality_budget).toFixed(0));
        const period = Number(info.quality_period_ms);
        const slow = i18n("Its frames take %1 ms on the graphics card, more than the %2 ms a frame has at %3 frames a second: frames drop and battery use rises.", Number(info.quality_frame_ms).toFixed(1), period.toFixed(1), period > 0 ? Math.round(1000 / period) : 0);
        const timed = info.quality_frame_ms !== null && info.quality_frame_ms !== undefined;
        const would = auto === "stop" ? i18n("Automatic quality would keep it still at its poster.") : i18n("Automatic quality would lower it.");
        if (auto === "lower")
            return [timed ? slow : cost, would, kept].filter(t => t).join(" ");
        if (auto === "stop" && over)
            return [cost, would, kept].filter(t => t).join(" ");
        if (auto === "stop")
            return [i18n("This scene draws more model triangles than this device holds: frames may drop and battery use rise. Automatic quality would keep it still at its poster."), kept].filter(t => t).join(" ");
        if (auto === "still")
            return [i18n("This scene is too heavy for this device, so automatic quality keeps it still at its poster. A fixed level plays it anyway, with fewer frames per second and more battery use."), kept].filter(t => t).join(" ");
        return "";
    }
}
