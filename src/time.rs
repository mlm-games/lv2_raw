// Copyright 2017 Michael Oswald

// Documentation copied from http://lv2plug.in/ns/ext/time/time.h

// Copyright text of the original C file:

// Copyright 2011-2016 David Robillard <http://drobilla.net>

// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.

// THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Documentation of the corresponding C header files: <http://lv2plug.in/ns/ext/time/time.html>.

//! Properties for describing time, see <http://lv2plug.in/ns/ext/time> for
//! details.
//!
//! Note the time extension is purely data, this header merely defines URIs for
//! convenience.

pub static LV2_TIME_URI: &[u8] = b"http://lv2plug.in/ns/ext/time\0";
pub static LV2_TIME_PREFIX: &[u8] = b"http://lv2plug.in/ns/ext/time#\0";

pub static LV2_TIME__TIME: &[u8] = b"http://lv2plug.in/ns/ext/time#Time\0";
pub static LV2_TIME__POSITION: &[u8] = b"http://lv2plug.in/ns/ext/time#Position\0";
pub static LV2_TIME__RATE: &[u8] = b"http://lv2plug.in/ns/ext/time#Rate\0";
pub static LV2_TIME___POSITION: &[u8] = b"http://lv2plug.in/ns/ext/time#position\0";
pub static LV2_TIME__BARBEAT: &[u8] = b"http://lv2plug.in/ns/ext/time#barBeat\0";
pub static LV2_TIME__BAR: &[u8] = b"http://lv2plug.in/ns/ext/time#bar\0";
pub static LV2_TIME__BEAT: &[u8] = b"http://lv2plug.in/ns/ext/time#beat\0";
pub static LV2_TIME__BEATUNIT: &[u8] = b"http://lv2plug.in/ns/ext/time#beatUnit\0";
pub static LV2_TIME__BEATSPERBAR: &[u8] = b"http://lv2plug.in/ns/ext/time#beatsPerBar\0";
pub static LV2_TIME__BEATSPERMINUTE: &[u8] = b"http://lv2plug.in/ns/ext/time#beatsPerMinute\0";
pub static LV2_TIME__FRAME: &[u8] = b"http://lv2plug.in/ns/ext/time#frame\0";
pub static LV2_TIME__FRAMESPERSECOND: &[u8] = b"http://lv2plug.in/ns/ext/time#framesPerSecond\0";
pub static LV2_TIME__SPEED: &[u8] = b"http://lv2plug.in/ns/ext/time#speed\0";
