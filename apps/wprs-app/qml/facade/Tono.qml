pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function of(tone, lit, rest) {
        if (tone === "ambar")
            return lit ? Look.ambarHondo : Look.ambar;
        if (tone === "coral")
            return lit ? Look.coralHondo : Look.coral;
        if (tone === "brote")
            return lit ? Look.broteHondo : Look.brote;
        return rest;
    }

    function icon(tone) {
        return tone === "brote" ? "check-circle" : tone === "ambar" ? "warning" : tone === "coral" ? "warning-circle" : "info";
    }
}
