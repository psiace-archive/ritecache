#[macro_use]
extern crate log;
#[cfg(feature = "heapsize")]
extern crate heapsize_;

mod cache;
#[allow(dead_code)]
mod disk_cache;
mod meter;

pub use cache::lru::LruCache;
pub use cache::Cache;
pub use disk_cache::result::Error as DiskCacheError;
pub use disk_cache::result::Result as DiskCacheResult;
pub use disk_cache::DiskCache;
pub use disk_cache::LruDiskCache;
pub use meter::count_meter::Count;
pub use meter::count_meter::CountableMeter;
pub use meter::file_meter::FileSize;
#[cfg(feature = "heapsize")]
pub use meter::heap_meter::HeapSize;
pub use meter::Meter;
