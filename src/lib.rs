use std::fs::File;
use std::io::prelude::*;

pub fn uptime() -> (f64, f64) {
    let mut f = match File::open("/proc/uptime") {
        Ok(file) => file,
        Err(_) => panic!("panic")
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => (),
        Err(_) => panic!("panic")
    }
    let split = s.split(' ');
    let vec: Vec<&str> = split.collect();
    let uptime_secs: f64 = vec[0].trim().parse().unwrap();
    let idle_secs: f64 = vec[1].trim().parse().unwrap();
    (uptime_secs, idle_secs)
}

pub fn loadavg() -> (f64, f64, f64) {
    let mut f = match File::open("/proc/loadavg") {
        Ok(file) => file,
        Err(_) => panic!("panic")
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => (),
        Err(_) => panic!("panic")
    }
    let split = s.split(' ');
    let vec: Vec<&str> = split.collect();
    let av1: f64 = vec[0].trim().parse().unwrap();
    let av5: f64 = vec[1].trim().parse().unwrap();
    let av15: f64 = vec[2].trim().parse().unwrap();
    (av1, av5, av15)
}
