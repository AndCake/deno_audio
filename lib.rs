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
use rodio::*;
use rodio::cpal::traits::{HostTrait,DeviceTrait};

static mut GLOBAL_DATA: Vec<Sink> = Vec::new();

fn list_host_devices() {
   let host = cpal::default_host();
   let devices = host.output_devices().unwrap();
   for device in devices{ 
      let dev:rodio::Device = device;
      let dev_name:String=dev.name().unwrap();
      println!(" # Device : {}", dev_name);
   }
}

fn get_output_stream(_device_name:&str) -> (OutputStream, OutputStreamHandle) {
   //let host = cpal::default_host();
   //let devices = host.output_devices().unwrap();
   let ( _stream, stream_handle) = OutputStream::try_default().unwrap();
/*   for device in devices{ 
      let dev:rodio::Device = device;
      let dev_name:String=dev.name().unwrap();
      if dev_name==device_name {
         ( _stream, stream_handle) = OutputStream::try_from_device(&dev).unwrap();
      }
   }*/
   return (_stream,stream_handle);
}

#[no_mangle]
pub extern "C" fn init(_voices: u8) {
	list_host_devices();
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn play(buf: *const u8, len: usize, volume: f32, in_loop: u8) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    let filename: &BStr = slice.into();

    match std::fs::File::open(filename.to_string()) {
		Ok(file) => {
		    let (_stream, handle) = get_output_stream("default");
			let sink = Sink::try_new(&handle).unwrap();    
		    if in_loop == 1 {
		        sink.append(Decoder::new_looped(BufReader::new(file)).unwrap());
		    } else {
			    sink.append(Decoder::new(BufReader::new(file)).unwrap());
		    }
		    sink.set_volume(volume);
			unsafe { GLOBAL_DATA.push(sink) };
		    let data = unsafe { &GLOBAL_DATA };
		    data[data.len() - 1].sleep_until_end();

		    return data.len() as i32 - 1;
	    },
	    Err(err) => {
		    let (_stream, handle) = get_output_stream("default");
			let sink = Sink::try_new(&handle).unwrap();    
	    	eprintln!("ERROR {}", err);
			unsafe { GLOBAL_DATA.push(sink) };
	    	return -1;
	    }
	}
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
