use crate::io::Cursor;
use std::result;
use std::fs::File;
use std::io;

use binrw::io::*;
use binrw::BinReaderExt;
//use rs_merkle::{Hasher,MerkleTree};
//use rs_merkle::algorithms::*;


use magic_tahoe::lib::*;


fn main() {
}

fn read_cap(filename: &str) -> result::Result<Share, io::Error> {
    let mut part1 = File::open(filename)?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part1.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let share: Share = rdr.read_be().unwrap();
    Ok(share)
}


#[cfg(test)]
mod tests {
    // for 1of2.0 and 1of2.1 :
    // wellKnownConvergenceSecret = decodeBase32Unpadded "lcngfrvgaksfwrelc6ae5kucb3zufssoe6cj74rozcqibnl6uy2a"
    // cap = "URI:CHK:pyv3qypbpk6knq5ozeibenuubq:jh3twlgmxtytwqtzn6jtbsfy2w574ybkcnalurlnlq2snuu3j5da:1:2:56"
    use super::*;
    #[test]
    fn it_works() {
	let s = read_cap("1of2.0").unwrap();
	assert_eq!(s.lease_version, 2);
    }
}
