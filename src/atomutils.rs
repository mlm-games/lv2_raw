// Copyright 2017 Michael Oswald
// Copyright 2012-2016 David Robillard <http://drobilla.net>

//! Utility functions for working with LV2 Atoms.

use crate::atom::*;
use libc::{memcmp, memcpy};
use std::mem::size_of;
use std::os::raw::c_void;

/// Pad a size to 64 bits.
#[inline]
#[must_use]
pub const fn lv2_atom_pad_size(size: u32) -> u32 {
    (size + 7) & (!7)
}

/// Return the total size of `atom`, including the header.
#[inline]
#[must_use]
pub fn lv2_atom_total_size(atom: &LV2Atom) -> u32 {
    size_of::<LV2Atom>() as u32 + atom.size
}

/// Return true iff `atom` is null.
///
/// # Safety
///
/// The caller must ensure that `atom` is either null or points to a valid `LV2Atom`.
#[inline]
pub unsafe fn lv2_atom_is_null(atom: *const LV2Atom) -> bool { unsafe {
    atom.is_null() || ((*atom).type_ == 0 && (*atom).size == 0)
}}

/// Return true iff `a` is equal to `b`.
///
/// # Safety
///
/// Both atoms must be valid pointers to properly initialized LV2Atom structures.
pub unsafe fn lv2_atom_equals(a: *const LV2Atom, b: *const LV2Atom) -> bool { unsafe {
    (a == b)
        || (((*a).type_ == (*b).type_)
            && ((*a).size == (*b).size)
            && (memcmp(
                a.add(1) as *const c_void,
                b.add(1) as *const c_void,
                (*a).size as usize,
            ) == 0))
}}

/// Get an iterator pointing to the first event in a Sequence body.
///
/// # Safety
///
/// The body must point to a valid LV2AtomSequenceBody.
#[inline]
pub unsafe fn lv2_atom_sequence_begin(body: *const LV2AtomSequenceBody) -> *mut LV2AtomEvent { unsafe {
    body.add(1) as *mut LV2AtomEvent
}}

/// Get an iterator pointing to the end of a Sequence body.
///
/// # Safety
///
/// The body must point to a valid LV2AtomSequenceBody with the given size.
#[inline]
pub unsafe fn lv2_atom_sequence_end(
    body: *const LV2AtomSequenceBody,
    size: u32,
) -> *const LV2AtomEvent { unsafe {
    (body as *const u8).add(lv2_atom_pad_size(size) as usize) as *const LV2AtomEvent
}}

/// Return true iff `i` has reached the end of `body`.
///
/// # Safety
///
/// All pointers must be valid and properly aligned.
#[inline]
pub unsafe fn lv2_atom_sequence_is_end(
    body: *const LV2AtomSequenceBody,
    size: u32,
    i: *const LV2AtomEvent,
) -> bool { unsafe {
    i as *const u8 >= (body as *const u8).add(size as usize)
}}

/// Return an iterator to the element following `i`.
///
/// # Safety
///
/// The iterator must point to a valid LV2AtomEvent within a sequence.
#[inline]
pub unsafe fn lv2_atom_sequence_next(i: *const LV2AtomEvent) -> *mut LV2AtomEvent { unsafe {
    let off = size_of::<LV2AtomEvent>() + lv2_atom_pad_size((*i).body.size) as usize;
    (i as *const u8).add(off) as *mut LV2AtomEvent
}}

/// Clear all events from `sequence`.
///
/// This simply resets the size field, the other fields are left untouched.
///
/// # Safety
///
/// The sequence must be a valid pointer to an LV2AtomSequence.
#[inline]
pub unsafe fn lv2_atom_sequence_clear(seq: *mut LV2AtomSequence) { unsafe {
    (*seq).atom.size = size_of::<LV2AtomSequenceBody>() as u32;
}}

/// Append an event at the end of `sequence`.
///
/// # Safety
///
/// All pointers must be valid and the sequence must have sufficient capacity.
///
/// # Parameters
///
/// * `seq` - Sequence to append to.
/// * `capacity` - Total capacity of the sequence atom.
/// * `event` - Event to write.
///
/// # Returns
///
/// A pointer to the newly written event in `seq`, or NULL on failure.
pub unsafe fn lv2_atom_sequence_append_event(
    seq: *mut LV2AtomSequence,
    capacity: u32,
    event: *const LV2AtomEvent,
) -> *const LV2AtomEvent { unsafe {
    let total_size = size_of::<LV2AtomEvent>() as u32 + (*event).body.size;

    if (capacity - (*seq).atom.size) < total_size {
        return std::ptr::null();
    }

    let e = lv2_atom_sequence_end(&(*seq).body, (*seq).atom.size);
    memcpy(
        e as *mut c_void,
        event as *const c_void,
        total_size as usize,
    );

    (*seq).atom.size += lv2_atom_pad_size(total_size);
    e
}}

/// Return a pointer to the first property in `body`.
///
/// # Safety
///
/// The body must point to a valid LV2AtomObjectBody.
#[inline]
pub unsafe fn lv2_atom_object_begin(body: *const LV2AtomObjectBody) -> *mut LV2AtomPropertyBody { unsafe {
    body.add(1) as *mut LV2AtomPropertyBody
}}

