#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod hal {
    include!(concat!(env!("OUT_DIR"), "/hal-bindings.rs"));
}
include!("AnalogInput.rs");
