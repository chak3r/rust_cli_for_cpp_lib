use crate::converting;
use crate::native_wrapper;
use crate::types;

pub unsafe fn sum(a_collection: &types::ACollection, b_collection: &types::BCollection) -> f64 {
    let native_a_collection = converting::convert_to_a_collection(a_collection);
    let native_b_collection = converting::convert_to_b_collection(b_collection);

    let result = native_wrapper::sum(native_a_collection, native_b_collection);

    native_wrapper::freeACollection(native_a_collection);
    native_wrapper::freeBCollection(native_b_collection);

    println!("result: {{ {} }}", result);
    return result;
}
