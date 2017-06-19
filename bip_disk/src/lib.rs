extern crate bip_metainfo;
extern crate bip_util;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate log;
extern crate tokio_core;

mod disk;
mod memory;

/// Both `Block` and `Torrent` error types.
pub mod error;

pub use disk::{IDiskMessage, ODiskMessage};
pub use disk::fs::FileSystem;
pub use disk::builder::DiskManagerBuilder;
pub use disk::manager::{DiskManager};

pub use memory::block::{Block, BlockMetadata};

/// Built in objects implementing `FileSystem`.
pub mod fs {
    pub use disk::fs::native::{NativeFile, NativeFileSystem};
}

pub use bip_util::bt::InfoHash;