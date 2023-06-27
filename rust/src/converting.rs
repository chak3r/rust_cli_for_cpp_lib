use std::slice::from_raw_parts;

use crate::native_wrapper;
use crate::types;

impl From<native_wrapper::A> for types::A {
    fn from(value: native_wrapper::A) -> types::A {
        types::A {
            first: value.first,
            second: value.second,
        }
    }
}

impl Into<native_wrapper::A> for types::A {
    fn into(self) -> native_wrapper::A {
        native_wrapper::A {
            first: self.first,
            second: self.second,
        }
    }
}

impl From<native_wrapper::B> for types::B {
    fn from(value: native_wrapper::B) -> types::B {
        types::B {
            first: value.first,
            second: value.second,
        }
    }
}

impl Into<native_wrapper::B> for types::B {
    fn into(self) -> native_wrapper::B {
        native_wrapper::B {
            first: self.first,
            second: self.second,
        }
    }
}

impl From<native_wrapper::ACollection> for types::ACollection {
    fn from(value: native_wrapper::ACollection) -> types::ACollection {
        let collection = unsafe { from_raw_parts(value.data, value.size as usize) };

        types::ACollection {
            data: collection.iter().map(|it| types::A::from(*it)).collect(),
        }
    }
}

impl Into<native_wrapper::ACollection> for types::ACollection {
    fn into(self) -> native_wrapper::ACollection {
        let native_collection =
            unsafe { native_wrapper::allocACollection(self.data.len().try_into().unwrap()) };

        for i in 0..self.data.len() {
            unsafe {
                let mut element = native_collection.data.offset(i.try_into().unwrap());
                (*element).first = self.data[i].first;
                (*element).second = self.data[i].second;
                println!("A: {{ {} , {} }}", (*element).first, (*element).second);
            }
        }
        return native_collection;
    }
}

impl Into<native_wrapper::BCollection> for types::BCollection {
    fn into(self) -> native_wrapper::BCollection {
        let native_collection =
            unsafe { native_wrapper::allocBCollection(self.data.len().try_into().unwrap()) };

        for i in 0..self.data.len() {
            unsafe {
                let mut element = native_collection.data.offset(i.try_into().unwrap());
                (*element).first = self.data[i].first;
                (*element).second = self.data[i].second;
                println!("A: {{ {} , {} }}", (*element).first, (*element).second);
            }
        }
        return native_collection;
    }
}

impl From<native_wrapper::BCollection> for types::BCollection {
    fn from(value: native_wrapper::BCollection) -> types::BCollection {
        let collection = unsafe { from_raw_parts(value.data, value.size as usize) };

        types::BCollection {
            data: collection.iter().map(|it| types::B::from(*it)).collect(),
        }
    }
}