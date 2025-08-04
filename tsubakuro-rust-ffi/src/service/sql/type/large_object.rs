use log::trace;
use std::ffi::CString;
use tsubakuro_rust_core::prelude::TgLargeObjectCache;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiLargeObjectCache {
    cache: TgLargeObjectCache,
    path: Option<CString>,
}

impl TsurugiFfiLargeObjectCache {
    pub(crate) fn new(cache: TgLargeObjectCache) -> TsurugiFfiLargeObjectCache {
        TsurugiFfiLargeObjectCache { cache, path: None }
    }
}

impl std::ops::Deref for TsurugiFfiLargeObjectCache {
    type Target = TgLargeObjectCache;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

pub type TsurugiFfiLargeObjectCacheHandle = *mut TsurugiFfiLargeObjectCache;

/// LargeObjectCache: Get path.
///
/// See [`TgLargeObjectCache::path`].
///
/// # Receiver
/// - `large_object_cache` - large object cache.
///
/// # Returns
/// - `path_out` - path (`null` if not exists).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_large_object_cache_get_path(
    context: TsurugiFfiContextHandle,
    large_object_cache: TsurugiFfiLargeObjectCacheHandle,
    path_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_large_object_cache_get_path()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, large_object_cache={:?}, path_out={:?}",
        context,
        large_object_cache,
        path_out
    );

    ffi_arg_out_initialize!(path_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, large_object_cache);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, path_out);

    let cache = unsafe { &mut *large_object_cache };

    if cache.path.is_none() {
        if let Some(path) = cache.path() {
            let value = path.to_string_lossy().to_string();
            cchar_field_set!(context, cache.path, value);
        }
    }
    let ptr = cstring_to_cchar!(cache.path);
    unsafe {
        *path_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (path={:?})", rc, ptr);
    rc
}

/// LargeObjectCache: Dispose.
///
/// # Receiver
/// - `large_object_cache` - large object cache.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_large_object_cache_dispose(
    large_object_cache: TsurugiFfiLargeObjectCacheHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_large_object_cache_dispose()";
    trace!(
        "{FUNCTION_NAME} start. large_object_cache={:?}",
        large_object_cache
    );

    if large_object_cache.is_null() {
        trace!("{FUNCTION_NAME} end. arg[cache] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(large_object_cache);
    }

    trace!("{FUNCTION_NAME} end");
}
