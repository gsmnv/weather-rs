#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;
extern crate linked_hash_map;
extern crate chrono;

pub mod wunderground;
