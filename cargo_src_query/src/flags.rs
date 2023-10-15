use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const EXCLUDE_PRIVATE = 0b00000001;
    }
}

pub const DEFAULT_FLAGS: Flags = Flags::EXCLUDE_PRIVATE;