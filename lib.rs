//! Deno Audio playback library with rodio.
//!
//! The main purpose of this library is to play audio from Deno.
//!
//! For example, here is how you would play an audio file:
//!
//! ```typescript
//! // example.ts
//! await play("examples/music.mp3");
//! ```
use rodio::Decoder;
use rodio::Sink;
use std::io::BufReader;

static mut GLOBAL_DATA: Vec<Sink> = Vec::new();

unsafe fn ptr_to_string(ptr: *const u8, len: usize) -> String {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)).to_owned()
}

#[no_mangle]
pub unsafe extern "C" fn play(buf: *const u8, len: usize, volume: f32) {
    //!# Safety
    let filename = ptr_to_string(buf, len);
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(filename).unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());
    sink.set_volume(volume);

    let idx: usize;
    {
        GLOBAL_DATA.push(sink);
        idx = GLOBAL_DATA.len() - 1;
    } // write lock should be dropped here...

    GLOBAL_DATA[idx].sleep_until_end();
}

#[no_mangle]
pub unsafe extern "C" fn set_volume(index: i32, new_volume: f32) {
    //!# Safety
    GLOBAL_DATA[index as usize].set_volume(new_volume);
}

#[no_mangle]
pub unsafe extern "C" fn pause(index: i32) {
    //!# Safety
    GLOBAL_DATA[index as usize].pause();
    GLOBAL_DATA[index as usize].stop();
}
