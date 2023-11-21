// Copyright (c) 2023, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

/// Endian aware write to a byte buffer.
pub trait WriteBytes {
    /// Writes the bytes of self into the given buffer, in little endian order.
    /// 
    /// # Panics
    /// 
    /// Panics if the size of bytes is too small to fit the value of self.
    fn write_bytes_le(&self, bytes: &mut [u8]);

    /// Writes the bytes of self into the given buffer, in big endian order.
    /// 
    /// # Panics
    /// 
    /// Panics if the size of bytes is too small to fit the value of self.
    fn write_bytes_be(&self, bytes: &mut [u8]);
}

/// Endian aware read from a byte buffer.
pub trait ReadBytes {
    /// Reads the bytes of self from the given buffer, in little endian order.
    /// 
    /// # Panics
    /// 
    /// Panics if the size of bytes is too small to store the value of self.
    fn read_bytes_le(bytes: &[u8]) -> Self;

    /// Reads the bytes of self from the given buffer, in big endian order.
    /// 
    /// # Panics
    /// 
    /// Panics if the size of bytes is too small to store the value of self.
    fn read_bytes_be(bytes: &[u8]) -> Self;
}

/// Endian aware write to a [Write](std::io::Write).
#[cfg(feature = "std")]
pub trait WriteTo {
    /// Writes the bytes of self into the given [Write](std::io::Write), in little endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be written.
    fn write_to_le<T: std::io::Write>(&self, dst: T) -> std::io::Result<()>;

    /// Writes the bytes of self into the given [Write](std::io::Write), in big endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be written.
    fn write_to_be<T: std::io::Write>(&self, dst: T) -> std::io::Result<()>;
}

/// Endian aware read from a [Read](std::io::Read).
#[cfg(feature = "std")]
pub trait ReadFrom: Sized {
    /// Reads the bytes of self from the given [Read](std::io::Read), in little endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be read.
    fn read_from_le<T: std::io::Read>(src: T) -> std::io::Result<Self>;

    /// Reads the bytes of self from the given [Read](std::io::Read), in big endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be read.
    fn read_from_be<T: std::io::Read>(src: T) -> std::io::Result<Self>;
}

/// Endian aware write to a [Write](std::io::Write).
#[cfg(feature = "std")]
pub trait WriteExt {
    /// Writes the bytes of val into self, in little endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be written.
    fn write_le<T: WriteTo>(&mut self, val: T) -> std::io::Result<()>;

    /// Writes the bytes of val into self, in big endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be written.
    fn write_be<T: WriteTo>(&mut self, val: T) -> std::io::Result<()>;
}

/// Endian aware read from a [Read](std::io::Read).
#[cfg(feature = "std")]
pub trait ReadExt: Sized {
    /// Reads bytes from self and return an instance of val in little endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be read.
    fn read_le<T: ReadFrom>(&mut self) -> std::io::Result<T>;

    /// Reads bytes from self and return an instance of val in big endian order.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if some bytes could not be read.
    fn read_be<T: ReadFrom>(&mut self) -> std::io::Result<T>;
}

#[cfg(feature = "std")]
impl<W: std::io::Write> WriteExt for W {
    fn write_le<T: WriteTo>(&mut self, val: T) -> std::io::Result<()> {
        val.write_to_le(self)
    }

    fn write_be<T: WriteTo>(&mut self, val: T) -> std::io::Result<()> {
        val.write_to_be(self)
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> ReadExt for R {
    fn read_le<T: ReadFrom>(&mut self) -> std::io::Result<T> {
        T::read_from_le(self)
    }

    fn read_be<T: ReadFrom>(&mut self) -> std::io::Result<T> {
        T::read_from_be(self)
    }
}

macro_rules! impl_bytes {
    ($($t: ty: $size: literal)*) => {
        $(
            impl WriteBytes for $t {
                fn write_bytes_le(&self, bytes: &mut [u8]) {
                    let block = (*self).to_le_bytes();
                    bytes[..$size].copy_from_slice(&block);
                }

                fn write_bytes_be(&self, bytes: &mut [u8]) {
                    let block = self.to_be_bytes();
                    bytes[..$size].copy_from_slice(&block);
                }
            }

            impl ReadBytes for $t {
                fn read_bytes_le(bytes: &[u8]) -> Self {
                    <$t>::from_le_bytes(bytes[..$size].try_into().unwrap())
                }

                fn read_bytes_be(bytes: &[u8]) -> Self {
                    <$t>::from_be_bytes(bytes[..$size].try_into().unwrap())
                }
            }

            #[cfg(feature = "std")]
            impl WriteTo for $t {
                fn write_to_le<T: std::io::Write>(&self, mut dst: T) -> std::io::Result<()> {
                    let block = (*self).to_le_bytes();
                    dst.write_all(&block)?;
                    Ok(())
                }

                fn write_to_be<T: std::io::Write>(&self, mut dst: T) -> std::io::Result<()> {
                    let block = (*self).to_be_bytes();
                    dst.write_all(&block)?;
                    Ok(())
                }
            }

            #[cfg(feature = "std")]
            impl ReadFrom for $t {
                fn read_from_le<T: std::io::Read>(mut src: T) -> std::io::Result<Self> {
                    let mut block: [u8; $size] = [0; $size];
                    src.read_exact(&mut block)?;
                    Ok(<$t>::from_le_bytes(block))
                }

                fn read_from_be<T: std::io::Read>(mut src: T) -> std::io::Result<Self> {
                    let mut block: [u8; $size] = [0; $size];
                    src.read_exact(&mut block)?;
                    Ok(<$t>::from_be_bytes(block))
                }
            }
        )*
    };
}

impl_bytes!(i8: 1 u8: 1 i16: 2 u16: 2 i32: 4 u32: 4 i64: 8 u64: 8 i128: 16 u128: 16 f32: 4 f64: 8);

impl WriteBytes for bool {
    fn write_bytes_le(&self, bytes: &mut [u8]) {
        match self {
            true => (1 as u8).write_bytes_le(bytes),
            false => (0 as u8).write_bytes_le(bytes)
        }
    }

    fn write_bytes_be(&self, bytes: &mut [u8]) {
        match self {
            true => (1 as u8).write_bytes_be(bytes),
            false => (0 as u8).write_bytes_be(bytes)
        }
    }
}

impl ReadBytes for bool {
    fn read_bytes_le(bytes: &[u8]) -> Self {
        match u8::read_bytes_le(bytes) {
            0 => false,
            _ => true
        }
    }

    fn read_bytes_be(bytes: &[u8]) -> Self {
        match u8::read_bytes_be(bytes) {
            0 => false,
            _ => true
        }
    }
}

#[cfg(feature = "std")]
impl WriteTo for bool {
    fn write_to_le<T: std::io::Write>(&self, mut dst: T) -> std::io::Result<()> {
        match self {
            true => dst.write_le(1 as u8),
            false => dst.write_le(0 as u8)
        }
    }

    fn write_to_be<T: std::io::Write>(&self, mut dst: T) -> std::io::Result<()> {
        match self {
            true => dst.write_be(1 as u8),
            false => dst.write_be(0 as u8)
        }
    }
}

#[cfg(feature = "std")]
impl ReadFrom for bool {
    fn read_from_le<T: std::io::Read>(src: T) -> std::io::Result<Self> {
        Ok(match u8::read_from_le(src)? {
            0 => false,
            _ => true
        })
    }

    fn read_from_be<T: std::io::Read>(src: T) -> std::io::Result<Self> {
        Ok(match u8::read_from_be(src)? {
            0 => false,
            _ => true
        })
    }
}
