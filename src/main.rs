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

fn read_cap(filename: &str) -> result::Result<Lease, io::Error> {
    let mut part1 = File::open(filename)?;
    let mut pile_of_bytes: Vec<u8> = vec![0; 2500];
    part1.read(&mut pile_of_bytes).unwrap();
    let mut rdr = Cursor::new(pile_of_bytes);
    let share: Lease = rdr.read_be().unwrap();
    Ok(share)
}


#[cfg(test)]
mod tests {
    // for 1of2.0 and 1of2.1 :
    // wellKnownConvergenceSecret = decodeBase32Unpadded "lcngfrvgaksfwrelc6ae5kucb3zufssoe6cj74rozcqibnl6uy2a"
    // cap = "URI:CHK:pyv3qypbpk6knq5ozeibenuubq:jh3twlgmxtytwqtzn6jtbsfy2w574ybkcnalurlnlq2snuu3j5da:1:2:56"
    use super::*;
    #[test]
    fn basic_read_lease() {
	let s = read_cap("1of2.0").unwrap();
	assert_eq!(s.version, 2);
        assert_eq!(s.data_length, 1906);
        assert_eq!(s.count, 1);

	let s = read_cap("1of2.1").unwrap();
	assert_eq!(s.version, 2);
        assert_eq!(s.data_length, 1906);
        assert_eq!(s.count, 1);
    }

    #[test]
    fn sanity_check_bytes() -> Result<()>{
	let s = read_cap("1of2.0").unwrap();
        let raw_file = File::open("1of2.0")?;

        assert_eq!(s.share_data.len(), s.data_length as usize);
        // check that our assumptions about this file layout are
        // valid: there are 3 longs as the "header" and one "lease" on
        // the end, so everything else should be share-data
        assert_eq!(raw_file.metadata()?.len(), (s.data_length + 12 + (72 * s.count)) as u64);
        Ok(())
    }

    #[test]
    fn read_a_share() {
	let lease = read_cap("1of2.0").unwrap();
        let mut rdr = Cursor::new(lease.share_data);
        let share: Share = rdr.read_be().unwrap();

        // stuff we "just know" about this Share test-vector
        assert_eq!(share.version, 1);
        assert_eq!(share.block_size, 8);
        assert_eq!(share.data_size, 56);
    }
}
