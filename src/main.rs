extern crate getopts;
extern crate byteorder;

use std::net;
use std::fmt;
use std::convert;
use std::env;

use getopts::Options;
use byteorder::{BigEndian, ByteOrder};

mod enums;
use enums::*;


struct SNTPResponse {
    leap_indicator: LeapIndicator,
    version: Version,
    mode: Mode,
    stratum: u8,
    precision: i8,
    trans_ts: u32,
    trans_ts_frac: u32,
}

impl convert::From<[u8; 48]> for SNTPResponse {
    fn from(res: [u8; 48]) -> SNTPResponse {
        SNTPResponse {
            leap_indicator: LeapIndicator::from(res[0] >> 6),
            version: Version::from((res[0] & 0x38) >> 3),
            mode: Mode::from(res[0] & 0x07),
            stratum: res[1],
            precision: res[3] as i8,
            trans_ts: BigEndian::read_u32(&res[40 .. 44]) - 2208988800,
            trans_ts_frac: BigEndian::read_u32(&res[44 .. 48]),
        }
    }
}

impl fmt::Display for SNTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               r#"Leap indicator: {}
Version number: {}
 Response mode: {}
       Stratum: {}
     Precision: {}
    UNIX epoch: {}.{}"#,
               self.leap_indicator,
               self.version,
               self.mode,
               self.stratum,
               self.precision,
               self.trans_ts, self.trans_ts_frac)
    }
}


fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message.");
    opts.optopt("s", "server", "NTP server to query.", "HOST");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(e) => { panic!("{}", e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let server = match matches.opt_str("s") {
        Some(s) => { s }
        None    => { "0.pool.ntp.org".to_string() }
    };

    let mut request: [u8; 48] = [0; 48];
    let mut response: [u8; 48] = [0; 48];

    request[0] |= Into::<u8>::into(LeapIndicator::AlarmCondition) << 6;
    request[0] |= Into::<u8>::into(Version::V4) << 3;
    request[0] |= Into::<u8>::into(Mode::Client);

    let socket = net::UdpSocket::bind(("0.0.0.0", 0)).unwrap();
    socket.send_to(&request, (server.as_str(), 123)).expect("Failed sending SNTP request");
    socket.recv_from(&mut response).expect("Failed receiving SNTP response");

    let sntp_response = SNTPResponse::from(response);
    println!("{}", sntp_response);
}
