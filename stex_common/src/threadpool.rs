use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};

pub trait Executor<T> {
    fn execute(param: T);
}

pub struct ThreadPool<E, T> {
    sender: Sender<Box<T>>,
    phantom: PhantomData<E>,
}

impl<E, T> ThreadPool<E, T>
where
    E: Executor<T>,
    T: Sync + Send + 'static,
{
    pub fn new(num_threads: usize) -> ThreadPool<impl Executor<T>, T> {
        let (sender, receiver): (Sender<Box<T>>, Receiver<Box<T>>) = channel();

        let receiver_mutex = Arc::new(Mutex::new(receiver));
        for _ in 0..num_threads {
            let receiver_clone = receiver_mutex.clone();
            std::thread::spawn(move || loop {
                let param = receiver_clone.lock().expect("Other thread paniced").recv();
                match param {
                    Ok(param) => E::execute(*param),
                    Err(_) => break,
                }
            });
        }

        ThreadPool {
            sender,
            phantom: PhantomData::<E>,
        }
    }

    pub fn push(&self, param: T) -> Result<(), SendError<Box<T>>> {
        self.sender.send(Box::new(param))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        thread::sleep,
        time::Duration,
    };

    use super::*;

    struct NaiveExecutor;
    type NaiveParam = usize;

    static X: AtomicUsize = AtomicUsize::new(0);

    impl Executor<NaiveParam> for NaiveExecutor {
        fn execute(param: NaiveParam) {
            X.swap(param, Ordering::Relaxed);
        }
    }

    #[test]
    fn it_works() {
        let pool = ThreadPool::<NaiveExecutor, NaiveParam>::new(4);
        assert_eq!(X.load(Ordering::Relaxed), 0);

        pool.push(1).expect("should push 1");
        sleep(Duration::from_secs(1));
        assert_eq!(X.load(Ordering::Relaxed), 1);

        pool.push(2).expect("should push 2");
        sleep(Duration::from_secs(1));
        assert_eq!(X.load(Ordering::Relaxed), 2);

        pool.push(3).expect("should push 3");
        sleep(Duration::from_secs(1));
        assert_eq!(X.load(Ordering::Relaxed), 3);
    }
}
