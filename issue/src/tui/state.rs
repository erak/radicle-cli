use std::convert::{TryFrom, TryInto};

pub enum Page {
    Overview = 0,
    Edit = 1,
}

impl TryFrom<usize> for Page {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Page::Overview),
            1 => Ok(Page::Edit),
            _ => Err("Page index not allowed!")
        }
    }
}

impl TryInto<usize> for Page {
    type Error = &'static str;

    fn try_into(self) -> Result<usize, Self::Error> {
        Ok(self as usize)
    }
}


pub enum Tab {
    Open = 0,
}