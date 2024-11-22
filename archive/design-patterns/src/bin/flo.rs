// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/functional/optics.html

use std::{collections::HashMap, str::FromStr};

trait Serde {
    type V;
    fn deserialize(visitor: Self::V) -> Self;
    fn serialize(self) -> Self::V;
}

trait Visitor {
    fn to_json(self) -> String;
    fn from_json(json: String) -> Self;
}

struct Concordance {
    keys: HashMap<String, usize>,
    value_table: Vec<(usize, usize)>,
}

struct ConcordanceSerde {}

impl ConcordanceSerde {
    fn serialize(value: Concordance) -> String {
        todo!()
    }

    // invalid concordances
    fn deserialize(value: String) -> Concordance {
        todo!()
    }
}

// #[derive(Default, Serde)] // the "Serde" derive creates the traint impl block
struct TestStruct {
    a: usize,
    b: String,
}

impl FromStr for TestStruct {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: serde_json::Value = serde_json::from_str(s)?;
        Ok(TestStruct {
            a: v["a"].as_u64().unwrap() as usize,
            b: v["b"].as_str().unwrap().to_string(),
        })
    }
}

impl ToString for TestStruct {
    fn to_string(&self) -> String {
        format!("{{\"a\": {}, \"b\": \"{}\"}}", self.a, self.b)
    }
}

fn main() {
    let a = TestStruct {
        a: 5,
        b: "hello".to_string(),
    };
    // let a_data = a.serialize().to_json();
    println!("Our Test Struct as JSON: {}", a.to_string());
    // let b = TestStruct::deserialize(generated_visitor_for!(TestStruct)::from_json(a_data));
}

// user writes this macro to generate an associated visitor type
// generate_visitor!(TestStruct);
