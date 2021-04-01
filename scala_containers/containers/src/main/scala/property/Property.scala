package property
import scala.collection.immutable.Vector
import scala.collection.Searching._


abstract class VecOps[A] {
    def contains [A1 >: A](vec: Vector[A], elem: A1) : Boolean = {
        vec.contains(elem)
    }
    def size(vec: Vector[A]): Int = {
        vec.size
    }
    def isEmpty(vec: Vector[A]): Boolean = {
        vec.isEmpty
    }
}

trait DefaultVecT[A] {
    def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        vec.appended(elem)
    }

    def insert[B >: A](vec: Vector[A], i: Int, elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        vec.take(i).appended(elem).appendedAll(vec.drop(i))
    }
}

trait UniqueVecT[A] {
    def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B]
    def insert[B >: A](vec: Vector[A], i: Int, elem: B)(implicit ord: Ordering[B]): Vector[B]
    def assertionU[A] (vec: Vector[A]): Boolean = {
        vec.distinct.size == vec.size
    }
}

trait SortedVecT[A] {
    def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B]
    def insert[B >: A](vec: Vector[A], i: Int, elem: B)(implicit ord: Ordering[B]): Vector[B]
    def assertionS[A] (vec : Vector[A]) (implicit ord: Ordering[A]) : Boolean = vec.isEmpty match {
        case true => true
        case _ => vec.sliding(2).forall { case Vector(x, y) => ord.lteq(x, y) }
    }
}

class DefaultVec[A] extends VecOps[A] with DefaultVecT [A] {
}

class UniqueVec[A] extends DefaultVec[A] with UniqueVecT[A] {
    override def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        if (!contains(vec, elem)) {
            super.appended(vec, elem)
        } else {
            vec
        }
    }

    override def insert[B >: A](vec: Vector[A], i: Int, elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        if (!contains(vec, elem)) {
            super.insert(vec, i, elem)
        } else {
            vec
        }
    }
}

class SortedVec[A] extends DefaultVec[A] with SortedVecT[A] {
    override def appended [B >: A](vec: Vector[A], elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        vec.search(elem) match {
            case Found(x) => {
                super.insert(vec, x, elem)
            }
            case InsertionPoint(x) => {
                super.insert(vec, x, elem)
            }
        }
    }

    override def insert[B >: A](vec: Vector[A], i: Int, elem: B)(implicit ord: Ordering[B]): Vector[B] = {
        if (ord.gteq(vec(i), elem)) {
            if (i - 1 < 0) {
                super.insert(vec, i, elem)
            } else {
                if (ord.lteq(vec(i - 1), elem)) {
                    super.insert(vec, i, elem)
                } else {
                        vec.search(elem) match {
                        case Found(x) => {
                            super.insert(vec, x, elem)
                        }
                        case InsertionPoint(x) => {
                            super.insert(vec, x, elem)
                        }
                    }
                }
            }
        } else {
            vec.search(elem) match {
                case Found(x) => {
                    super.insert(vec, x, elem)
                }
                case InsertionPoint(x) => {
                    super.insert(vec, x, elem)
                }
            }
        }
    }
}
