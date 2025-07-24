pub mod lib {
    use binrw::*;
    use binrw::io::*;
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

    #[derive(BinRead, PartialEq, Debug)]
    pub struct Share {
	pub version: u32,
	pub block_size: u32,
	pub data_size: u32,

	pub data_offset: u32,
	pub plaintxt_hash_tree_offset: u32,
	pub crypttext_hash_tree_offset: u32,
	pub block_hashes_offset: u32,
	pub share_hashes_offset: u32,
	pub uri_ext_offset: u32,

	#[br(seek_before(SeekFrom::Start(uri_ext_offset as u64)))]
	pub uri_ext_size: u32,
	#[br(count=uri_ext_size)]
	pub uri_ext: Vec<u8>,

        #[br(
            seek_before(SeekFrom::Start(crypttext_hash_tree_offset as u64)),
            count=block_hashes_offset - crypttext_hash_tree_offset
        )]
        pub crypttext_hash_tree: Vec<u8>,
        // TODO: can we read _directly_ into a merkle_rs::Tree or whatever?
    }
}
