/**
 * Багатопотокове програмування
 * Лабораторна номер 1
 * F1: 2.25 MF = (MG + TRANS(MH*MK) - TRANS(ML))
 * F2: 3.7 O = (P+R)*(MS*MT)
 * Шпильовий Роман
 * ІП-83
 * 16.12.2020 
**/

mod data;
mod func;

use std::{thread, time::Instant};

use func::{f1, f2};

fn parse_n() -> Result<usize, String> {
    let n = std::env::args()
        .nth(1)
        .ok_or_else(|| "Usage: lab N".to_string())?;
    let n = n
        .parse()
        .map_err(|err| format!("Error parsing N: {}", err))?;
    Ok(n)
}

fn main() -> Result<(), String> {
    let n = parse_n()?;

    let start = Instant::now();
    let handles = vec![thread::spawn(move || f1(n)), thread::spawn(move || f2(n))];
    for handle in handles {
        handle.join().unwrap();
    }
    eprintln!("Elapsed {:?}", start.elapsed());
    Ok(())
}
