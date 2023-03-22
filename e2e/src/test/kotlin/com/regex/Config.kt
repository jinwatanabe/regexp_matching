package com.regex

import io.github.cdimascio.dotenv.dotenv

object Config {
    val env by lazy { dotenv { ignoreIfMissing = true } }

//    val baseUrl
//        get() = env.get("BASE_URL") ?: "http://localhost:8000"

    object selenide {
        val timeout = env.get("SELENIDE_TIMEOUT")?.toLong() ?: 10000L
    }
}