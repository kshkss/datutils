use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn save<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    match filepath.as_ref().extension() {
        Some(ext) if ext == "lz4" => save_lz4(filepath, obj)?,
        Some(ext) if ext == "zstd" => save_zstd(filepath, obj)?,
        Some(ext) if ext == "gz" => save_gz(filepath, obj)?,
        Some(ext) if ext == "xz" => save_xz(filepath, obj)?,
        Some(_) => save_raw(filepath, obj)?,
        None => save_raw(filepath, obj)?,
    };
    Ok(())
}

pub fn load<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let obj = match filepath.as_ref().extension() {
        Some(ext) if ext == "lz4" => load_lz4(filepath)?,
        Some(ext) if ext == "zstd" => load_zstd(filepath)?,
        Some(ext) if ext == "gz" => load_gz(filepath)?,
        Some(ext) if ext == "xz" => load_xz(filepath)?,
        Some(_) => load_raw(filepath)?,
        None => load_raw(filepath)?,
    };
    Ok(obj)
}

fn save_raw<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let msg = rmp_serde::to_vec_named(obj)?;
    let enc = File::create(filepath)?;
    let mut enc = BufWriter::new(enc);
    enc.write_all(msg.as_ref())?;
    Ok(())
}

fn load_raw<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let dec = File::open(filepath)?;
    let dec = BufReader::new(dec);
    let obj = rmp_serde::from_read(dec)?;
    Ok(obj)
}

fn save_lz4<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let msg = rmp_serde::to_vec_named(obj)?;
    let enc = File::create(filepath)?;
    let enc = BufWriter::new(enc);
    let mut enc = lz4::EncoderBuilder::new().build(enc)?;
    enc.write_all(msg.as_ref())?;
    enc.finish().1?;
    Ok(())
}

fn load_lz4<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let dec = File::open(filepath)?;
    let dec = BufReader::new(dec);
    let dec = lz4::Decoder::new(dec)?;
    let obj = rmp_serde::from_read(dec)?;
    Ok(obj)
}

fn save_zstd<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let msg = rmp_serde::to_vec_named(obj)?;
    let fd = File::create(filepath)?;
    let enc = BufWriter::new(fd);
    let mut enc = zstd::Encoder::new(enc, 0)?;
    enc.write_all(msg.as_ref())?;
    enc.finish()?;
    Ok(())
}

fn load_zstd<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let fd = File::open(filepath)?;
    let dec = BufReader::new(fd);
    let dec = zstd::Decoder::new(dec)?;
    let obj = rmp_serde::from_read(dec)?;
    Ok(obj)
}

fn save_gz<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let msg = rmp_serde::to_vec_named(obj)?;
    let fd = File::create(filepath)?;
    let enc = BufWriter::new(fd);
    let mut enc = flate2::write::GzEncoder::new(enc, Default::default());
    enc.write_all(msg.as_ref())?;
    enc.finish()?;
    Ok(())
}

fn load_gz<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let fd = File::open(filepath)?;
    let dec = BufReader::new(fd);
    let dec = flate2::bufread::GzDecoder::new(dec);
    let obj = rmp_serde::from_read(dec)?;
    Ok(obj)
}

fn save_xz<P, T>(filepath: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let msg = rmp_serde::to_vec_named(obj)?;
    let fd = File::create(filepath)?;
    let enc = BufWriter::new(fd);
    let mut enc = xz::write::XzEncoder::new(enc, 6);
    enc.write_all(msg.as_ref())?;
    enc.try_finish()?;
    Ok(())
}

fn load_xz<P, T>(filepath: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let fd = File::open(filepath)?;
    let dec = BufReader::new(fd);
    let dec = xz::read::XzDecoder::new(dec);
    let obj = rmp_serde::from_read(dec)?;
    Ok(obj)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Foo {
        bar: u32,
        baz: Array1<f64>,
    }

    #[test]
    fn it_works() {
        let foo = Foo {
            bar: 1,
            baz: Array1::<f64>::linspace(-1., 1., 10),
        };

        save("test.msg", &foo).unwrap();
        let boo: Foo = load("test.msg").unwrap();
        assert_eq!(foo, boo);
    }

    #[test]
    fn it_works_with_lz4() {
        let foo = Foo {
            bar: 1,
            baz: Array1::<f64>::linspace(-1., 1., 10),
        };

        save("test.msg.lz4", &foo).unwrap();
        let boo: Foo = load("test.msg.lz4").unwrap();
        assert_eq!(foo, boo);
    }

    #[test]
    fn it_works_with_zstd() {
        let foo = Foo {
            bar: 1,
            baz: Array1::<f64>::linspace(-1., 1., 10),
        };

        save("test.msg.zstd", &foo).unwrap();
        let boo: Foo = load("test.msg.zstd").unwrap();
        assert_eq!(foo, boo);
    }

    #[test]
    fn it_works_with_gzip() {
        let foo = Foo {
            bar: 1,
            baz: Array1::<f64>::linspace(-1., 1., 10),
        };

        save("test.msg.gz", &foo).unwrap();
        let boo: Foo = load("test.msg.gz").unwrap();
        assert_eq!(foo, boo);
    }

    #[test]
    fn it_works_with_xz() {
        let foo = Foo {
            bar: 1,
            baz: Array1::<f64>::linspace(-1., 1., 10),
        };

        save("test.msg.xz", &foo).unwrap();
        let boo: Foo = load("test.msg.xz").unwrap();
        assert_eq!(foo, boo);
    }
}
