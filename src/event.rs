use std::fmt;
use neovim_lib::neovim_api::Buffer;

pub enum Event {
    LiveUpdateStart {
        buf: Buffer,
        changedtick: u64,
        linedata: Vec<String>,
        more: bool,
    },
    LiveUpdate {
        buf: Buffer,
        changedtick: u64,
        firstline: u64,
        numreplaced: u64,
        linedata: Vec<String>,
    },
    LiveUpdateTick { buf: Buffer, changedtick: u64 },
    LiveUpdateEnd { buf: Buffer },
    Quit,
}

//impl Event {
//    pub fn name(&self) -> &'static str {
//        use Event::*;
//        match *self {
//            LiveUpdateStart { .. } => "LiveUpdateStart",
//            LiveUpdate { .. } => "LiveUpdate",
//            LiveUpdateTick { .. } => "LiveUpdateTick",
//            LiveUpdateEnd { .. } => "LiveUpdateEnd",
//            Quit => "Quit",
//        }
//    }
//}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Event::*;

        match *self {
            LiveUpdateStart {
                changedtick,
                ref linedata,
                more,
                ..
            } => {
                write!(
                    f,
                    "LiveUpdateStart{{ changedtick: {}, #linedata: {}, \
                                       more: {} }}",
                    changedtick,
                    linedata.len(),
                    more
                )
            }
            LiveUpdate {
                changedtick,
                firstline,
                numreplaced,
                ref linedata,
                ..
            } => {
                write!(
                    f,
                    "LiveUpdate{{ changedtick: {}, firstline: {}, \
                                  numreplaced: {}, #linedata: {} }}",
                    changedtick,
                    firstline,
                    numreplaced,
                    linedata.len()
                )
            }
            LiveUpdateTick { changedtick, .. } => {
                write!(
                    f,
                    "LiveUpdateTick{{ changedtick: {} }}",
                    changedtick,
                )
            }
            LiveUpdateEnd { .. } => write!(f, "LiveUpdateEnd"),
            Quit => write!(f, "Quit"),
        }
    }
}
