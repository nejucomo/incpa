#![doc = include_str!("../../README.md")]
#![deny(missing_docs, unsafe_code)]

macro_rules! link_subcrate {
    ($name:ident) => {
        pub mod $name {
            #![doc = include_str!(concat!("../../", stringify!($name), "/Description.md"))]

            //! # Re-export
            //!
            #![doc = concat!("//! This mod is a re-export of all of [incpa_", stringify!($name), "]; see [Related Crates](crate#related_crates) about the crate relationships.")]

            paste::paste! {
                pub use [<incpa_ $name>]::*;
            }
        }
    };
}

link_subcrate!(parser);
link_subcrate!(state);
link_subcrate!(byte);
link_subcrate!(str);
link_subcrate!(tokio);
link_subcrate!(recursive);
