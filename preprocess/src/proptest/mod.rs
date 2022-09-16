pub mod strategies;

use im::conslist::{ConsList};
use im::conslist;
use std::sync::Arc;

pub fn contains<T: PartialEq>(list: &ConsList<T>, elem: &T) -> bool {
    list.iter().find(|x| x.as_ref() == elem).is_some()
}

pub fn clear<T>(list: &ConsList<T>) -> ConsList<T> {
    ConsList::<T>::new()
}

pub fn remove<T: PartialEq+Clone>(list: &ConsList<T>, a: T) -> (ConsList<T>, Option<T>) {
    if contains(list, &a) {
        let mut result = ConsList::<T>::new();
        let mut found = false;
        for i in list.iter() {
            if i.as_ref() == &a && !found {
                found = true;
                continue;
            } else {
                result = result.append(conslist![i.clone()]);
            }
        }
        (result, Some(a))
    } else {
        (list.clone(), None)
    }
}

pub fn first<T>(list: &ConsList<T>) -> Option<&T> {
    list.head().map(|x| unsafe{&*Arc::into_raw(x)})
}

pub fn last<T>(list: &ConsList<T>) -> Option<&T> {
    list.reverse().head().map(|x| unsafe{&*Arc::into_raw(x)})
}

pub fn nth<T>(list: &ConsList<T>, n: usize) -> Option<&T> {
    list.iter().nth(n).map(|x| unsafe{&*Arc::into_raw(x)})
}

pub fn push<T>(list: &ConsList<T>, a: T) -> ConsList<T> {
    list.append(conslist![a])
}

pub fn pop<T>(list: &ConsList<T>) -> (ConsList<T>, Option<Arc<T>>) {
    if list.is_empty() {
        (ConsList::<T>::new(), None)
    } else {
        let (elem, result) = list.reverse().uncons().unwrap();
        (result.reverse(), Some(elem))
    }
}

pub fn unique<T: PartialEq>(list: &ConsList<T>) -> ConsList<T> {
    let mut result = ConsList::<T>::new();
    for i in list.iter() {
        if contains(&result, &i) {
            continue;
        } else {
            result = result.append(conslist![i.clone()]);
        }
    }
    result
}