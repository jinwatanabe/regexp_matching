package com.regex

import com.codeborne.selenide.Configuration
import com.thoughtworks.gauge.BeforeSuite

class ExecutionHooks {
    @BeforeSuite
    fun setup() {
        // 設定リスト: https://peter.sh/experiments/chromium-command-line-switches/
        System.setProperty("chromeoptions.args", "--no-sandbox,--disable-dev-shm-usage,--remote-allow-origins=*")

        Configuration.headless =
            System.getenv("SELENIDE_HEADLESS")
                ?.toBoolean()
//                ?: true
                ?: false

        // 1文字づつ入力したい場合はfalseにする
        Configuration.fastSetValue = true

        if (!System.getProperties().containsKey("selenide.timeout")) {
            Configuration.timeout = Config.selenide.timeout
        }
    }
}