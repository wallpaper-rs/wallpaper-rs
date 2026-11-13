package io.github.wallpaper_rs

import kotlin.concurrent.thread

class Stalling(private val millis: Long = STALL_MS) : AutoCloseable {
    private val socket = Loopback.socket(false)
    val url get() = "http://127.0.0.1:${socket.localPort}/repo/"

    init {
        thread(isDaemon = true) {
            while (!socket.isClosed) {
                val client = runCatching { socket.accept() }.getOrNull() ?: break
                thread(isDaemon = true) {
                    client.use {
                        runCatching {
                            Thread.sleep(millis)
                            it.getOutputStream().write("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".toByteArray())
                        }
                    }
                }
            }
        }
    }

    override fun close() = socket.close()

    private companion object {
        const val STALL_MS = 2_000L
    }
}
