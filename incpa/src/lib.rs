#![doc = include_str!("../../README.md")]
#![deny(missing_docs, unsafe_code)]

pub mod parser {
    #![doc = include_str!("../../parser/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_parser]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_parser::*;
}

pub mod state {
    #![doc = include_str!("../../state/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_state]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_state::*;
}

pub mod byte {
    #![doc = include_str!("../../byte/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_byte]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_byte::*;
}

pub mod str {
    #![doc = include_str!("../../str/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_str]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_str::*;
}

pub mod tokio {
    #![doc = include_str!("../../tokio/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_tokio]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_tokio::*;
}

pub mod recursive {
    #![doc = include_str!("../../recursive/Description.md")]

    //! # Re-export
    //!
    //! This mod is a re-export of all of [incpa_recursive]; see [Related Crates](crate#related_crates) about the crate relationships.

    pub use incpa_recursive::*;
}
