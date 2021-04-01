import property.DefaultVec
import property.UniqueVec
import property.SortedVec
import scala.collection.immutable.Vector
import org.scalatest.funsuite.AnyFunSuite

class VecPropTets extends AnyFunSuite {
    test("Vec.size") {
        val v = new DefaultVec[Int]
        assert(v.size(Vector.empty) == 0)
    }

    test("Vec.appeneded") {
        val v = new DefaultVec[Int]
        var vec = Vector[Int]()
        for ( a <- 0 until 10){
            vec = v.appended(vec, a)
            vec = v.appended(vec, a)
        }
        assert(v.size(vec) == 20)
    }

    test("UniqueVec.appeneded") {
        val v = new UniqueVec[Int]
        var vec = Vector[Int]()
        for ( a <- 0 until 10){
            vec = v.appended(vec, a)
            vec = v.appended(vec, a)
        }
        assert(v.size(vec) == 10)
        assert(v.assertionU(vec))
    }
    
    test("SortedVec.appeneded") {
        val v = new SortedVec[Int]
        var vec = Vector[Int]()
        for ( a <- 0 until 10){
            vec = v.appended(vec, 9 - a)
            vec = v.appended(vec, 9 - a)   
        }
        assert(v.size(vec) == 20)
        assert(v.assertionS(vec))
    }
}