/// Return true iff `i` has reached the end of `obj`.
///
/// # Safety
///
/// All pointers must be valid and properly aligned.
#[inline]
pub unsafe fn lv2_atom_object_is_end(
    body: *const LV2AtomObjectBody,
    size: u32,
    i: *const LV2AtomPropertyBody,
) -> bool { unsafe {
    i as *const u8 >= (body as *const u8).add(size as usize)
}}

/// Return an iterator to the property following `i`.
///
/// # Safety
///
/// The iterator must point to a valid property within an object.
#[inline]
pub unsafe fn lv2_atom_object_next(i: *const LV2AtomPropertyBody) -> *mut LV2AtomPropertyBody { unsafe {
    let value = (i as *const u8).add(2 * size_of::<u32>()) as *const LV2Atom;
    let offset = lv2_atom_pad_size(size_of::<LV2AtomPropertyBody>() as u32 + (*value).size);
    (i as *mut u8).add(offset as usize) as *mut LV2AtomPropertyBody
}}

/// A single entry in an Object query.
#[derive(Debug)]
pub struct LV2AtomObjectQuery {
    /// Key to query (input set by user)
    pub key: u32,
    /// Found value (output set by query function)
    pub value: *mut *mut LV2Atom,
}

/// Get an object's values for various keys.
///
/// # Safety
///
/// The object must be valid and all value pointers in the query must be initialized to NULL.
pub unsafe fn lv2_atom_object_query(
    obj: *mut LV2AtomObject,
    query: *mut LV2AtomObjectQuery,
) -> i32 { unsafe {
    let object = &mut *obj;

    let mut n_queries = 0;
    let mut matches = 0;

    let mut q = query;
    while (*q).key != 0 {
        n_queries += 1;
        q = q.add(1);
    }

    object.foreach(|prop: *mut LV2AtomPropertyBody| -> bool {
        let mut q = query;
        while (*q).key != 0 {
            if (*q).key == (*prop).key && !(*q).value.is_null() {
                let val = &mut (*prop).value;
                *(*q).value = val;

                matches += 1;
                if matches == n_queries {
                    return true;
                }
                break;
            }
            q = q.add(1);
        }
        false
    });

    matches
}}

/// Helper for object property queries.
#[derive(Debug)]
pub struct ObjectHelper {
    /// Property key
    pub key: u32,
    /// Pointer to atom value
    pub atom: *mut *mut LV2Atom,
}

/// Variable argument version of lv2_atom_object_query().
///
/// # Safety
///
/// The object must be valid and all atom pointers must be initialized to NULL.
pub unsafe fn lv2_atom_object_get(body: *mut LV2AtomObject, query: &[ObjectHelper]) -> i32 { unsafe {
    let mut matches = 0;
    let mut n_queries = 0;

    for it in query {
        if it.atom.is_null() {
            return -1;
        }
        n_queries += 1;
    }

    (*body).foreach(|prop: *mut LV2AtomPropertyBody| -> bool {
        for it in query {
            let qkey = it.key;

            if qkey == (*prop).key && (*it.atom).is_null() {
                *it.atom = &mut (*prop).value;
                matches += 1;
                if matches == n_queries {
                    return matches > 0;
                }
                break;
            }
        }
        true
    });

    matches
}}

impl LV2AtomSequenceBody {
    /// Iterate over events in the sequence body.
    ///
    /// # Safety
    ///
    /// The sequence body must have a valid memory layout.
    pub unsafe fn foreach<F>(&mut self, size: u32, mut closure: F)
    where
        F: FnMut(*const LV2AtomEvent),
    { unsafe {
        let mut it = lv2_atom_sequence_begin(self);
        while !lv2_atom_sequence_is_end(self, size, it) {
            closure(it);
            it = lv2_atom_sequence_next(it);
        }
    }}
}

/// An iterator for atom sequences.
///
/// This was written by a beginner. Note that
///
/// - The iterator may be implemented incorrectly.
/// - We are not sure whether it is actually advisable to use it in
///   functions with hard real-time requirements.
/// - The `struct` `LV2AtomSequenceIterator` is, by itself, probably not
///   useful. The only reason why its a public struct is that the code
///   doesn't compile otherwise.
///
pub struct LV2AtomSequenceIterator<'a> {
    /// The sequence being iterated
    pub seq: &'a LV2AtomSequence,
    /// Current position
    pub current: &'a LV2AtomEvent,
}

impl<'a> Iterator for LV2AtomSequenceIterator<'a> {
    type Item = &'a LV2AtomEvent;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let body = &self.seq.body;
            let size = self.seq.atom.size;
            if !lv2_atom_sequence_is_end(body, size, self.current) {
                let out = self.current;
                self.current = &*lv2_atom_sequence_next(self.current);
                Some(out)
            } else {
                None
            }
        }
    }
}

// perhaps wrong. TODO: understand this: http://stackoverflow.com/questions/41448232/issues-constraining-implementation-lifetimes-on-type-without-lifetime-parameter
impl<'a> IntoIterator for &'a LV2AtomSequence {
    type Item = &'a LV2AtomEvent;
    type IntoIter = LV2AtomSequenceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            LV2AtomSequenceIterator {
                seq: self,
                current: &*lv2_atom_sequence_begin(&self.body as *const LV2AtomSequenceBody),
            }
        }
    }
}
