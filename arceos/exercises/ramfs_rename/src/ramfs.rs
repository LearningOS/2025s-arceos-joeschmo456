extern crate alloc;

use alloc::sync::Arc;
use axfs_ramfs::{RamFileSystem, DirNode};
use axfs_vfs::{VfsOps, VfsNodeOps, VfsError};
use std::os::arceos::api::fs::{AxDisk, MyFileSystemIf};

struct MyFileSystemIfImpl;

#[crate_interface::impl_interface]
impl MyFileSystemIf for MyFileSystemIfImpl {
    fn new_myfs(_disk: AxDisk) -> Arc<dyn VfsOps> {
        Arc::new(RamFileSystem::new())
    }
}
