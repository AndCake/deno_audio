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

struct Sound {
	sink: Sink,
	duration: u64,	
}

static mut GLOBAL_DATA: Vec<Sound> = Vec::new();

fn list_host_devices() {
   println!("Available audio devices: ");
   let host = cpal::default_host();
   let devices = host.output_devices().unwrap();
   for device in devices { 
      let dev: rodio::Device = device;
      let dev_name: String = dev.name().unwrap();
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
			let duration: u64;
		    if in_loop == 1 {
			    let source = Decoder::new_looped(BufReader::new(file)).unwrap();
			    duration = 0;
		        sink.append(source);
		    } else {
		    	let source = Decoder::new(BufReader::new(file)).unwrap();
		        match source.total_duration() {
			        None => {
				        duration = 0;
				    },
		        	Some(dur) => {
		        		duration = dur.as_millis() as u64;
		        	}
		        }
		        sink.append(source);
		    }
		    sink.set_volume(volume);
		    
			unsafe { GLOBAL_DATA.push(Sound{
				sink,
				duration,
			}) };
		    let data = unsafe { &GLOBAL_DATA };
		    data[data.len() - 1].sink.sleep_until_end();

		    return data.len() as i32 - 1;
	    },
	    Err(err) => {
		    let (_stream, handle) = get_output_stream("default");
			let sink = Sink::try_new(&handle).unwrap();    
	    	eprintln!("ERROR {}", err);
			unsafe { GLOBAL_DATA.push(Sound{
				sink,
				duration: 0 as u64,
			}) };
	    	return -1;
	    }
	}
}

#[no_mangle]
pub extern "C" fn duration(sink_index: i32) -> u64 {
    if sink_index < 0 {
    	return 0;
    }
	let sound = unsafe { &GLOBAL_DATA[sink_index as usize] };
	return sound.duration;    
}

#[no_mangle]
pub extern "C" fn has_stopped(sink_index: i32) -> u8 {
    if sink_index < 0 {
    	return 1;
    }
   
	let sound = unsafe { &GLOBAL_DATA[sink_index as usize] };
	if sound.sink.empty() {
		return 1 as u8;
	}
	return 0 as u8;
}

#[no_mangle]
pub extern "C" fn set_volume(sink_index: i32, new_volume: f32) {
    if sink_index < 0 {
    	return;
    }
    let sound = unsafe { &GLOBAL_DATA[sink_index as usize] };
	sound.sink.set_volume(new_volume);
}

#[no_mangle]
pub extern "C" fn pause(sink_index: i32) {
    if sink_index < 0 {
    	return;
    }
    let sound = unsafe { &GLOBAL_DATA[sink_index as usize] };
	sound.sink.pause();
	sound.sink.stop();
}
