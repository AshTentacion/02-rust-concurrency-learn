use std::{sync::mpsc, thread, time::Duration};

use anyhow::{Result, anyhow};
use rand::{Rng, RngCore};

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

const PROCEDURE_NUM: usize = 4;
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..PROCEDURE_NUM {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
        println!("Consumer exit");
        42
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("Secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    let mut rng = rand::rng();
    loop {
        let value = rng.next_u64() as usize;
        tx.send(Msg::new(idx, value))?;
        let sleep: u64 = rng.random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep));
        if rng.random::<u8>() % 10 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
