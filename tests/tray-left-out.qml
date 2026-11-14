import QtQuick
import QtTest
import "plasmoid/contents/ui"

TestCase {
    function shown(info) {
        return Object.assign({
            "quality": "q2",
            "quality_reason": "user",
            "quality_cost": 0,
            "quality_budget": 0,
            "quality_auto": "",
            "quality_frame_ms": null,
            "quality_period_ms": null,
            "quality_step": "",
            "quality_pace": 1,
            "fps": 60,
            "left_out": []
        }, info);
    }

    function test_cost_notes() {
        const over = {
            "quality_cost": 6,
            "quality_budget": 4
        };
        const timed = {
            "quality_frame_ms": 25,
            "quality_period_ms": 16.7,
            "quality_step": "steady"
        };
        const cost = "This scene costs 6.0 units a frame at this level, more than the 4 this device is estimated to hold: frames may drop and battery use rise.";
        const slow = "Its frames take 25.0 ms on the graphics card, more than the 16.7 ms a frame has at 60 frames a second: frames drop and battery use rises.";
        compare(reasons.note(shown(Object.assign({
            "quality_auto": "lower"
        }, over))), cost + " Automatic quality would lower it.");
        compare(reasons.note(shown(Object.assign({
            "quality_auto": "lower"
        }, over, timed))), slow + " Automatic quality would lower it.");
        compare(reasons.note(shown(Object.assign({
            "quality_auto": ""
        }, over, timed))), "");
        compare(reasons.note(shown(Object.assign({
            "quality_auto": "stop",
            "left_out": ["a"]
        }, over))), cost + " Automatic quality would keep it still at its poster. Effects left out to protect the graphics card stay out at every level: 1");
        verify(reasons.note(shown({
            "quality_auto": "stop"
        })).startsWith("This scene draws more model triangles"));
        verify(reasons.note(shown({
            "quality_auto": "still"
        })).startsWith("This scene is too heavy for this device"));
        compare(reasons.note(shown({})), "");
    }

    function test_level_lines() {
        const timed = shown({
            "quality_frame_ms": 25,
            "quality_period_ms": 33.3,
            "quality_step": "halved",
            "quality_pace": 2,
            "fps": 30
        });
        compare(reasons.level(timed, {}).split("\n"), ["Quality 2 of 3 at 30 frames a second: chosen by you", "Measured GPU time: 25.0 ms a frame, for a target of 33.3 ms. The frame rate was halved: frames took longer than the target and the level stays."]);
        compare(reasons.timing(shown({
            "quality_step": "model"
        })), "GPU time not measured yet. The level comes from the estimate until frames are measured.");
        compare(reasons.level(shown({
            "quality_auto": "still"
        }), {}).split("\n").length, 2);
    }

    LeftOut {
        id: reasons

        function i18n(text, ...values) {
            return values.reduce((done, value, at) => done.replace("%" + (at + 1), value), text);
        }
    }
}
