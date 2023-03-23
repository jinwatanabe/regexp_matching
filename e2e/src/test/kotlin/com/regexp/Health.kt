package com.regexp

import com.codeborne.selenide.Condition
import com.codeborne.selenide.Selenide
import com.thoughtworks.gauge.Step

class HealthCheck {
    @Step("HelloWorldを出力する")
    fun DisplayHelloWorld() {
        println("HelloWorld")
    }

    @Step("ログイン画面を開く")
    fun OpenLoginPage() {
        Selenide.open("${Config.baseUrl}")
        shouldDisplayLogin()
    }

    @Step("ロゴが表示されている")
    fun shouldDisplayLogin() {
        Selenide.`$`(".lnXdpd").shouldBe(Condition.visible)
    }
}