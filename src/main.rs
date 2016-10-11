extern crate byteorder;

use std::net;
use std::fmt;
use std::convert;

use byteorder::{BigEndian, ByteOrder};


enum LeapIndicator {
    NoWarning,
    LastSixtyOne,
    LastFiftyNine,
    AlarmCondition,
}

impl convert::From<u8> for LeapIndicator {
    fn from(li: u8) -> LeapIndicator {
        match li {
            0 => LeapIndicator::NoWarning,
            1 => LeapIndicator::LastSixtyOne,
            2 => LeapIndicator::LastFiftyNine,
            3 => LeapIndicator::AlarmCondition,
            _ => LeapIndicator::AlarmCondition, // Until TryFrom is stabilized
        }
    }
}

impl convert::Into<u8> for LeapIndicator {
    fn into(self) -> u8 {
        match self {
            LeapIndicator::NoWarning => 0,
            LeapIndicator::LastSixtyOne => 1,
            LeapIndicator::LastFiftyNine => 2,
            LeapIndicator::AlarmCondition => 3,
        }
    }
}

impl fmt::Display for LeapIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LeapIndicator::NoWarning => write!(f, "No warning"),
            LeapIndicator::LastSixtyOne => write!(f, "Last minute has 61 seconds"),
            LeapIndicator::LastFiftyNine => write!(f, "Last minute has 59 seconds"),
            LeapIndicator::AlarmCondition => write!(f, "Alarm condition (clock not synchronized)"),
        }
    }
}



enum Version {
    V4,
    Unknown
}

impl convert::From<u8> for Version {
    fn from(v: u8) -> Version {
        match v {
            4 => Version::V4,
            _ => Version::Unknown,
        }
    }
}
impl convert::Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Version::V4 => 4,
            Version::Unknown => 0,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version::V4 => write!(f, "4"),
            Version::Unknown => write!(f, "Unknown"),
        }
    }
}



enum Mode {
    Reserved,
    SymmetricActive,
    SymmetricPassive,
    Client,
    Server,
    Broadcast,
    ReservedControl,
    ReservedPrivate,
}

impl convert::From<u8> for Mode {
    fn from(i: u8) -> Mode {
        match i {
            0 => Mode::Reserved,
            1 => Mode::SymmetricActive,
            2 => Mode::SymmetricPassive,
            3 => Mode::Client,
            4 => Mode::Server,
            5 => Mode::Broadcast,
            6 => Mode::ReservedControl,
            7 => Mode::ReservedPrivate,
            _ => Mode::ReservedPrivate, // Until TryFrom is stabilized
        }
    }
}

impl convert::Into<u8> for Mode {
    fn into(self) -> u8 {
        match self {
            Mode::Reserved => 0,
            Mode::SymmetricActive => 1,
            Mode::SymmetricPassive => 2,
            Mode::Client => 3,
            Mode::Server => 4,
            Mode::Broadcast => 5,
            Mode::ReservedControl => 6,
            Mode::ReservedPrivate => 7,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Reserved => write!(f, "Reserved"),
            Mode::SymmetricActive => write!(f, "Symmetric active"),
            Mode::SymmetricPassive => write!(f, "Symmetric passive"),
            Mode::Client => write!(f, "Client"),
            Mode::Server => write!(f, "Server"),
            Mode::Broadcast => write!(f, "Broadcast"),
            Mode::ReservedControl => write!(f, "Reserved for NTP control message"),
            Mode::ReservedPrivate => write!(f, "Reserved for private use"),
        }
    }
}



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



fn main() {
    let mut request: [u8; 48] = [0; 48];
    let mut response: [u8; 48] = [0; 48];

    request[0] |= Into::<u8>::into(LeapIndicator::AlarmCondition) << 6;
    request[0] |= Into::<u8>::into(Version::V4) << 3;
    request[0] |= Into::<u8>::into(Mode::Client);

    let socket = net::UdpSocket::bind(("0.0.0.0", 0)).unwrap();
    socket.send_to(&request, ("0.pool.ntp.org", 123));
    socket.recv_from(&mut response);

    let sntp_response = SNTPResponse::from(response);
    println!("{}", sntp_response);
}
