use std::future::Future;
use std::pin::pin;

use incpa::Parser;
use incpa::byte::{BufferManager, ByteParser};

/// Every [ByteParser] is a [ByteParserExt]
impl<P> ByteParserExt for P where P: ByteParser {}

/// An extension of [ByteParser] with [tokio::io::AsyncRead] support
pub trait ByteParserExt: ByteParser {
    /// Consume and parse all of input from `r` asynchronously
    fn parse_reader_async<R, E>(self, r: R) -> impl Future<Output = Result<Self::Output, E>>
    where
        R: tokio::io::AsyncRead,
        E: From<<Self as Parser<[u8]>>::Error> + From<std::io::Error>,
    {
        self.parse_reader_with_initial_buffer_size_async(r, 1 << 12)
    }

    /// Consume and parse all of input from `r` asynchronously while using an initial buffer size
    fn parse_reader_with_initial_buffer_size_async<R, E>(
        self,
        r: R,
        bufsize: usize,
    ) -> impl Future<Output = Result<Self::Output, E>>
    where
        R: tokio::io::AsyncRead,
        E: From<<Self as Parser<[u8]>>::Error> + From<std::io::Error>,
    {
        use incpa::state::Outcome::{Next, Parsed};
        use tokio::io::AsyncReadExt;

        async move {
            let mut update = Next(self.into_parser());
            let mut bufmgr = BufferManager::with_initial_size(bufsize);
            let mut rpin = pin!(r);

            loop {
                match update {
                    Next(parser) => {
                        let writeslice = bufmgr.get_write_slice();
                        let readcnt = rpin.read(writeslice).await?;
                        update = bufmgr.process_write::<_, E>(parser, readcnt)?;
                    }

                    Parsed(output) => {
                        return Ok(output);
                    }
                }
            }
        }
    }
}
