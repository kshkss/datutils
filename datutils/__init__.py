import os
import msgpack_numpy as msgpack
import gzip
import lzma
import lz4.frame
import zstandard as zstd

__version__ = "0.1.0"

def check_ext(filename):
    ext = os.path.splitext(filename)[-1]
    if ext == ".lz4":
        compress = "lz4"
    elif ext == ".zstd":
        compress = "zstd"
    elif ext == ".gz":
        compress = "gzip"
    elif ext == ".xz":
        compress = "lzma"
    else:
        compress = "none"

    return compress


def save(filename, obj, compress="auto"):
    if compress == "auto":
        compress = check_ext(filename)

    msg = msgpack.packb(obj)
    
    if compress == "none":
        save_raw(filename, msgpack.packb(obj))
    elif compress == "gzip":
        save_gzip(filename, msgpack.packb(obj))
    elif compress == "lzma":
        save_lzma(filename, msgpack.packb(obj))
    elif compress == "lz4":
        save_lz4(filename, msgpack.packb(obj))
    elif compress == "zstd":
        save_zstd(filename, msgpack.packb(obj))
    else:
        raise NotImplemented()

def load(filename):
    compress = check_ext(filename)
    
    if compress == "none":
        msg = load_raw(filename)
    elif compress == "gzip":
        msg = load_gzip(filename)
    elif compress == "lzma":
        msg = load_lzma(filename)
    elif compress == "lz4":
        msg = load_lz4(filename)
    elif compress == "zstd":
        msg = load_zstd(filename)
    else:
        raise NotImplemented()

    return msgpack.unpackb(msg)

def save_raw(filename, msg):
    with open(filename, "wb") as fd:
        fd.write(msg)

def load_raw(filename):
    with open(filename, "rb") as fd:
        return fd.read()

def save_gzip(filename, msg):
    with gzip.open(filename, "wb") as fd:
        fd.write(msg)

def load_gzip(filename):
    with gzip.open(filename, "rb") as fd:
        return fd.read()

def save_lzma(filename, msg):
    with lzma.open(filename, "wb") as fd:
        fd.write(msg)

def load_lzma(filename):
    with lzma.open(filename, "rb") as fd:
        return fd.read()

def save_lz4(filename, msg):
    with lz4.frame.open(filename, "wb") as fd:
        fd.write(msg)

def load_lz4(filename):
    with lz4.frame.open(filename, "rb") as fd:
        return fd.read()

def save_zstd(filename, msg):
    with zstd.open(filename, "wb") as fd:
        fd.write(msg)

def load_zstd(filename):
    with zstd.open(filename, "rb") as fd:
        return fd.read()

