mod pb;
mod storage;
mod error;
mod service;

pub use pb::abi::*;
pub use error::KvError;
pub use storage::*;
pub use service::*;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
