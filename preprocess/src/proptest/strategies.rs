use proptest::strategy::*;
use proptest::prelude::*;
use std::ops::Range;

use crate::library::eager_unique_vector::{EagerUniqueVec};
use crate::library::eager_sorted_vector::{EagerSortedVec};
use crate::library::lazy_unique_vector::{LazyUniqueVec};
use proptest::collection::vec;
use proptest::prelude::*;
use im::conslist::{ConsList};


pub fn eager_unique_vec<T: Strategy + 'static>(element: T, size: Range<usize>) -> impl Strategy<Value = EagerUniqueVec<T::Value>>
where <T as Strategy>::Value: PartialEq 
{
    vec(element, size.clone()).prop_map(EagerUniqueVec::from_vec)
}


pub fn lazy_unique_vec<T: Strategy + 'static>(element: T, size: Range<usize>) -> impl Strategy<Value = LazyUniqueVec<T::Value>>
where <T as Strategy>::Value: Ord
{
    vec(element, size.clone()).prop_map(LazyUniqueVec::from_vec)
}

pub fn eager_sorted_vec<T: Strategy + 'static>(element: T, size: Range<usize>) -> impl Strategy<Value = EagerSortedVec<T::Value>>
where <T as Strategy>::Value: Ord 
{
    vec(element, size.clone()).prop_map(EagerSortedVec::from_vec)
}