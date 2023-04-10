#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
fn main() {
    println!("Hello, world!");
    let arr1 = [1.0, 2.0, 3.0, 4.0];
    let arr2 = [5.0, 6.0, 7.0, 8.0];
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());

    result.extend_from_slice(&arr1);
    result.extend_from_slice(&arr2);

    let nativeSum: f64 = result.iter().sum();
    println!("native sum: {}", nativeSum);

    unsafe {
        let size = arr1.len() / 2;
        let a = allocACollection(size.try_into().unwrap());
        let b = allocBCollection(size.try_into().unwrap());

        for n in 0..size {
            let mut element = a.data.offset(n.try_into().unwrap());
            (*element).first = arr1[n * 2];
            (*element).second = arr1[n * 2 + 1];
            println!("A: {{ {} , {} }}", (*element).first, (*element).second);
        }

        for n in 0..size {
            let mut element = b.data.offset(n.try_into().unwrap());
            (*element).first = arr2[n * 2];
            (*element).second = arr2[n * 2 + 1];
            println!("B: {{ {} , {} }}", (*element).first, (*element).second);
        }

        let sumOfAAndB = sum(a, b);
        println!("sum: {}", sumOfAAndB);
    }
}
