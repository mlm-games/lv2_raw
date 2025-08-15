// Copyright 2017 Stefan Riha, Michael Oswald
// Copyright 2008-2016 David Robillard <http://drobilla.net>

//! LV2 Atom data structures and utilities.
//!
//! Atoms are generic containers for data that can be used to communicate
//! between plugins and hosts.

use crate::atomutils::*;
use std::mem::transmute;

/// LV2 Atom URI as bytes (null-terminated)
pub const LV2_ATOM_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom\0";
/// LV2 Atom URI as string
pub const LV2_ATOM_URI_STR: &str = "http://lv2plug.in/ns/ext/atom";

/// LV2 Atom prefix as bytes (null-terminated)
pub const LV2_ATOM_PREFIX: &[u8] = b"http://lv2plug.in/ns/ext/atom#\0";
/// LV2 Atom prefix as string
pub const LV2_ATOM_PREFIX_STR: &str = "http://lv2plug.in/ns/ext/atom#";

pub const LV2_ATOM__ATOM: &[u8] = b"http://lv2plug.in/ns/ext/atom#Atom\0";
pub const LV2_ATOM__ATOMPORT: &[u8] = b"http://lv2plug.in/ns/ext/atom#AtomPort\0";
pub const LV2_ATOM__BLANK: &[u8] = b"http://lv2plug.in/ns/ext/atom#Blank\0";
pub const LV2_ATOM__BOOL: &[u8] = b"http://lv2plug.in/ns/ext/atom#Bool\0";
pub const LV2_ATOM__CHUNK: &[u8] = b"http://lv2plug.in/ns/ext/atom#Chunk\0";
pub const LV2_ATOM__DOUBLE: &[u8] = b"http://lv2plug.in/ns/ext/atom#Double\0";
pub const LV2_ATOM__EVENT: &[u8] = b"http://lv2plug.in/ns/ext/atom#Event\0";
pub const LV2_ATOM__FLOAT: &[u8] = b"http://lv2plug.in/ns/ext/atom#Float\0";
pub const LV2_ATOM__INT: &[u8] = b"http://lv2plug.in/ns/ext/atom#Int\0";
pub const LV2_ATOM__LITERAL: &[u8] = b"http://lv2plug.in/ns/ext/atom#Literal\0";
pub const LV2_ATOM__LONG: &[u8] = b"http://lv2plug.in/ns/ext/atom#Long\0";
pub const LV2_ATOM__NUMBER: &[u8] = b"http://lv2plug.in/ns/ext/atom#Number\0";
pub const LV2_ATOM__OBJECT: &[u8] = b"http://lv2plug.in/ns/ext/atom#Object\0";
pub const LV2_ATOM__PATH: &[u8] = b"http://lv2plug.in/ns/ext/atom#Path\0";
pub const LV2_ATOM__PROPERTY: &[u8] = b"http://lv2plug.in/ns/ext/atom#Property\0";
pub const LV2_ATOM__RESOURCE: &[u8] = b"http://lv2plug.in/ns/ext/atom#Resource\0";
pub const LV2_ATOM__SEQUENCE: &[u8] = b"http://lv2plug.in/ns/ext/atom#Sequence\0";
pub const LV2_ATOM__SOUND: &[u8] = b"http://lv2plug.in/ns/ext/atom#Sound\0";
pub const LV2_ATOM__STRING: &[u8] = b"http://lv2plug.in/ns/ext/atom#String\0";
pub const LV2_ATOM__TUPLE: &[u8] = b"http://lv2plug.in/ns/ext/atom#Tuple\0";
pub const LV2_ATOM__URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#URI\0";
pub const LV2_ATOM__URID: &[u8] = b"http://lv2plug.in/ns/ext/atom#URID\0";
pub const LV2_ATOM__VECTOR: &[u8] = b"http://lv2plug.in/ns/ext/atom#Vector\0";
pub const LV2_ATOM__ATOMTRANSFER: &[u8] = b"http://lv2plug.in/ns/ext/atom#atomTransfer\0";
pub const LV2_ATOM__BEATTIME: &[u8] = b"http://lv2plug.in/ns/ext/atom#beatTime\0";
pub const LV2_ATOM__BUFFERTYPE: &[u8] = b"http://lv2plug.in/ns/ext/atom#bufferType\0";
pub const LV2_ATOM__CHILDTYPE: &[u8] = b"http://lv2plug.in/ns/ext/atom#childType\0";
pub const LV2_ATOM__EVENTTRANSFER: &[u8] = b"http://lv2plug.in/ns/ext/atom#eventTransfer\0";
pub const LV2_ATOM__FRAMETIME: &[u8] = b"http://lv2plug.in/ns/ext/atom#frameTime\0";
pub const LV2_ATOM__SUPPORTS: &[u8] = b"http://lv2plug.in/ns/ext/atom#supports\0";
pub const LV2_ATOM__TIMEUNIT: &[u8] = b"http://lv2plug.in/ns/ext/atom#timeUnit\0";

/// The header of an atom:Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2Atom {
    /// Size in bytes, not including type and size.
    pub size: u32,
    /// Type of this atom (mapped URI).
    pub type_: u32,
}

/// An atom:Int or atom:Bool.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomInt {
    /// Atom header.
    pub atom: LV2Atom,
    /// Integer value.
    pub body: i32,
}

