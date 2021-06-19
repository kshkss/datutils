# datutils
This library provides simple interface for Rust and Python programs
to save/load data using MessagePack. The result can be compressed by
lz4/zstd/gzip/xz if you would like.

# Usage
```rust
use datutils::*;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use ndarray::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Foo {
    bar: u32,
    baz: Array1::<f64>::linspace(-1., 1., 10),
}

fn main() -> Result<()> {
    let foo = Foo {
        bar: 1,
        baz: Array1::<f64>::linspace(-1., 1., 10),
    };

    save("test.msg.lz4", &foo)?;
    let boo: Foo = load("test.msg.lz4")?;
    assert_eq!(foo, boo);

    Ok(())
}
```

```python
import datutils
import numpy as np

obj = [dict(a="123", b=np.linspace(-1.0, 1.0), c=42)]
datutils.save("test.msg.lz4", obj)
print(datutils.load("test.msg.lz4"))
```
