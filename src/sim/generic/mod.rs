use std::sync::mpsc::{self};

pub fn receive_nonblocking<T: Clone>(receiver: &mpsc::Receiver<T>, current: &T) -> T {
    let result = receiver.try_recv();
    if let Result::Ok(val) = result {
        return val;
    }

    return current.clone();
}