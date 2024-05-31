use crate::comms::{GenPref, SetSpecial};

use rand::Rng;
use std::collections::HashSet;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const MORE_SYMBOLS: &[u8] = b"@#$%^&*()-,._=+";

pub fn generate_string(prf: GenPref) -> Vec<String> {
    

    
    let char_set = match prf.special_characters {
        SetSpecial::AllMinus(op) => {
            match op {
                Some(chs) => {
                    let mut seen = HashSet::new();
                    let mut cha = chs.clone();
                    cha.retain(|c| seen.insert(*c) );                    
                    let mut s = [CHARSET, MORE_SYMBOLS]
                        .concat();
                        s.retain(|c| !cha.contains(c));
                    s
                }
                None => [CHARSET, MORE_SYMBOLS].concat(),
            }
        }
        SetSpecial::NonePlus(op) => {
            match op {
                Some(chs) => {
                    let mut cha = chs.clone();
                    let mut out = CHARSET.to_vec();
                    out.append(&mut cha);
                    out
                }
                None => CHARSET.to_vec(),
            }
        }
    };


    
    let mut rng = rand::thread_rng();
    let gens: Vec<String> = (0..prf.times)
        .map(|_| (0..prf.length)
        .map(|_| {
            let indx = rng.gen_range(0..char_set.len());
            char_set[indx] as char
        })
        .collect()
    ).collect();

    gens
}
