pub mod lib {
    use binrw::*;
    use std::fmt::Debug;

    #[derive(BinRead, PartialEq, Debug)]
    pub struct Lease {
	pub version: u32,
	pub data_length: u32,
	pub count: u32,

// per Tahoe-LAFS source code, the above is the header of a "least file"
// each "lease" is 72 bytes: a long, two 32-byte keys, and another long
// so there should be count * 72 "trailing" bytes of lease information
// (claims are made "count" can be at most 4 .. before it stores leases elsewhere?)

// we want "count" here to be: total file size, minus 12, minus (72*count)
// ....OR it's just data_length, thanks shae!
        #[br(count=data_length)]
        pub share_data: Vec<u8>,
    }
}
