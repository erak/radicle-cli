use std::convert::{TryFrom, TryInto};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PageError {
    #[error("{0}")]
    InvalidIndex(String),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Page {
    Overview = 0,
    Detail = 1,
}

impl TryFrom<usize> for Page {
    type Error = PageError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Page::Overview),
            1 => Ok(Page::Detail),
            _ => Err(PageError::InvalidIndex("Page index not allowed!".to_owned()))
        }
    }
}

impl TryInto<usize> for Page {
    type Error = PageError;

    fn try_into(self) -> Result<usize, Self::Error> {
        Ok(self as usize)
    }
}


pub enum Tab {
    Open = 0,
}