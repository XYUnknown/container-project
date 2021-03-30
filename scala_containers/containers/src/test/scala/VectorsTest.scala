import vector.Vec
import vector.UniqueVec
import vector.SortedVec
import vector.UniqueSortedVec
import scala.collection.immutable.Vector
import org.scalatest.funsuite.AnyFunSuite
    
class VecTest extends AnyFunSuite {
    test("Vec.new") {
        val vec = new Vec[Int](v = Vector.empty)
        assert(vec.size == 0)
    }

    test("Vec.appended") {
        var vec = new Vec[Int](v = Vector.empty)
        for ( a <- 0 until 10){
            vec = vec.appended(a)
            vec = vec.appended(a)
        }
        assert(vec.size == 20)
    }

    test("UniqueVec.appended") {
        var vec = new UniqueVec[Int](v = Vector.empty)
        for (a <- 0 until 10){
            vec = vec.appended(a)
            vec = vec.appended(a)
        }
        assert(vec.size == 10)
        assert(vec.isUnique)
    }

    test("UniqueVec.insert") {
        var vec = new UniqueVec[Int](v = Vector.empty)
        for (a <- 0 until 10){
            vec = vec.appended(a)
        }
        for (a <- 0 until 10){
            vec = vec.insert(a, a)
        }
        assert(vec.size == 10)
        assert(vec.isUnique)
    }

    test("SortedVec.appended") {
        var vec = new SortedVec[Int](v = Vector.empty)
        for (a <- 0 until 10){
            vec = vec.appended(9 - a)
            vec = vec.appended(9 - a)
        }
        assert(vec.size == 20)
        assert(vec.isSorted)
    }

    test("SortedVec.insert") {
        var vec = new SortedVec[Int](v = Vector.empty)
        for (a <- 0 until 10 ){
            vec = vec.appended(9 - a)
        }
        for (a <- 0 until 10) {
            vec = vec.insert(9 - a, a)
        }
        assert(vec.size == 20)
        assert(vec.isSorted)
    }

    test("UniqueSortedVec.appended") {
        var vec = new UniqueSortedVec[Int](v = Vector.empty)
        for ( a <- 0 until 10){
            vec = vec.appended(9 - a)
            vec = vec.appended(9 - a)
        }
        assert(vec.size == 10)
        assert(vec.isSorted)
        assert(vec.isUnique)
    }

    test("UniqueSortedVec.insert") {
        var vec = new UniqueSortedVec[Int](v = Vector.empty)
        for (a <- 0 until 10 ){
            vec = vec.appended(9 - a)
        }
        for (a <- 0 until 10) {
            vec = vec.insert(9 - a, a)
        }
        assert(vec.size == 10)
        assert(vec.isSorted)
        assert(vec.isUnique)
    }
}