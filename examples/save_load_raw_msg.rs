use datutils::*;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use ndarray::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Foo {
    bar: u32,
    baz: Array1<f64>,
}

fn main() -> Result<()> {
    let foo = Foo {
        bar: 1,
        baz: Array1::<f64>::linspace(-1., 1., 10),
    };

    save("test.msg", &foo)?;
    let boo: Foo = load("test.msg")?;
    assert_eq!(foo, boo);

    Ok(())
}


