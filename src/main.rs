mod leftright;

use crate::leftright::*;

use std::{collections::HashMap, time::Instant};

#[derive(Default)]
struct Mutex_<T>(std::sync::Mutex<T>);
#[derive(Default)]
struct TokioMutex_<T>(tokio::sync::Mutex<T>);
#[derive(Default)]
struct RwLock_<T>(std::sync::RwLock<T>);
#[derive(Default)]
struct TokioRwLock_<T>(tokio::sync::RwLock<T>);

const ITER: usize = 10_000usize;
const RUNS_PER_ITER: usize = 100_000usize;

#[tokio::main]
async fn main() {
    read().await;
    //write().await;
}

#[allow(dead_code)]
async fn read() {
    for r in 0..ITER {
        let mutex = Mutex_::<HashMap<u32, u32>>::default();
        let t_mutex = TokioMutex_::<HashMap<u32, u32>>::default();
        let rwlock = RwLock_::<HashMap<u32, u32>>::default();
        let t_rwlock = TokioRwLock_::<HashMap<u32, u32>>::default();
        let left_right = LeftRight::<HashMap<u32, u32>>::default();

        let instant_mutex = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            mutex.0.lock().unwrap().get(&5).map(|x| x.clone());
        }
        let instant_mutex = instant_mutex.elapsed().as_micros();

        let instant_t_mutex = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            t_mutex.0.lock().await.get(&5).map(|x| x.clone());
        }
        let instant_t_mutex = instant_t_mutex.elapsed().as_micros();

        let instant_rwlock = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            rwlock.0.read().unwrap().get(&5).map(|x| x.clone());
        }
        let instant_rwlock = instant_rwlock.elapsed().as_micros();

        let instant_t_rwlock = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            t_rwlock.0.read().await.get(&5).map(|x| x.clone());
        }
        let instant_t_rwlock = instant_t_rwlock.elapsed().as_micros();

        let instant_leftright = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            left_right.read(|x| x.get(&5).map(|x| x.clone()) );
        }
        let instant_leftright = instant_leftright.elapsed().as_micros();

        println!("{:5} {:10} {:10} {:10} {:10} {:10}", r, instant_leftright, instant_mutex, instant_rwlock, instant_t_mutex, instant_t_rwlock);
    }
}

#[allow(dead_code)]
async fn write() {
    for r in 0..ITER {
        let mutex = Mutex_::<HashMap<u32, u32>>::default();
        let t_mutex = TokioMutex_::<HashMap<u32, u32>>::default();
        let rwlock = RwLock_::<HashMap<u32, u32>>::default();
        let t_rwlock = TokioRwLock_::<HashMap<u32, u32>>::default();
        let left_right = LeftRight::<HashMap<u32, u32>>::default();

        let instant_mutex = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            mutex.0.lock().unwrap().entry(5).and_modify(|x| *x += 1).or_insert(0);
        }
        let instant_mutex = instant_mutex.elapsed().as_micros();

        let instant_t_mutex = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            t_mutex.0.lock().await.entry(5).and_modify(|x| *x += 1).or_insert(0);
        }
        let instant_t_mutex = instant_t_mutex.elapsed().as_micros();

        let instant_rwlock = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            rwlock.0.write().unwrap().entry(5).and_modify(|x| *x += 1).or_insert(0);
        }
        let instant_rwlock = instant_rwlock.elapsed().as_micros();

        let instant_t_rwlock = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            t_rwlock.0.write().await.entry(5).and_modify(|x| *x += 1).or_insert(0);
        }
        let instant_t_rwlock = instant_t_rwlock.elapsed().as_micros();

        let instant_leftright = Instant::now();
        for _ in 0..RUNS_PER_ITER {
            left_right.write(|x| { x.entry(5).and_modify(|x| *x += 1).or_insert(0); } );
        }
        let instant_leftright = instant_leftright.elapsed().as_micros();

        println!("{:5} {:10} {:10} {:10} {:10} {:10}", r, instant_leftright, instant_mutex, instant_rwlock, instant_t_mutex, instant_t_rwlock);
    }
}
