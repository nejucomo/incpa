use crate::parsing::{Outcome, ParserState};

/// Manage the buffering necessary for driving [ByteFormat](crate::syntax::ByteFormat) in an i/o agnostic manner
#[derive(Debug, Default)]
pub struct BufferManager {
    buffer: Vec<u8>,
    rstart: usize,
}

impl BufferManager {
    /// Construct a [BufferManager] with `cap` byte size initially.
    pub fn with_initial_size(cap: usize) -> Self {
        BufferManager {
            buffer: vec![0; cap],
            rstart: 0,
        }
    }

    /// Get a writable byte slice for inserting new data
    pub fn get_write_slice(&mut self) -> &mut [u8] {
        if self.rstart == self.buffer.len() {
            // The parser is using the entire buffer for storage, so let's grow for new input:
            self.buffer
                .resize(std::cmp::max(self.buffer.len() * 2, 1 << 12), 0);
        }
        &mut self.buffer[self.rstart..]
    }

    /// Process newly inserted data
    ///
    /// # Diagram
    ///
    /// ```text
    ///             rstart-+           +-end
    ///                    | _readcnt_ |
    ///                    v/         \v
    ///        +-----------+-----------+--------+
    /// buffer | prev-kept | new       | uninit |
    ///        +-----------+----+------+--------+
    /// rslice |    consumed    | kept |
    ///        +------+---------+------+--------+
    /// rotate | kept | uninit                  |
    ///        +------+-------------------------+
    /// ```
    pub fn process_write<P, E>(
        &mut self,
        parser: P,
        readcnt: usize,
    ) -> Result<Outcome<P, P::Output>, E>
    where
        P: ParserState<[u8]>,
        E: From<P::Error>,
    {
        use crate::parsing::Update;
        use Outcome::Parsed;

        let end = self.rstart + readcnt;
        let rslice = &self.buffer[..end];

        if readcnt == 0 {
            let output = parser.end_input(rslice)?;
            Ok(Parsed(output))
        } else {
            let Update { consumed, outcome } = parser.feed(rslice)?;

            self.buffer.rotate_left(consumed);
            self.rstart = end - consumed;

            Ok(outcome)
        }
    }
}
