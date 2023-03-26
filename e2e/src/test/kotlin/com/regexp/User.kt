package com.regexp

import com.codeborne.selenide.Condition
import com.thoughtworks.gauge.Step
import com.codeborne.selenide.Selenide.*

class User {
    @Step("ユーザーページを開く")
    fun OpenUserPage() {
        open("http://localhost:3000")
    }

    @Step("ユーザーテーブルが表示されている")
    fun shouldDisplayUserTable() {
        `$`(".user-table").shouldBe(Condition.exist)
        val header = `$`("table").`$`("thead").`$`("tr").`$$`("th")
        header[0].shouldBe(Condition.text("ID"))
        header[1].shouldBe(Condition.text("名前"))
        header[2].shouldBe(Condition.text("メールアドレス"))
    }

    @Step("ID, ユーザー名, メールアドレスが表示されている")
    fun shouldDisplayUser() {
        val user = `$`("table").`$`("tbody").`$`("tr").`$$`("td")
        user.get(0).shouldHave(Condition.text("1"))
        user.get(1).shouldHave(Condition.text("山田太郎"))
        user.get(2).shouldHave(Condition.text("test1@example.com"))
    }
}