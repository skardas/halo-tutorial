extern crate core;
use ff::PrimeField;
use group::ff::Field;
use pairing::bn256::{Fr as Fp};
use rand_core::OsRng;
use core::convert::TryInto;

fn main() {
    let rng = OsRng;
    let a = Fp::from(3);
    let b = Fp::from(8);
    let c = Fp::from(121);
    let d = Fp::random(rng);
    let result = (a + b).square();
    println!("({} + {})^2 is {}", fr2num(a), fr2num(b), fr2num(result));
    println!("{} == {} is {}", fr2num(result), fr2num(c), result == c);
    println!("Next random number is {}", fr2num(d));
 }


fn fr2num(num :Fp) ->u32{
    let x = &num.to_repr().to_vec()[0..4];

    u32::from_le_bytes(x.try_into().unwrap() )
}