package property
import scala.collection.immutable.Vector
import scala.collection.Searching._

/*trait DefaultT[+A] {
    def contains [A1 >: A](elem: A1) : Boolean
    def size: Int
    def isEmpty: Boolean
    def appended [B >: A](elem: B)(implicit ord: Ordering[B]): DefaultT[B]
}

trait UniqueT[+A] extends DefaultT[A] {
    override def appended [B >: A](elem: B)(implicit ord: Ordering[B]): UniqueT[B] = {
        if (!this.contains(elem)) {
            super.appended(elem) // type mismatch
        } else {
            this
        }
    }
}*/

trait DefaultVecT[A] {
    def contains [A1 >: A](vec: Vector[A], elem: A1) : Boolean = {
        vec.contains(elem)
    }
    def size(vec: Vector[A]): Int = {
        vec.size
    }
    def isEmpty(vec: Vector[A]): Boolean = {
        vec.isEmpty
    }
    def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        vec.appended(elem)
    }
}

trait UniqueVecT[A] extends DefaultVecT[A] {
    override def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        if (!this.contains(vec, elem)) {
            super.appended(vec, elem)
        } else {
            vec
        }
    }
}

trait SortedVecT[A] extends DefaultVecT[A] {
    override def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        vec.search(elem) match {
            case Found(x) => {
                vec.take(x).appended(elem).appendedAll(vec.drop(x))
            }
            case InsertionPoint(x) => {
                vec.take(x).appended(elem).appendedAll(vec.drop(x))
            }
        }
    }
}

trait UniqueSortedVecT[A] 
    extends DefaultVecT[A] 
        with UniqueVecT[A]
        with SortedVecT[A] {
    override def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        super.appended(vec, elem)
    }
}

class Vec[A] extends DefaultVecT[A]{}
class UniqueVec[A] extends UniqueVecT[A]{}
class SortedVec[A] extends UniqueVecT[A]{}
class UniqueSortedVec[A] extends UniqueSortedVecT[A]{}
