// Test
trait SchrodingerCat {
    def status : String = "not alive or dead"
    def talk : String = "say Hello"
}
trait LivingCat extends SchrodingerCat {
    override def status : String = "I stay alive: " + super.talk
    override def talk : String = "Meow meow" + super.talk
}

trait DeadCat extends SchrodingerCat {
    override def status : String = "I become dead from status: " + super.status
    override def talk : String = "Dead cat shouldn't talk, but I'm forced to talk: " + super.talk
}

class Cat1 extends LivingCat with DeadCat {
   override def status : String = super.status
}

class Cat2 extends DeadCat with LivingCat {
    override def status : String = super.status
}

object Main extends App {
    var cat1 = new Cat1
    var cat2 = new Cat2
    println(cat1.status)
    println(cat2.status)
}