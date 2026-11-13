package io.github.wallpaper_rs

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontVariation
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp

@OptIn(androidx.compose.ui.text.ExperimentalTextApi::class)
private fun variable(id: Int, weight: Int) =
    Font(id, FontWeight(weight), variationSettings = FontVariation.Settings(FontVariation.weight(weight)))

val Onest = FontFamily(listOf(400, 500, 600, 700).map { variable(R.font.onest, it) })
val RedHat = FontFamily(listOf(500, 600, 700).map { variable(R.font.red_hat_display, it) })

private const val TEXT_LEADING = 1.45f
private const val TITLE_LEADING = 1.3f

private fun body(size: Float, weight: Int = 400, leading: Float = TEXT_LEADING) =
    TextStyle(fontFamily = Onest, fontWeight = FontWeight(weight), fontSize = size.sp, lineHeight = (size * leading).sp)

private fun heading(size: Float, weight: Int = 700, tracking: TextUnit = TextUnit.Unspecified) = TextStyle(
    fontFamily = RedHat,
    fontWeight = FontWeight(weight),
    fontSize = size.sp,
    lineHeight = (size * TITLE_LEADING).sp,
    letterSpacing = tracking,
)

object Type {
    val foot = body(14f)
    val text = body(15.5f)
    val row = body(17f, 500, TITLE_LEADING)
    val tab = body(14f, 500, TITLE_LEADING)
    val button = heading(16f, 700, (-0.005).em)
    val word = heading(16f, 600)
    val group = heading(14f, 600)
    val fold = heading(15.5f, 600)
    val titleS = heading(21f, 700, (-0.015).em)
    val empty = heading(26f, 700, (-0.02).em)
    val titleM = heading(32f, 700, (-0.025).em)
    val titleL = heading(50f, 700, (-0.025).em)
    val digits = heading(44f, 700, 0.08.em)

    val material = Typography(
        displayLarge = titleL,
        displayMedium = digits,
        displaySmall = titleM,
        headlineLarge = titleM,
        headlineMedium = empty,
        headlineSmall = titleS,
        titleLarge = titleS,
        titleMedium = row,
        titleSmall = group,
        bodyLarge = row.copy(fontWeight = FontWeight.Normal),
        bodyMedium = text,
        bodySmall = foot,
        labelLarge = button,
        labelMedium = tab,
        labelSmall = foot,
    )
}
