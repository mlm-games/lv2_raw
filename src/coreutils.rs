// Copyright 2017 Michael Oswald
// Copyright 2016 David Robillard <http://drobilla.net>

//! Utility functions for LV2 core features.

use crate::core::LV2Feature;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

/// Return the data for a feature in a features array.
///
/// If the feature is not found, None is returned.
///
/// # Safety
///
/// The features array and URI string must be valid pointers.
pub unsafe fn lv2_features_data(
    features: *const *const LV2Feature,
    curi: *const c_char,
) -> Option<NonNull<c_void>> {
    unsafe {
        if features.is_null() {
            return None;
        }

        let uri = CStr::from_ptr(curi).to_string_lossy();

        let mut i = 0;
        loop {
            let feature = *features.add(i);
            if feature.is_null() {
                break;
            }

            let feature_uri = CStr::from_ptr((*feature).uri).to_string_lossy();
            if feature_uri == uri {
                return NonNull::new((*feature).data);
            }

            i += 1;
        }

        None
    }
}

/// Helper for feature queries.
#[derive(Debug)]
pub struct FeatureHelper {
    /// Feature URI
    pub urid: *const c_char,
    /// Pointer to store feature data
    pub data: *mut *mut c_void,
    /// Whether this feature is required
    pub required: bool,
}

/// Query a features array.
///
/// # Safety
///
/// All pointers must be valid and data pointers must be initialized to NULL.
///
/// # Returns
///
/// NULL on success, otherwise the URI of the missing required feature.
pub unsafe fn lv2_features_query(
    features: *const *const LV2Feature,
    query: &[FeatureHelper],
) -> *const c_char {
    unsafe {
        for it in query {
            let data_ptr = lv2_features_data(features, it.urid);
            *it.data = data_ptr.map_or(std::ptr::null_mut(), |p| p.as_ptr());

            if it.required && (*it.data).is_null() {
                return it.urid;
            }
        }

        std::ptr::null()
    }
}
