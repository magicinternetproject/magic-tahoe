pub mod lib {
    use binrw::*;
    use std::fmt::Debug;

    #[derive(BinRead, PartialEq, Debug)]
    pub struct Share {
	pub lease_version: u32,
	pub lease_data_length: u32,
	pub lease_count: u32,

//        #[br(count=
//        pub share_data: Vec<u8>,
    }
}
