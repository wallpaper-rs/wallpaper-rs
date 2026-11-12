package io.github.wallpaper_rs

import android.content.Context
import androidx.work.BackoffPolicy
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.ListenableWorker
import androidx.work.PeriodicWorkRequest
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.util.concurrent.TimeUnit
import org.json.JSONArray
import org.json.JSONObject

class ProviderCheck(context: Context, params: WorkerParameters) : Worker(context, params) {
    override fun doWork(): ListenableWorker.Result {
        if (!ProviderWork.wanted()) return ListenableWorker.Result.success().also { ProviderWork.schedule(applicationContext) }
        val request = JSONObject().put("op", "provider_sync").put("id", "").put("force", false)
        val done = runCatching { JSONArray(Native.account(request.toString())) }.getOrNull() ?: return ListenableWorker.Result.retry()
        val brought = List(done.length()) { done.optJSONObject(it)?.optInt("brought") ?: 0 }.sum()
        if (brought > 0) Render.post { Wallpaper.reload(false) }
        return ListenableWorker.Result.success()
    }
}

object ProviderWork {
    const val WORK = "providers"
    private const val HOURS = 6L

    fun wanted() = runCatching {
        val listed = JSONObject(Native.account("{\"op\": \"providers\"}")).optJSONArray("providers")
        (0 until (listed?.length() ?: 0)).any { listed?.getJSONObject(it)?.optBoolean("on") == true }
    }.getOrDefault(false)

    fun schedule(context: Context, on: Boolean = wanted()) {
        val work = WorkManager.getInstance(context)
        if (!on) {
            work.cancelUniqueWork(WORK)
            return
        }
        val request = PeriodicWorkRequest.Builder(ProviderCheck::class.java, HOURS, TimeUnit.HOURS)
            .setConstraints(Sync.constraints())
            .setBackoffCriteria(BackoffPolicy.EXPONENTIAL, 30, TimeUnit.MINUTES)
            .build()
        work.enqueueUniquePeriodicWork(WORK, ExistingPeriodicWorkPolicy.UPDATE, request)
    }
}
