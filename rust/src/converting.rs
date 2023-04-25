use crate::native_wrapper;
use crate::types;

#[allow(dead_code)]
pub unsafe fn convert_from_a_collection(
    native_collection: &native_wrapper::ACollection,
) -> types::ACollection {
    let mut collection = types::ACollection {
        data: Vec::with_capacity(native_collection.size.try_into().unwrap()),
    };
    for i in 0..native_collection.size {
        let element = native_collection.data.offset(i.try_into().unwrap());
        let new_object = types::A {
            first: (*element).first,
            second: (*element).second,
        };
        collection.data.push(new_object);
    }

    return collection;
}

pub unsafe fn convert_to_a_collection(
    collection: &types::ACollection,
) -> native_wrapper::ACollection {
    let native_collection =
        native_wrapper::allocACollection(collection.data.len().try_into().unwrap());
    for i in 0..collection.data.len() {
        let mut element = native_collection.data.offset(i.try_into().unwrap());
        (*element).first = collection.data[i].first;
        (*element).second = collection.data[i].second;
        println!("A: {{ {} , {} }}", (*element).first, (*element).second);
    }

    return native_collection;
}

#[allow(dead_code)]
pub unsafe fn convert_from_b_collection(
    native_collection: &native_wrapper::BCollection,
) -> types::BCollection {
    let mut collection = types::BCollection {
        data: Vec::with_capacity(native_collection.size.try_into().unwrap()),
    };
    for i in 0..native_collection.size {
        let element = native_collection.data.offset(i.try_into().unwrap());
        let new_object = types::B {
            first: (*element).first,
            second: (*element).second,
        };
        collection.data.push(new_object);
    }

    return collection;
}

pub unsafe fn convert_to_b_collection(
    collection: &types::BCollection,
) -> native_wrapper::BCollection {
    let native_collection =
        native_wrapper::allocBCollection(collection.data.len().try_into().unwrap());
    for i in 0..collection.data.len() {
        let mut element = native_collection.data.offset(i.try_into().unwrap());
        (*element).first = collection.data[i].first;
        (*element).second = collection.data[i].second;
        println!("A: {{ {} , {} }}", (*element).first, (*element).second);
    }

    return native_collection;
}
