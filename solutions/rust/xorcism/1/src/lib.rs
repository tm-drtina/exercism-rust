/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: std::iter::Cycle<std::slice::Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|d| *d ^= self.key.next().unwrap());
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'data, 'b, 'iter, Data, Item>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + 'iter + use<'iter, 'b, 'a, Data, Item>
    where
        Data: IntoIterator<Item = Item>,
        Item: std::borrow::Borrow<u8>,
        <Data as IntoIterator>::IntoIter: 'data,
        'a: 'iter,
        'b: 'iter,
        'data: 'iter,
    {
        data.into_iter()
            .map(|d| *d.borrow() ^ self.key.next().unwrap())
    }

    pub fn reader<'read, 'res, R>(self, reader: R) -> impl std::io::Read + 'res
    where
        R: std::io::Read + 'read,
        'read: 'res,
        'a: 'res,
    {
        XorcismReader {
            xorcism: self,
            reader,
        }
    }

    pub fn writer<'write, 'res, W>(self, writer: W) -> impl std::io::Write + 'res
    where
        W: std::io::Write + 'write,
        'write: 'res,
        'a: 'res,
    {
        XorcismWriter {
            xorcism: self,
            writer,
        }
    }
}

struct XorcismReader<'a, R: std::io::Read> {
    xorcism: Xorcism<'a>,
    reader: R,
}
impl<'a, R: std::io::Read> std::io::Read for XorcismReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..len]);
        Ok(len)
    }
}

struct XorcismWriter<'a, W: std::io::Write> {
    xorcism: Xorcism<'a>,
    writer: W,
}
impl<'a, W: std::io::Write> std::io::Write for XorcismWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // TODO: avoid alloc
        self.writer.write(&self.xorcism.munge(buf).collect::<Vec<_>>())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
