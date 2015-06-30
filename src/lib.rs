extern crate libc;
use std::fs::File;
use std::io::prelude::*;

//utmpx stuff inspired by:
// https://github.com/uutils/coreutils/blob/master/src/common/utmpx.rs

pub const UT_LINESIZE: usize = 32;
pub const UT_NAMESIZE: usize = 32;
pub const UT_IDSIZE: usize = 4;
pub const UT_HOSTSIZE: usize = 256;

pub const USER_PROCESS: libc::c_short = 7;

#[repr(C)]
pub struct c_exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}

#[repr(C)]
pub struct c_utmp {
    pub ut_type: libc::c_short,
    pub ut_pid: libc::pid_t,
    pub ut_line: [libc::c_char; UT_LINESIZE],
    pub ut_id: [libc::c_char; UT_IDSIZE],

    pub ut_user: [libc::c_char; UT_NAMESIZE],
    pub ut_host: [libc::c_char; UT_HOSTSIZE],
    pub ut_exit: c_exit_status,
    pub ut_session: libc::c_long,
    pub ut_tv: libc::timeval,

    pub ut_addr_v6: [libc::c_int; 4],
    pub __unused: [libc::c_char; 20],
}

extern {
    pub fn getutxent() -> *const c_utmp;
    pub fn setutxent();
    pub fn endutxent();
}

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
