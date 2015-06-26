extern crate getopts;
extern crate libc;
extern crate time;
use getopts::Options;
use libc::{c_double, c_int};
use std::env;

#[link(name="procps")]
extern {
    fn loadavg(av1: *mut c_double, av5: *mut c_double, av15: *mut c_double);
    fn uptime(uptime_seconds: *mut c_double, idle_seconds: *mut c_double) -> c_int;
}

//struct utmp;
//static USER_PROCESS: i32 = 7;
//
//#[link(name = "utmp")]
//extern {
//    fn getutent() -> *mut utmp;
//    fn setutent();
//    fn endutent();
//}

fn print_uptime_since() {
    panic!("Not yet implemented!");
}

fn print_uptime(human_readable: bool) {
    let mut res = String::new();

    /* first get the current time */
    if !human_readable {
        let realtime = time::now().to_local();
        res = format!(" {:02}:{:02}:{:02} ", realtime.tm_hour, realtime.tm_min,
                      realtime.tm_sec);
    }

    /* read and calculate the amount of uptime */

    let mut uptime_secs = 0.0;
    let mut idle_secs = 0.0;
    let mut updecades = 0;
    let mut upyears = 0;
    let mut upweeks = 0;
    let mut updays = 0;

    unsafe {
        uptime(&mut uptime_secs, &mut idle_secs);
    }
    if human_readable {
        updecades = uptime_secs as i32 / (60*60*24*365*10);
        upyears = (uptime_secs as i32 / (60*60*24*365)) as i32 % 10;
        upweeks = (uptime_secs as i32 / (60*60*24*7)) as i32 % 52;
        updays = (uptime_secs as i32 / (60*60*24)) % 7;
    } else {
        updays = uptime_secs as i32 / (60*60*24);
    }
    res.push_str("up ");

    if !human_readable {
        if updays > 0 {
            let days_s = if updays != 1 { "s" } else { "" };
            let s = format!("{:02} day{}", updays, days_s);
            res.push_str(&s);
        }
    }

    let mut upminutes = uptime_secs as i32 / 60;
    let mut uphours = upminutes / 60;
    uphours = uphours % 24;
    upminutes = upminutes % 60;

    if !human_readable {
        if uphours > 0 {
            res.push_str(&format!("{:02}:{:02}, ", uphours, upminutes));
        } else {
            res.push_str(&format!("{} min, ", upminutes));
        }

        /* count the number of users */

        let numuser = 0;
        //TODO: Get it working
        //unsafe {
        //    setutent();
        //    let mut utmpstruct = getutent();
        //    while utmpstruct.is_null() {
        //        if (utmpstruct.ut_type == USER_PROCESS) &&
        //            (utmpstruct.ut_name[0] != '\0') {
        //                numuser += 1;
        //            }
        //        utmpstruct = getutent();
        //    }
        //    endutent();
        //}

        let plural =  if numuser > 1 {"s"} else {""};
        res.push_str(&format!("{} user{}, ", numuser, plural));

        let mut av1 = 0.0;
        let mut av5 = 0.0;
        let mut av15 = 0.0;

        unsafe {
            loadavg(&mut av1, &mut av5, &mut av15);
        }

        res.push_str(&format!(" load average: {:.2}, {:.2}, {:.2}", av1, av5, av15));

    }

    if human_readable {
        let mut comma = 0;

        if updecades > 0 {
            let plural = if updecades > 1 { "decades" } else { "decade" };
            res.push_str(&format!("{} {}", updecades, plural));
            comma += 1;
        }

        if upyears > 0 {
            let comma_s = if comma > 0 {", "} else {""};
            let plural = if upyears > 1 {"years"} else {"year"};

            res.push_str(&format!("{}{} {}", comma_s, upyears, plural));
            comma += 1;
        }

        if upweeks > 0 {
            let comma_s = if comma > 0 {", "} else {""};
            let plural = if upyears > 1 {"weeks"} else {"week"};

            res.push_str(&format!("{}{} {}", comma_s, upweeks, plural));
            comma += 1;
        }

        if updays > 0 {
            let comma_s = if comma > 0 {", "} else {""};
            let plural = if upyears > 1 {"days"} else {"day"};

            res.push_str(&format!("{}{} {}", comma_s, updays, plural));
            comma += 1;
        }

        if uphours > 0 {
            let comma_s = if comma > 0 {", "} else {""};
            let plural = if upyears > 1 {"hours"} else {"hour"};

            res.push_str(&format!("{}{} {}", comma_s, uphours, plural));
            comma += 1;
        }

        if upminutes > 0 {
            let comma_s = if comma > 0 {", "} else {""};
            let plural = if upyears > 1 {"minutes"} else {"minute"};

            res.push_str(&format!("{}{} {}", comma_s, upminutes, plural));
        }
    }


    println!("{}", res);
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

    let mut human_readable = false;
    if matches.opt_present("p") {
        human_readable = true;
    }
    print_uptime(human_readable);
}