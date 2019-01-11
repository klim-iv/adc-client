use base32;

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

#[derive(Clone, Copy)]
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

    pub cmd: String,

    inner_cmd: Vec<u8>,
}

impl AdcCommand {
    pub fn new(_type: Type, _from: u32, _to: u32, _cmd: String) -> AdcCommand {
        AdcCommand { cmd_type: _type, from: _from, to: _to, cmd: _cmd, inner_cmd: Vec::new() }
    }

    pub fn to_vec(mut self) -> Vec<u8> {
        self.inner_cmd.push(get_type(&self.cmd_type) as u8);

        let mut _cmd = self.cmd.clone().into_bytes();
        self.inner_cmd.append(&mut _cmd);

        let mut _from = base32::encode(base32::Alphabet::RFC4648 {padding: false}, &self.from.to_be_bytes()).into_bytes();
        self.inner_cmd.append(&mut _from);

        self.inner_cmd
    }
}

impl From<Vec<u8>> for AdcCommand {
    fn from(data: Vec<u8>) -> AdcCommand {
        AdcCommand::new(Type::Udp, 77, 56, "XYZ".to_string())
    }
}
