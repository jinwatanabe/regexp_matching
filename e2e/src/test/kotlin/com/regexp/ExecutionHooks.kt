package com.regexp

import com.codeborne.selenide.Configuration
import com.codeborne.selenide.Selenide
import com.thoughtworks.gauge.AfterScenario
import com.thoughtworks.gauge.BeforeSuite

class ExecutionHooks {
    @BeforeSuite
    fun setup() {
        System.setProperty("chromeoptions.args", "--no-sandbox,--disable-dev-shm-usage,--remote-allow-origins=*")

        Configuration.headless =
            System.getenv("SELENIDE_HEADLESS")
                ?.toBoolean()
//                ?: true
                ?: false

        Configuration.fastSetValue = true

        if (!System.getProperties().containsKey("selenide.timeout")) {
            Configuration.timeout = Config.selenide.timeout
        }
    }

    @AfterScenario
    fun after() {
        Selenide.closeWindow()
    }
}