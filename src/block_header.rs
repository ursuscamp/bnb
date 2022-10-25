#![allow(dead_code, unused_imports)]

use nom::{
    bytes::complete::take,
    combinator::{map, map_res},
    number::{complete::le_i32, streaming::le_u32},
    IResult,
};

/// https://en.bitcoin.it/wiki/Protocol_documentation#Block_Headers
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHeader {
    version: i32,
    prev_block: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    bits: u32,
    nonce: u32,
}

fn parse_block_header(input: &[u8]) -> IResult<&[u8], BlockHeader> {
    let (input, version) = parse_i32(input)?;
    let (input, prev_block) = parse_bytes(32, input)?;
    let (input, merkle_root) = parse_bytes(32, input)?;
    let (input, timestamp) = parse_u32(input)?;
    let (input, bits) = parse_u32(input)?;
    let (input, nonce) = parse_u32(input)?;
    Ok((
        input,
        BlockHeader {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
        },
    ))
}

fn parse_i32(input: &[u8]) -> IResult<&[u8], i32> {
    le_i32(input)
}

fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    le_u32(input)
}

fn parse_bytes(count: usize, input: &[u8]) -> IResult<&[u8], [u8; 32]> {
    map_res(take(count), |pb: &[u8]| pb.try_into())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://mempool.space/block/00000000000000000006ff82442b971512e1d7a5865c323a169ae8e238181430
    // bitcoin-cli getblock "00000000000000000006ff82442b971512e1d7a5865c323a169ae8e238181430"
    static HEADER: &'static str = "04000020cc58a01f0c48627419b089b5020b658dcc1fd0e3c1a0000000000000000000009959dac9f575d922c975f275932f69342f921adc017159eef78fcd54184e1e7e9d8c566329a40717c5d73b67";
    static VERSION: i32 = 0x20000004;
    static PREV_BLOCK: &'static str =
        "00000000000000000000a0c1e3d01fcc8d650b02b589b0197462480c1fa058cc";
    static MERKLE_ROOT: &'static str =
        "7e1e4e1854cd8ff7ee597101dc1a922f34692f9375f275c922d975f5c9da5999";
    static TIMESTAMP: u32 = 0x63568C9D;
    static BITS: u32 = 0x1707a429;
    static NONCE: u32 = 0x673BD7C5;

    fn hb() -> Vec<u8> {
        hex::decode(HEADER).unwrap()
    }

    fn pbh() -> [u8; 32] {
        let mut h: [u8; 32] = hex::decode(PREV_BLOCK).unwrap().try_into().unwrap();
        h.reverse(); // Almost everything is in little endian so let's reverse the byte order of the hash
        h
    }

    fn mr() -> [u8; 32] {
        let mut m: [u8; 32] = hex::decode(MERKLE_ROOT).unwrap().try_into().unwrap();
        m.reverse();
        m
    }

    #[test]
    fn test_parse_version() {
        let hb = hb();
        let res = parse_i32(&hb);
        assert_eq!(res, Ok((&hb[4..], VERSION)));
    }

    #[test]
    fn test_parse_prev_block() {
        let hb = hb();
        let res = parse_bytes(32, &hb[4..]);
        assert_eq!(res, Ok((&hb[36..], pbh())));
    }

    #[test]
    fn test_parse_merkle_root() {
        let hb = hb();
        let res = parse_bytes(32, &hb[36..]);
        assert_eq!(res, Ok((&hb[68..], mr())));
    }

    #[test]
    fn test_parse_timestamp() {
        let hb = hb();
        let res = parse_u32(&hb[68..]);
        assert_eq!(res, Ok((&hb[72..], TIMESTAMP)));
    }

    #[test]
    fn test_parse_bits() {
        let hb = hb();
        let res = parse_u32(&hb[72..]);
        assert_eq!(res, Ok((&hb[76..], BITS)));
    }

    #[test]
    fn test_parse_nonce() {
        let hb = hb();
        let res = parse_u32(&hb[76..]);
        assert_eq!(res, Ok((&hb[80..], NONCE)));
        assert!(&hb[80..].is_empty());
    }

    #[test]
    fn test_parse_block_header() {
        let hb = hb();
        let (input, bh) = parse_block_header(&hb).unwrap();
        assert!(input.is_empty());
        assert_eq!(
            bh,
            BlockHeader {
                version: VERSION,
                prev_block: pbh(),
                merkle_root: mr(),
                timestamp: TIMESTAMP,
                bits: BITS,
                nonce: NONCE
            }
        )
    }
}
