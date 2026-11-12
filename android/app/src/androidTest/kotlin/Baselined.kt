package io.github.wallpaper_rs

import androidx.test.internal.runner.junit4.AndroidJUnit4ClassRunner
import java.io.File
import org.junit.runners.model.FrameworkMethod
import org.junit.runners.model.MultipleFailureException
import org.junit.runners.model.Statement

@Target(AnnotationTarget.CLASS, AnnotationTarget.FUNCTION)
@Retention(AnnotationRetention.RUNTIME)
annotation class CleanApp

open class Baselined(klass: Class<*>) : AndroidJUnit4ClassRunner(klass) {
    override fun methodBlock(method: FrameworkMethod): Statement {
        Baseline.ready()
        val test = super.methodBlock(method)
        val clean = method.getAnnotation(CleanApp::class.java) != null || testClass.getAnnotation(CleanApp::class.java) != null
        return object : Statement() {
            override fun evaluate() {
                val owner = "${testClass.name}.${method.name}"
                val baseline = Baseline.begin(clean)
                val failures = mutableListOf<Throwable>()
                try {
                    test.evaluate()
                } catch (failure: Throwable) {
                    failures += failure
                }
                Baseline.end(owner, baseline)?.let { failures += it }
                MultipleFailureException.assertEmpty(failures)
            }
        }
    }
}

object Baseline {
    private const val SETTLE_MS = 3_000L
    private val known: MutableList<Seen<*>> by lazy {
        File(Device.context.cacheDir, "render-test").deleteRecursively()
        storage()
        Device.normalize()
        Probes.all().map { it.capture() }.toMutableList()
    }

    fun ready() = known.size

    fun begin(clean: Boolean): List<Seen<*>> {
        Device.sanctioned.clear()
        storage()
        Device.batteryReset()
        Device.powerReset()
        known.filterNot { it.same() }.forEach { it.restore() }
        if (clean) wipe()
        Device.sanctioned.clear()
        if (clean) Device.sanctioned += Probes.PREFERENCES
        return known
    }

    fun end(owner: String, baseline: List<Seen<*>>): Throwable? {
        val deadline = System.currentTimeMillis() + SETTLE_MS
        while (baseline.any { !it.same() } && System.currentTimeMillis() < deadline) Thread.sleep(50)
        val changed = baseline.filterNot { it.same() }
        val unsanctioned = changed.filter { it.name !in Device.sanctioned }
        val message = unsanctioned.joinToString("; ") { "${it.name} was ${it.was()} and is ${it.now()}" }
        changed.forEach { it.restore() }
        changed.filterNot { it.same() }.filter { it.name in Device.sanctioned }.forEach { stuck ->
            known[known.indexOf(stuck)] = stuck.rebased()
        }
        Device.powerReset()
        Device.sanctioned.clear()
        return if (unsanctioned.isEmpty()) null else AssertionError("$owner left global state changed: $message")
    }

    private fun wipe() {
        File(Device.context.cacheDir, "render-test").deleteRecursively()
        Probes.forgetPreferences()
        storage()
    }
}
