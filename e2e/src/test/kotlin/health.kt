import com.thoughtworks.gauge.Step

class HealthCheck {
    @Step("HelloWorldを出力する")
    fun DisplayHelloWorld() {
        println("HelloWorld")
    }
}