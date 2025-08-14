extern crate libc;
extern crate lv2_raw;

use lv2_raw::*;
use std::mem;

const BUFFER_SIZE: usize = 64; // Need to define constant for buffer allocation
const EVENT_TYPE_1: u32 = 7; // random type
const EVENT_TYPE_2: u32 = 8;
const EVENT_TIME_1: i64 = 33333; // random data
const EVENT_TIME_2: i64 = 44444;
// event pad size is 64 bits, using u64 no is padding necessary
const ATOM_DATA_1: u64 = 11; // random data
const ATOM_DATA_2: u64 = 22;

fn get_buf() -> State {
    // Construct a sequence of two events by hand:

    // How much memory must be allocated?
    // 1 * (size of LV2AtomSequence)
    // 2 * (size of event1)
    // 2 * 8 for one u64 data per atom

    let s_seq = mem::size_of::<LV2AtomSequence>() as isize;
    let s_ev = mem::size_of::<LV2AtomEvent>() as isize;
    let s_atom = 8 as isize;
    let n = s_seq + 2 * s_ev + 2 * s_atom;
    if n != BUFFER_SIZE as isize {
        panic!(
            "Need to adjust buffer size. Size is {}. Buffer is {}.",
            n, BUFFER_SIZE
        )
    }
    let s_atom_header = mem::size_of::<LV2Atom>() as isize;
    let atom = LV2Atom {
        // Size in bytes, not including type and size.
        size: BUFFER_SIZE as u32 - s_atom_header as u32,
        // Type of this atom (mapped URI).
        type_: 0,
    };

    let seqbody = LV2AtomSequenceBody {
        // URID of unit of event time stamps.
        unit: 0,
        // Currently unused.
        pad: 0, // Contents (a series of events) follow here.
    };

    let sequence = LV2AtomSequence {
        // Atom header.
        atom: atom,
        // Body.
        body: seqbody,
    };

    ////////////////////////////////////////
    let atom_ev1 = LV2Atom {
        size: s_atom as u32,
        type_: EVENT_TYPE_1,
    };
    let atom_ev2 = LV2Atom {
        size: s_atom as u32,
        type_: EVENT_TYPE_2,
    };
    let event1 = LV2AtomEvent {
        time_in_frames: EVENT_TIME_1,
        body: atom_ev1,
    };
    let event2 = LV2AtomEvent {
        time_in_frames: EVENT_TIME_2,
        body: atom_ev2,
    };

    let buf = [1u8; BUFFER_SIZE];

    let mut state = State {
        buf: buf,
        current: 0,
    };

    let p = &sequence as *const LV2AtomSequence as *const libc::c_void;
    state.append(p, s_seq);

    // Event 1
    let p = &event1 as *const LV2AtomEvent as *const libc::c_void;
    state.append(p, s_ev);
    let p = &ATOM_DATA_1 as *const u64 as *const libc::c_void;
    state.append(p, s_atom);

    // Event 2
    let p = &event2 as *const LV2AtomEvent as *const libc::c_void;
    state.append(p, s_ev);
    let p = &ATOM_DATA_2 as *const u64 as *const libc::c_void;
    state.append(p, s_atom);

    state
}

#[test]
fn it_works() {
    let truth = [
        EVENT_TIME_1 as u64,
        EVENT_TYPE_1 as u64,
        ATOM_DATA_1,
        EVENT_TIME_2 as u64,
        EVENT_TYPE_2 as u64,
        ATOM_DATA_2,
    ];

    let s_atom_header = mem::size_of::<LV2Atom>() as isize;

    let mut cnt = 0;
    let state = get_buf();

    unsafe {
        // next line basically says
        //  "let seq = &state.buf[0] as &LV2AtomSequence;"
        // but that's not allowed by the compiler
        let seq = &*(&state.buf[0] as *const u8 as *const LV2AtomSequence);
        for ev in seq {
            println! {"*************TIME: {}", ev.time_in_frames}
            assert_eq!(ev.time_in_frames as u64, truth[cnt]);

            println! {"*************ATOM.type_: {}", ev.body.type_}
            assert_eq!(ev.body.type_ as u64, truth[cnt + 1]);

            let atomptr = &ev.body as *const LV2Atom as *const u8;

            let dataptr = atomptr.offset(s_atom_header);
            let data = *(dataptr as *const u64);
            println! {"************ data: {}", data};
            assert_eq!(data as u64, truth[cnt + 2]);

            cnt = cnt + 3;
        }
        // did we really loop throuh *2* events?
        assert_eq!(cnt, 6)
    }
}

struct State {
    buf: [u8; BUFFER_SIZE],
    current: isize,
}

impl State {
    fn append(&mut self, p: *const libc::c_void, size: isize) {
        let p1 = &mut self.buf[0] as *mut u8 as *mut libc::c_void;
        unsafe {
            libc::memcpy(p1.offset(self.current), p, size as usize);
        }
        self.current = self.current + size;
    }
}
