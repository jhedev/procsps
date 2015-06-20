extern crate libc;
extern crate getopts;
use libc::{c_double, c_int};
use getopts::Options;
use std::env;

#[link(name="procps")]
extern {
    fn uptime(uptime_seconds: *mut c_double, idle_seconds: *mut c_double) -> c_int;
    fn print_uptime(humand_readable: c_int);

}

fn print_uptime_since() {
    panic!("Not yet implemented!");
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief))
}

fn print_version() {
    println!("Version 0.0.1");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("p", "pretty", "show uptime in pretty format");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("s", "since", "system up since");
    opts.optflag("V", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("V") {
        print_version();
        return;
    }
    if matches.opt_present("s") {
        print_uptime_since();
        return;
    }

    let mut p = 0;

    if matches.opt_present("p") {
        p = 1;
    }

    unsafe {
        print_uptime(p as c_int)
    };
}
