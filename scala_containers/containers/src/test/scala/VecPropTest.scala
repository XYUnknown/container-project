import property.Vec
import property.UniqueSortedVec
import scala.collection.immutable.Vector
import org.scalatest.funsuite.AnyFunSuite

class VecPropTets extends AnyFunSuite {
    test("Vec.size") {
        val v = new Vec[Int]
        assert(v.size(Vector.empty) == 0)
    }

    test("Vec.appeneded") {
        val v = new Vec[Int]
        var vec = Vector[Int]()
        for ( a <- 0 until 10){
            vec = v.appended(vec, a)
            vec = v.appended(vec, a)
        }
        assert(v.size(vec) == 20)
    }

    test("UniqueSortedVec.appeneded") {
        val v = new Vec[Int]
        var vec = Vector[Int]()
        for ( a <- 0 until 10){
            vec = v.appended(vec, a)
            vec = v.appended(vec, a)
        }
        assert(v.size(vec) == 20)
    }
}