/// An atom:Long.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomLong {
    /// Atom header.
    pub atom: LV2Atom,
    /// Integer value.
    pub body: i64,
}

/// An atom:Float.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomFloat {
    /// Atom header.
    pub atom: LV2Atom,
    /// Float value.
    pub body: f32,
}

/// An atom:Double.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomDouble {
    /// Atom header.
    pub atom: LV2Atom,
    /// Double value.
    pub body: f64,
}

/// Type alias for atom:Bool (same as atom:Int)
pub type LV2AtomBool = LV2AtomInt;

/// An atom:URID.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomURID {
    /// Atom header.
    pub atom: LV2Atom,
    /// URID.
    pub body: u32,
}

/// An atom:String.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomString {
    /// Atom header.
    pub atom: LV2Atom,
    // Contents (a null-terminated UTF-8 string) follow here.
}

/// The body of an atom:Literal.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomLiteralBody {
    /// Datatype URID.
    pub datatype: u32,
    /// Language URID.
    pub lang: u32,
    // Contents (a null-terminated UTF-8 string) follow here.
}

/// An atom:Literal.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomLiteral {
    /// Atom header.
    pub atom: LV2Atom,
    /// Body.
    pub body: LV2AtomLiteralBody,
}

/// An atom:Tuple.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomTuple {
    /// Atom header.
    pub atom: LV2Atom,
    // Contents (a series of complete atoms) follow here.
}

/// The body of an atom:Vector.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomVectorBody {
    /// The size of each element in the vector.
    pub child_size: u32,
    /// The type of each element in the vector.
    pub child_type: u32,
    // Contents (a series of packed atom bodies) follow here.
}

/// An atom:Vector.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomVector {
    /// Atom header.
    pub atom: LV2Atom,
    /// Body.
    pub body: LV2AtomVectorBody,
}

/// The body of an atom:Property (e.g. in an atom:Object).
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomPropertyBody {
    /// Key (predicate) (mapped URI).
    pub key: u32,
    /// Context URID (may be, and generally is, 0).
    pub context: u32,
    /// Value atom header.
    pub value: LV2Atom,
    // Value atom body follows here.
}

/// An atom:Property.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomProperty {
    /// Atom header.
    pub atom: LV2Atom,
    /// Body.
    pub body: LV2AtomPropertyBody,
}

/// The body of an atom:Object. May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomObjectBody {
    /// URID, or 0 for blank.
    pub id: u32,
    /// Type URID (same as rdf:type, for fast dispatch).
    pub otype: u32,
    // Contents (a series of property bodies) follow here.
}

/// An atom:Object.  May be cast to LV2_Atom.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomObject {
    /// Atom header.
    pub atom: LV2Atom,
    /// Body.
    pub body: LV2AtomObjectBody,
}

impl LV2AtomObject {
    /// Iterate over properties in the object.
    ///
    /// # Safety
    ///
    /// The object must have a valid memory layout with properly formatted property bodies.
    pub unsafe fn foreach<F>(&mut self, mut closure: F)
    where
        F: FnMut(*mut LV2AtomPropertyBody) -> bool,
    {
        unsafe {
            let body = &self.body;
            let mut it = lv2_atom_object_begin(body);
            while !lv2_atom_object_is_end(body, self.atom.size, it) {
                if closure(it) {
                    break;
                }
                it = lv2_atom_object_next(it);
            }
        }
    }
}

/// The header of an atom:Event.  Note this type is NOT an LV2_Atom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomEvent {
    /// Time stamp. Which type is determined by context.
    pub time_in_frames: i64,
    /// Event body atom header.
    pub body: LV2Atom,
}

impl LV2AtomEvent {
    /// Get time stamp as audio frames.
    #[inline]
    #[must_use]
    pub const fn time_as_frames(&self) -> i64 {
        self.time_in_frames
    }

    /// Get time stamp as beats (double).
    ///
    /// # Safety
    ///
    /// Only valid if the sequence uses beat time. Calling this when
    /// the sequence uses frame time will return meaningless data.
    #[inline]
    #[must_use]
    pub unsafe fn time_as_beats(&self) -> f64 {
        unsafe { transmute::<i64, f64>(self.time_in_frames) }
    }
}

/// The body of an atom:Sequence (a sequence of events).
///
/// The unit field is either a URID that described an appropriate time stamp
/// type, or may be 0 where a default stamp type is known.  For
/// LV2_Descriptor::run(), the default stamp type is audio frames.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LV2AtomSequenceBody {
    /// URID of unit of event time stamps.
    pub unit: u32,
    /// Currently unused.
    pub pad: u32,
    // Contents (a series of events) follow here.
}

/// An atom:Sequence.
#[repr(C)]
#[derive(Debug)]
pub struct LV2AtomSequence {
    /// Atom header.
    pub atom: LV2Atom,
    /// Body.
    pub body: LV2AtomSequenceBody,
}

impl LV2AtomSequence {
    /// Iterate over events in the sequence.
    ///
    /// # Safety
    ///
    /// The sequence must have a valid memory layout with properly formatted events.
    pub unsafe fn foreach<F>(&mut self, mut closure: F)
    where
        F: FnMut(*const LV2AtomEvent),
    {
        unsafe {
            let body = &self.body;
            let mut it = lv2_atom_sequence_begin(body);
            while !lv2_atom_sequence_is_end(body, self.atom.size, it) {
                closure(it);
                it = lv2_atom_sequence_next(it);
            }
        }
    }
}
