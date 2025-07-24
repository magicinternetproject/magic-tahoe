pub mod lib {
    use binrw::*;
    use std::fmt::Debug;

    #[derive(BinRead, PartialEq, Debug)]
    pub struct Lease {
	pub version: u32,
	pub data_length: u32,
	pub count: u32,

//        #[br(count=
//        pub share_data: Vec<u8>,
    }
}
