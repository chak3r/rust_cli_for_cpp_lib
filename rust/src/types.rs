use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct A {
    pub first: f64,
    pub second: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ACollection {
    pub data: Vec<A>,
}

#[derive(Serialize, Deserialize)]
pub struct B {
    pub first: f64,
    pub second: f64,
}

#[derive(Serialize, Deserialize)]
pub struct BCollection {
    pub data: Vec<B>,
}
