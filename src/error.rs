use std::io;

pub fn no_id() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "Id not generate")
}
pub fn no_playlist() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "No playlist found")
}
