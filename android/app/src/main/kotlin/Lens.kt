package io.github.wallpaper_rs

import android.Manifest
import android.annotation.SuppressLint
import android.content.Context
import android.content.pm.PackageManager
import android.graphics.ImageFormat
import android.graphics.SurfaceTexture
import android.hardware.camera2.CameraCaptureSession
import android.hardware.camera2.CameraCharacteristics
import android.hardware.camera2.CameraDevice
import android.hardware.camera2.CameraManager
import android.hardware.camera2.CaptureRequest
import android.hardware.camera2.params.OutputConfiguration
import android.hardware.camera2.params.SessionConfiguration
import android.media.ImageReader
import android.os.Handler
import android.os.HandlerThread
import android.os.SystemClock
import android.util.Size
import android.view.Surface
import android.view.TextureView
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import java.util.concurrent.Executor

typealias Sight = (ByteArray, IntArray) -> Unit

interface Lens {
    fun present(context: Context): Boolean

    fun allowed(context: Context): Boolean

    @Composable
    fun asker(answer: (Boolean) -> Unit): () -> Unit

    @Composable
    fun View(modifier: Modifier, seen: Sight)
}

private const val WIDEST = 1280
private const val PAUSE = 150L

object Camera : Lens {
    private fun Context.cameras() = getSystemService(CameraManager::class.java)

    private fun Context.back(): String? = cameras()?.let { manager ->
        manager.cameraIdList.firstOrNull { manager.getCameraCharacteristics(it).get(CameraCharacteristics.LENS_FACING) == CameraCharacteristics.LENS_FACING_BACK }
            ?: manager.cameraIdList.firstOrNull()
    }

    override fun present(context: Context) =
        context.packageManager.hasSystemFeature(PackageManager.FEATURE_CAMERA_ANY) && runCatching { context.back() != null }.getOrDefault(false)

    override fun allowed(context: Context) = context.checkSelfPermission(Manifest.permission.CAMERA) == PackageManager.PERMISSION_GRANTED

    @Composable
    override fun asker(answer: (Boolean) -> Unit): () -> Unit {
        val launcher = rememberLauncherForActivityResult(ActivityResultContracts.RequestPermission(), answer)
        return { launcher.launch(Manifest.permission.CAMERA) }
    }

    @Composable
    override fun View(modifier: Modifier, seen: Sight) {
        val context = LocalContext.current
        val id = remember { context.back() } ?: return
        val size = remember(id) { context.chosen(id) }
        val shot = remember(id) { Shot(context, id, size, seen) }
        DisposableEffect(shot) { onDispose(shot::close) }
        AndroidView({ TextureView(it).apply { surfaceTextureListener = shot } }, modifier.aspectRatio(size.height.toFloat() / size.width))
    }

    private fun Context.chosen(id: String): Size {
        val sizes = cameras()!!.getCameraCharacteristics(id).get(CameraCharacteristics.SCALER_STREAM_CONFIGURATION_MAP)!!.getOutputSizes(ImageFormat.YUV_420_888)
        return sizes.filter { it.width <= WIDEST }.maxByOrNull { it.width * it.height } ?: sizes.minBy { it.width * it.height }
    }
}

private class Shot(private val context: Context, private val id: String, private val size: Size, private val seen: Sight) : TextureView.SurfaceTextureListener {
    private val thread = HandlerThread("lens").apply { start() }
    private val handler = Handler(thread.looper)
    private val run = Executor { handler.post(it) }
    private val reader = ImageReader.newInstance(size.width, size.height, ImageFormat.YUV_420_888, 2)
    private var device: CameraDevice? = null
    private var preview: Surface? = null
    private var frame = ByteArray(0)
    private val facts = IntArray(3)
    private var last = 0L
    @Volatile private var closed = false

    init {
        reader.setOnImageAvailableListener({ it.acquireLatestImage()?.use(::look) }, handler)
    }

    private fun look(image: android.media.Image) {
        val now = SystemClock.uptimeMillis()
        if (closed || now - last < PAUSE) return
        last = now
        val plane = image.planes[0]
        val buffer = plane.buffer
        val need = plane.rowStride * image.height
        if (frame.size != need) frame = ByteArray(need)
        buffer.get(frame, 0, minOf(buffer.remaining(), need))
        facts[0] = image.width
        facts[1] = image.height
        facts[2] = plane.rowStride
        seen(frame, facts)
    }

    @SuppressLint("MissingPermission")
    override fun onSurfaceTextureAvailable(texture: SurfaceTexture, width: Int, height: Int) {
        texture.setDefaultBufferSize(size.width, size.height)
        val surface = Surface(texture).also { preview = it }
        val manager = context.getSystemService(CameraManager::class.java) ?: return
        runCatching {
            manager.openCamera(id, run, object : CameraDevice.StateCallback() {
                override fun onOpened(camera: CameraDevice) = if (closed) camera.close() else started(camera, surface)

                override fun onDisconnected(camera: CameraDevice) = camera.close()

                override fun onError(camera: CameraDevice, error: Int) = camera.close()
            })
        }
    }

    private fun started(camera: CameraDevice, surface: Surface) {
        device = camera
        val outputs = listOf(OutputConfiguration(surface), OutputConfiguration(reader.surface))
        val callback = object : CameraCaptureSession.StateCallback() {
            override fun onConfigured(session: CameraCaptureSession) {
                if (closed) return
                val request = camera.createCaptureRequest(CameraDevice.TEMPLATE_PREVIEW).apply {
                    addTarget(surface)
                    addTarget(reader.surface)
                    set(CaptureRequest.CONTROL_AF_MODE, CaptureRequest.CONTROL_AF_MODE_CONTINUOUS_PICTURE)
                }.build()
                runCatching { session.setRepeatingRequest(request, null, handler) }
            }

            override fun onConfigureFailed(session: CameraCaptureSession) = session.close()
        }
        runCatching { camera.createCaptureSession(SessionConfiguration(SessionConfiguration.SESSION_REGULAR, outputs, run, callback)) }
    }

    override fun onSurfaceTextureSizeChanged(texture: SurfaceTexture, width: Int, height: Int) = Unit

    override fun onSurfaceTextureDestroyed(texture: SurfaceTexture): Boolean {
        close()
        return true
    }

    override fun onSurfaceTextureUpdated(texture: SurfaceTexture) = Unit

    fun close() {
        if (closed) return
        closed = true
        handler.post {
            device?.close()
            device = null
            reader.close()
            preview?.release()
            thread.quitSafely()
        }
    }
}
