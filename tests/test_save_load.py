import unittest
import numpy as np
import datutils

class TestSaveLoad(unittest.TestCase):
    def obj(self):
        return [dict(a="123", b=np.linspace(-1.0, 1.0), c=42)]

    def check(self, tgt):
        src = self.obj()
        for src, tgt in zip(src, tgt):
            for k1, k2 in zip(src.keys(), tgt.keys()):
                if k1 != k2:
                    return False
                if not np.array_equal(src[k1], tgt[k2]):
                    return False
        return True

    def test_raw(self):
        obj = self.obj()
        datutils.save("test.msg", obj)
        self.assertTrue(self.check(datutils.load("test.msg")))

    def test_lz4(self):
        obj = self.obj()
        datutils.save("test.msg.lz4", obj)
        self.assertTrue(self.check(datutils.load("test.msg.lz4")))

    def test_zstd(self):
        obj = self.obj()
        datutils.save("test.msg.zstd", obj)
        self.assertTrue(self.check(datutils.load("test.msg.zstd")))

    def test_gz(self):
        obj = self.obj()
        datutils.save("test.msg.gz", obj)
        self.assertTrue(self.check(datutils.load("test.msg.gz")))

    def test_xz(self):
        obj = self.obj()
        datutils.save("test.msg.xz", obj)
        self.assertTrue(self.check(datutils.load("test.msg.xz")))

if __name__ == "__main__":
    unittest.main()
