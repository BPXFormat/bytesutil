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

use std::fmt::Arguments;
use std::io::{IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};

/// A tool which combines a [Read]+[Seek] and a [Write]+[Seek] into a [Read]+[Write]+[Seek].
///
/// * All calls to the [Read] interface are forwarded only to the [Read] end.
/// * All calls to the [Write] interface are forwarded only to the [Write] end.
/// * All calls to the [Seek] interface are forwarded to both the [Read] and the [Write] ends.
///
/// All interfaces are optional.
pub struct Combine<R, W> {
    reader: R,
    writer: W
}

impl<R, W> Combine<R, W> {
    /// Creates a new instance of a [Combine] tool.
    ///
    /// # Arguments
    ///
    /// * `read_end`: the [Read] (optionally [Seek]) end.
    /// * `write_end`: the [Write] (optionally [Seek]) end.
    pub fn new(read_end: R, write_end: W) -> Combine<R, W> {
        Self {
            reader: read_end,
            writer: write_end
        }
    }
}

impl<R: Read, W> Read for Combine<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        self.reader.read_vectored(bufs)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.reader.read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        self.reader.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.reader.read_exact(buf)
    }
}

impl<R, W: Write> Write for Combine<R, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self.writer.write_vectored(bufs)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.writer.write_fmt(fmt)
    }
}

impl<R: Seek, W: Seek> Seek for Combine<R, W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)?;
        self.writer.seek(pos)
    }

    fn rewind(&mut self) -> std::io::Result<()> {
        self.reader.rewind()?;
        self.writer.rewind()
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.reader.stream_position()
    }
}
