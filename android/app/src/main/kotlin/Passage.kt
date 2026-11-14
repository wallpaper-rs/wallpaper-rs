package io.github.wallpaper_rs

import android.animation.ValueAnimator

object Passage {
    const val OWN = "own"
    const val FADE = "fade"
    const val NONE = "none"
    private const val LIGHT_ONLY = 1

    fun still(setting: String, system: Boolean = !ValueAnimator.areAnimatorsEnabled()) = when (setting) {
        "always" -> true
        "never" -> false
        else -> system
    }

    fun note(choice: String, still: Boolean, saving: Boolean, quality: Int?): Int = when {
        choice == NONE -> R.string.passage_cut
        still -> R.string.passage_still
        saving -> R.string.passage_saving
        choice == FADE -> R.string.passage_fade
        quality != null && quality <= LIGHT_ONLY -> R.string.passage_light
        else -> R.string.passage_own
    }
}
