#[derive(Clone)]
pub enum Error {
    Success = 0,
    HubGeneric = 10,
    HubFull = 11,
    HubDisabled = 12,
    LoginGeneric = 20,
    NickInvalid = 21,
    NickTaken = 22,
    BadPassword = 23,
    CidTaken = 24,
    CommandAccess = 25,
    ReggedOnly = 26,
    InvalidPid = 27,
    BannedGeneric = 30,
    PermBanned = 31,
    TempBanned = 32,
    ProtocolGeneric = 40,
    ProtocolUnsupported = 41,
    ConnectFailed = 42,
    InfMissing = 43,
    BadState = 44,
    FeatureMissing = 45,
    BadIp = 46,
    NoHubHash = 47,
    TransferGeneric = 50,
    FileNotAvailable = 51,
    FilePartNotAvailable = 52,
    SlotsFull = 53,
    NoClientHash = 54,
}

impl std::string::ToString for Error {
    fn to_string(&self) -> std::string::String {
        (self.clone() as u32).to_string()
    }
}

#[derive(Clone)]
pub enum Severity {
    Success = 0,
    Recoverable = 1,
    Fatal = 2
}

#[derive(Clone)]
pub enum Type {
    Broadcast,
    Client,
    Direct,
    Echo,
    Feature,
    Info,
    Hub,
    Udp,
}

pub fn get_type(x: &Type) -> char {
    match *x {
        Type::Broadcast => 'B',
        Type::Client => 'C',
        Type::Direct => 'D',
        Type::Echo => 'E',
        Type::Feature => 'F',
        Type::Info => 'I',
        Type::Hub => 'H',
        Type::Udp => 'U',
    }
}

impl std::string::ToString for Type {
    fn to_string(&self) -> std::string::String {
        get_type(self).to_string()
    }
}


pub struct AdcCommand {
    pub cmd_type: Type,

    pub from: u32,
    pub to: u32,
}

