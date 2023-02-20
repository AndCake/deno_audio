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
use bstr::BStr;

static mut GLOBAL_DATA: Vec<Sink> = Vec::new();

#[no_mangle]
pub extern "C" fn init(voices: u8) {
	for _i in 0..voices {
	    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
	    let sink = Sink::try_new(&handle).unwrap();
		unsafe { GLOBAL_DATA.push(sink) };
	}
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn play(buf: *const u8, len: usize, volume: f32, in_loop: u8) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    let filename: &BStr = slice.into();

    let mut found_entry: Option<&Sink> = None;
    let mut result: i32 = -1;
    let data = unsafe { &GLOBAL_DATA };
	for i in 0..data.len() {
		if data[i].empty() {
			found_entry = Some(&(data[i]));
			result = i as i32;
		}
	}

	match found_entry {
		None => {}
		Some(sink) => {
		    let file = std::fs::File::open(filename.to_string()).unwrap();
		    if in_loop == 1 {
		        sink.append(Decoder::new_looped(BufReader::new(file)).unwrap());
		    } else {
		        sink.append(Decoder::new(BufReader::new(file)).unwrap());
		    }
		    sink.set_volume(volume);
		}
	}

	return result;
}

#[no_mangle]
pub extern "C" fn set_volume(sink_index: i32, new_volume: f32) {
    if sink_index < 0 {
    	return;
    }
    let sink = unsafe { &GLOBAL_DATA[sink_index as usize] };
	sink.set_volume(new_volume);
}

#[no_mangle]
pub extern "C" fn pause(sink_index: i32) {
    if sink_index < 0 {
    	return;
    }
    let sink = unsafe { &GLOBAL_DATA[sink_index as usize] };
	sink.pause();
	sink.stop();
}
