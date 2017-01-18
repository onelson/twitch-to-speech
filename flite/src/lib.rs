#![allow(
non_camel_case_types,
non_upper_case_globals,
non_snake_case)]

extern crate libc;
use std::ffi::{CString, CStr};
use std::collections::HashMap;

pub enum Voice {}


// voices on my system: kal awb_time kal16 awb rms slt

#[link(name="flite")]
#[link(name="flite_usenglish")]
#[link(name="flite_cmulex")]
#[link(name="flite_cmu_us_kal")]
#[link(name="flite_cmu_us_kal16")]
#[link(name="flite_cmu_us_awb")]  // also provides the awb_time voice
#[link(name="flite_cmu_us_rms")]
#[link(name="flite_cmu_us_slt")]
extern "C" {

    pub fn register_cmu_us_kal(voxdir: *const ::std::os::raw::c_char) -> *mut Voice;
    pub fn register_cmu_us_kal16(voxdir: *const ::std::os::raw::c_char) -> *mut Voice;
    pub fn register_cmu_us_awb(voxdir: *const ::std::os::raw::c_char) -> *mut Voice;
    pub fn register_cmu_us_rms(voxdir: *const ::std::os::raw::c_char) -> *mut Voice;
    pub fn register_cmu_us_slt(voxdir: *const ::std::os::raw::c_char) -> *mut Voice;

    pub fn flite_init() -> ::std::os::raw::c_int;

    // TODO: add a streaming synth version
    pub fn flite_text_to_speech(text: *const ::std::os::raw::c_char,
                                voice: *mut Voice,
                                outtype: *const ::std::os::raw::c_char) -> f32;
}

pub struct Flite {
    pub voices: HashMap<String, *mut Voice>
}

impl Flite {
    pub fn new() -> Self {
        Flite {
            voices: init()
        }
    }

    pub fn say(&self, text: &str, voice: String) -> SpeechDuration {
        text_to_speech(text, *self.voices.get(&voice).unwrap(), OutType::Play)
    }
}


fn init() -> HashMap<String, *mut Voice> {
    unsafe {
        flite_init();
        let mut voices = HashMap::new();
        voices.insert("cmu_us_kal".to_string(), register_cmu_us_kal(std::ptr::null()));
        voices.insert("cmu_us_awb_time".to_string(), register_cmu_us_kal(std::ptr::null()));
        voices.insert("cmu_us_kal16".to_string(), register_cmu_us_kal(std::ptr::null()));
        voices.insert("cmu_us_awb".to_string(), register_cmu_us_kal(std::ptr::null()));
        voices.insert("cmu_us_rms".to_string(), register_cmu_us_rms(std::ptr::null()));
        voices.insert("cmu_us_slt".to_string(), register_cmu_us_slt(std::ptr::null()));
        voices
    }
}

pub enum OutType {
    File(String),
    None,
    Play
}

pub type SpeechDuration = f32;

pub fn text_to_speech(text: &str, voice: *mut Voice, outtype: OutType) -> SpeechDuration {
    let out = match outtype {
        OutType::None => "none".to_string(),
        OutType::Play => "play".to_string(),
        OutType::File(path) => path
    };

    unsafe {
        let c_text = CString::new(text).unwrap();
        let c_out = CString::new(out).unwrap();
        flite_text_to_speech(
            c_text.as_ptr(),
            voice,
            c_out.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let tts = Flite::new();
    }
}