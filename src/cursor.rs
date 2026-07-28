use crate::error::{OrbitError, Result};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn starts_with(&self, magic: &[u8]) -> bool {
        self.data
            .get(self.pos..)
            .is_some_and(|tail| tail.starts_with(magic))
    }

    pub fn consume_magic(&mut self, magic: &[u8]) -> Result<()> {
        if self.read_bytes(magic.len())? == magic {
            Ok(())
        } else {
            Err(OrbitError::BadMagic)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.remaining() < 1 {
            return Err(OrbitError::ShortRead {
                needed: 1,
                remaining: self.remaining(),
            });
        }
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.read_array::<2>()?))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(i16::from_le_bytes(self.read_array::<2>()?))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_array::<4>()?))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.read_array::<4>()?))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_array::<8>()?))
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.remaining() < len {
            return Err(OrbitError::ShortRead {
                needed: len,
                remaining: self.remaining(),
            });
        }
        let start = self.pos;
        self.pos += len;
        Ok(&self.data[start..start + len])
    }

    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let bytes = self.read_bytes(N)?;
        let mut out = [0_u8; N];
        out.copy_from_slice(bytes);
        Ok(out)
    }

    pub fn tail(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }
}
