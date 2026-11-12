package io.github.wallpaper_rs

import android.content.Context
import android.os.SystemClock
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequest
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import java.io.File
import org.json.JSONArray
import org.json.JSONObject

object Pending {
    private const val FILE = "converting.json"

    private fun file(context: Context) = File(context.noBackupFilesDir, FILE)

    @Synchronized
    fun all(context: Context): List<Conversion> {
        val text = runCatching { file(context).readText() }.getOrNull() ?: return emptyList()
        val list = runCatching { JSONArray(text) }.getOrNull() ?: return emptyList()
        return (0 until list.length()).mapNotNull { runCatching { read(list.getJSONObject(it)) }.getOrNull() }
    }

    @Synchronized
    fun add(context: Context, conversion: Conversion) = write(context, all(context).filter { it.uid != conversion.uid } + conversion)

    @Synchronized
    fun remove(context: Context, uid: String) {
        val kept = all(context)
        if (kept.any { it.uid == uid }) write(context, kept.filter { it.uid != uid })
    }

    private fun write(context: Context, list: List<Conversion>) {
        val target = file(context)
        if (list.isEmpty()) return run { target.delete() }
        val text = JSONArray(list.map(::json)).toString()
        runCatching { File(target.path + ".new").apply { writeText(text) }.renameTo(target) }
    }

    private fun json(conversion: Conversion) = JSONObject().put("uid", conversion.uid).put("path", conversion.path)
        .put("title", conversion.title).put("keep", conversion.keep).put("width", conversion.size.first)
        .put("height", conversion.size.second).put("asked", conversion.asked)

    private fun read(json: JSONObject) = Conversion(
        json.getString("uid"), json.getString("path"), json.getString("title"), json.getBoolean("keep"),
        json.getInt("width") to json.getInt("height"), json.getBoolean("asked"),
    )
}

class ConvertKeeper(context: Context, params: WorkerParameters) : Worker(context, params) {
    override fun doWork(): Result {
        Converting.resume(applicationContext)
        val end = SystemClock.elapsedRealtime() + Keeper.LONGEST_MS
        Thread.sleep(Keeper.SETTLE_MS)
        while (Converting.busy()) {
            if (isStopped || SystemClock.elapsedRealtime() > end) return Result.retry()
            Thread.sleep(Keeper.SETTLE_MS)
        }
        return Result.success()
    }
}

object Keeper {
    const val WORK = "convert-keep"
    const val SETTLE_MS = 1000L
    const val LONGEST_MS = 9 * 60 * 1000L

    fun request() = OneTimeWorkRequest.Builder(ConvertKeeper::class.java)
        .setConstraints(Constraints.Builder().setRequiresBatteryNotLow(true).build()).build()

    fun keep(context: Context) {
        WorkManager.getInstance(context).enqueueUniqueWork(WORK, ExistingWorkPolicy.KEEP, request())
    }
}
