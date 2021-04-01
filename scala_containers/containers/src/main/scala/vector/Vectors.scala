package vector

import scala.collection.immutable.Vector
import scala.collection.Searching._ 

// Vector is the default implementation of immutable indexed sequences.

/**
 * Since the Vector is a sealed class that I cannot extend directly,
 * Wrapping it up allowing `extends` need to be done
 */
class Vec[+A](v: Vector[A]) (implicit ord: Ordering[A]) {

    def contains [A1 >: A](elem: A1) : Boolean = {
        this.v.contains(elem)
    }

    def size: Int = {
        this.v.size
    }

    def isEmpty: Boolean = {
        this.v.isEmpty
    }

    def appended[B >: A](elem: B)(implicit ord: Ordering[B]): Vec[B] = {
        val vNew = this.v.appended(elem)
        new Vec(vNew)
    }

    def insert[B >: A](i: Int, elem: B)(implicit ord: Ordering[B]): Vec[B] = {
        val vNew = this.v.take(i).appended(elem).appendedAll(this.v.drop(i))
        new Vec(vNew)
    }

    def isSorted : Boolean = this.isEmpty match {
        case true => true
        case _ => this.v.sliding(2).forall { case Vector(x, y) => ord.lteq(x, y) }
    }

    def isUnique : Boolean = {
        this.v.distinct.size == this.size
    }
}

class UniqueVec[+A](v: Vector[A]) (implicit ord: Ordering[A]) extends Vec[A](v) {
    override def appended[B >: A](elem: B)(implicit ord: Ordering[B]): UniqueVec[B] = {
        if (!this.contains(elem)) {
            val vNew = this.v.appended(elem)
            new UniqueVec(vNew)
        } else {
            this
        }
    }

    override def insert[B >: A](i: Int, elem: B)(implicit ord: Ordering[B]): UniqueVec[B] = {
        if (!this.contains(elem)) {
            val vNew = this.v.take(i).appended(elem).appendedAll(this.v.drop(i))
            new UniqueVec(vNew)
        } else {
            this
        }
    }
}

class SortedVec[+A](v: Vector[A]) (implicit ord: Ordering[A]) extends Vec[A](v) {
    override def appended [B >: A](elem: B)(implicit ord: Ordering[B]): SortedVec[B] = {
        this.v.search(elem) match {
            case Found(x) => {
                val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                new SortedVec(vNew)
            }
            case InsertionPoint(x) => {
                val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                new SortedVec(vNew)
            }
        }
    }

    override def insert[B >: A](i: Int, elem: B)(implicit ord: Ordering[B]): SortedVec[B] = {
        if (ord.gteq(this.v(i), elem)) {
            if (i - 1 < 0) {
                val vNew = this.v.prepended(elem)
                new SortedVec(vNew)
            } else {
                if (ord.lteq(this.v(i - 1), elem)) {
                    val vNew = this.v.take(i).appended(elem).appendedAll(this.v.drop(i))
                    new SortedVec(vNew)
                } else {
                        this.v.search(elem) match {
                        case Found(x) => {
                            val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                            new SortedVec(vNew)
                        }
                        case InsertionPoint(x) => {
                            val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                            new SortedVec(vNew)
                        }
                    }
                }
            }
        } else {
            this.v.search(elem) match {
                case Found(x) => {
                    val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                    new SortedVec(vNew)
                }
                case InsertionPoint(x) => {
                    val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                    new SortedVec(vNew)
                }
            }
        }
    } 
}

class UniqueSortedVec[+A](v: Vector[A]) (implicit ord: Ordering[A]) extends Vec[A](v) {
    override def appended [B >: A](elem: B)(implicit ord: Ordering[B]): UniqueSortedVec[B] = {
        this.v.search(elem) match {
            case Found(_) => {
                this
            }
            case InsertionPoint(x) => {
                val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                new UniqueSortedVec(vNew)
            }
        }
    }

    override def insert[B >: A](i: Int, elem: B)(implicit ord: Ordering[B]): UniqueSortedVec[B] = {
        if (ord.gt(this.v(i), elem)) {
            if (i - 1 < 0) {
                val vNew = this.v.prepended(elem)
                new UniqueSortedVec(vNew)
            } else {
                if (ord.lt(this.v(i - 1), elem)) {
                    val vNew = this.v.take(i).appended(elem).appendedAll(this.v.drop(i))
                    new UniqueSortedVec(vNew)
                } else {
                        this.v.search(elem) match {
                        case Found(x) => {
                            this
                        }
                        case InsertionPoint(x) => {
                            val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                            new UniqueSortedVec(vNew)
                        }
                    }
                }
            }
        } else {
            this.v.search(elem) match {
                case Found(x) => {
                    this
                }
                case InsertionPoint(x) => {
                    val vNew = this.v.take(x).appended(elem).appendedAll(this.v.drop(x))
                    new UniqueSortedVec(vNew)
                }
            }
        }
    }
}