pub enum ModEventType {
    Load = 0,
    Unload = 1,
    Shutdown = 2,
    Quiesce = 3,
    Unknown = 4,
}
impl From<i32> for ModEventType {
    fn from(i: i32) -> Self {
        match i {
            0 => ModEventType::Load,
            1 => ModEventType::Unload,
            2 => ModEventType::Shutdown,
            3 => ModEventType::Quiesce,
            _ => ModEventType::Unknown,
        }
    }
}
