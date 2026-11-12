package io.github.wallpaper_rs

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.Description
import org.junit.runner.RunWith
import org.junit.runner.notification.Failure
import org.junit.runner.notification.RunListener
import org.junit.runner.notification.RunNotifier
import org.junit.runners.model.FrameworkMethod

class Culprit {
    fun leaks() {
        Device.shell("settings put global animator_duration_scale 7")
    }

    fun helps() {
        Device.animations("7")
    }
}

@RunWith(Baselined::class)
class BaselineGuardTest {
    private fun animator() = Device.shell("settings get global animator_duration_scale").trim()

    private fun run(name: String): List<Failure> {
        val failures = mutableListOf<Failure>()
        val notifier = RunNotifier().apply {
            addListener(object : RunListener() {
                override fun testFailure(failure: Failure) {
                    failures += failure
                }
            })
        }
        object : Baselined(Culprit::class.java) {
            override fun computeTestMethods() = listOf(FrameworkMethod(Culprit::class.java.getMethod(name)))
            override fun describeChild(method: FrameworkMethod) = Description.createTestDescription(Culprit::class.java, name)
        }.run(notifier)
        return failures
    }

    @Test
    fun aTestThatLeavesStateBehindFailsNamingItselfAndTheStateIsRestored() {
        val before = animator()
        val failures = run("leaks")
        assertEquals(1, failures.size)
        val message = failures[0].exception.message.orEmpty()
        assertTrue(message, message.contains("Culprit.leaks") && message.contains("animation animator_duration_scale"))
        assertEquals(before, animator())
    }

    @Test
    fun stateChangedThroughTheHelpersIsRestoredWithoutFailing() {
        val before = animator()
        assertEquals(emptyList<Failure>(), run("helps"))
        assertEquals(before, animator())
    }

    @Test
    fun theBaselineEndsCleanWhenNothingChanged() {
        val baseline = Baseline.begin(false)
        assertNull(Baseline.end("Quiet.test", baseline))
        Device.shell("settings put global animator_duration_scale 5")
        assertNotNull(Baseline.end("Loud.test", baseline))
    }
}
