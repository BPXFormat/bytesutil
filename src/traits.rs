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

use std::io::Read;
use std::io::Result;

/// Allows to read into a buffer as much as possible.
///
/// *Allows the use BufReader with BPX.*
pub trait ReadFill: Read {
    /// Reads into `buf` as much as possible.
    ///
    /// *Returns the number of bytes that could be read.*
    ///
    /// # Arguments
    ///
    /// * `buf`: the buffer to read into.
    ///
    /// returns: Result<usize, Error>
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) when read has failed.
    fn read_fill(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut bytes = 0;
        let mut len = self.read(buf)?;
        bytes += len;
        while len > 0 && buf.len() - len > 0 {
            len = self.read(&mut buf[len..])?;
            bytes += len;
        }
        Ok(bytes)
    }
}

//Unfortunately it's impossible in rust to allow an extension of the Read trait with custom
// implementations.
impl<T: Read + ?Sized> ReadFill for T {}

/// Allows reading an entire IO stream into a vec.
pub trait ReadToVec: Read {
    /// Loads this stream into memory.
    ///
    /// # Errors
    ///
    /// An [Error](std::io::Error) is returned if the stream could not be loaded.
    fn read_to_vec(&mut self) -> Result<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();
        self.read_to_end(&mut data)?;
        Ok(data)
    }
}
