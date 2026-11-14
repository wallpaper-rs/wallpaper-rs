pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    function places() {
        return [
            ["grants", App.tr("What scenes may use")],
            ["sound", App.tr("Let scenes play their sounds")],
            ["sound", App.tr("Let scenes react to the sound that plays")],
            ["sound", App.tr("Silence wallpapers while another application plays sound")],
            ["media", App.tr("What plays now")],
            ["media", App.tr("Fetch covers from the internet")],
            ["lights", App.tr("Send colors to RGB lighting")],
            ["lights", App.tr("OpenRGB server")],
            ["lights", App.tr("Connect")],
            ["lights", App.tr("While a window covers the wallpaper and it pauses")],
            ["lights", App.tr("Test every device")],
            ["advanced", App.tr("Graphics card")],
            ["advanced", App.tr("Restart the daemon now")],
            ["advanced", App.tr("HDR, extra bright colors")],
            ["advanced", App.tr("Depth quality")],
            ["advanced", App.tr("If something does not show")],
            ["advanced", App.tr("Reload the settings")],
            ["advanced", App.tr("Check the whole library")],
            ["advanced", App.tr("For tests")],
            ["depth", App.tr("Allow downloading the depth model")],
            ["depth", App.tr("Fill behind near objects")],
            ["depth", App.tr("Depth layers")],
            ["version", App.tr("Version")],
            ["version", App.tr("Graphics")],
            ["version", App.tr("Data model")],
            ["version", App.tr("Configuration")],
            ["testing", App.tr("Surface name")],
            ["testing", App.tr("Draw with the processor")],
            ["testing", App.tr("Depth drawn by the processor")]
        ].map(pair => ({
                "place": pair[0],
                "text": pair[1]
            }));
    }
}
