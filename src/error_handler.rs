use std::io;

#[derive(Debug)]
pub enum ErrorHandler{
    ErrString(String),
    ErrIo(io::Error),
}