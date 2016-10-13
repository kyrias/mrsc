use std::fmt;
use std::convert;


pub enum LeapIndicator {
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



pub enum Version {
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



pub enum Mode {
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




