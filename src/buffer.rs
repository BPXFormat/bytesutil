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

use core::fmt::{Debug, Display};

use crate::{ReadBytes, WriteBytes};

/// A java-like wrapper over a buffer of bytes.
pub struct ByteBuf<T> {
    inner: T
}

impl<T: AsRef<[u8]>> ByteBuf<T> {
    /// Read a little-endian field at the given `pos` offset in bytes.
    pub fn get_le<V: ReadBytes>(&self, pos: usize) -> V {
        V::read_bytes_le(&self.inner.as_ref()[pos..])
    }

    /// Read a big-endian field at the given `pos` offset in bytes.
    pub fn get_be<V: ReadBytes>(&self, pos: usize) -> V {
        V::read_bytes_be(&self.inner.as_ref()[pos..])
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for ByteBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl<T: AsMut<[u8]>> ByteBuf<T> {
    /// Write the given little-endian `value` field at the given `pos` offset in bytes.
    pub fn set_le<V: WriteBytes>(&mut self, pos: usize, value: V) -> &mut Self {
        value.write_bytes_le(&mut self.inner.as_mut()[pos..]);
        self
    }

    /// Write the given big-endian `value` field at the given `pos` offset in bytes.
    pub fn set_be<V: WriteBytes>(&mut self, pos: usize, value: V) -> &mut Self {
        value.write_bytes_be(&mut self.inner.as_mut()[pos..]);
        self
    }
}

impl<T: AsMut<[u8]>> AsMut<[u8]> for ByteBuf<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl<T> ByteBuf<T> {
    /// Allocates a new ByteBuf by wrapping the given bytes-like object.
    /// 
    /// A bytes like object is an object which at least implements [AsRef](std::convert::AsRef) for `[u8]` type.
    /// To support read-write operations, the wrapped object shall implement [AsMut](std::convert::AsMut) for `[u8]` type.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns the underlying wrapped bytes-like object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> From<T> for ByteBuf<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Copy> From<&T> for ByteBuf<T> {
    fn from(value: &T) -> Self {
        Self::new(*value)
    }
}

impl<T: Copy> From<&mut T> for ByteBuf<T> {
    fn from(value: &mut T) -> Self {
        Self::new(*value)
    }
}

impl<T: Copy> From<&ByteBuf<T>> for ByteBuf<T> {
    fn from(value: &ByteBuf<T>) -> Self {
        *value
    }
}

impl<T: Copy> From<&mut ByteBuf<T>> for ByteBuf<T> {
    fn from(value: &mut ByteBuf<T>) -> Self {
        *value
    }
}

impl<T: Default> Default for ByteBuf<T> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<T: Clone> Clone for ByteBuf<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T: Copy> Copy for ByteBuf<T> {}

impl<T: PartialEq> PartialEq for ByteBuf<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for ByteBuf<T> {}

impl<T: Debug> Debug for ByteBuf<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Display> Display for ByteBuf<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

/// A shortcut to create a stack allocated fixed size [ByteBuf](ByteBuf)
pub type StaticByteBuf<const N: usize> = ByteBuf<[u8; N]>;

#[cfg(test)]
mod tests {
    use crate::{StaticByteBuf, ByteBuf};

    fn test_function<'a, I: Into<ByteBuf<[u8; 16]>>>(_: I) {
    }

    #[test]
    fn basic() {
        let mut buffer = StaticByteBuf::<16>::default();
        buffer.set_le(0, 42).set_be(8, 42.42);
        assert!(buffer.get_le::<i32>(0) == 42);
        assert!(buffer.get_be::<f64>(8) == 42.42);
        test_function(buffer.set_le(0, 12));
        test_function([0; 16]);
    }

    #[test]
    fn vec() {
        let mut buffer = ByteBuf::new(vec![0 as u8, 0 as u8, 0 as u8, 0 as u8]);
        buffer.set_le(0, 42);
        assert!(buffer.get_le::<i32>(0) == 42);
        let v = buffer.into_inner();
        assert!(v[0] == 42);
    }

    #[test]
    fn borrowed() {
        let mut inner = vec![0 as u8, 0 as u8, 0 as u8, 0 as u8];
        let mut buffer = ByteBuf::new(&mut *inner);
        buffer.set_be(0, 42);
        assert!(buffer.get_be::<i32>(0) == 42);
        assert!(inner[3] == 42);
    }
}
