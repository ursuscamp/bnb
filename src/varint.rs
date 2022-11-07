use nom::{
    bytes::complete::take,
    error::{make_error, ErrorKind},
    number::{
        complete::{le_u16, le_u64},
        streaming::le_u32,
    },
    IResult,
};

#[allow(dead_code)]
pub fn encode(num: u64) -> Vec<u8> {
    let mut output = Vec::with_capacity(9);
    if num <= 0xfc {
        let sl = (num as u8).to_le_bytes();
        output.extend_from_slice(&sl);
    } else if num <= 0xffff {
        output.push(0xfd);
        let sl = (num as u16).to_le_bytes();
        output.extend_from_slice(&sl);
    } else if num <= 0xffffffff {
        output.push(0xfe);
        let sl = (num as u32).to_le_bytes();
        output.extend_from_slice(&sl);
    } else {
        output.push(0xff);
        let sl = num.to_le_bytes();
        output.extend_from_slice(&sl);
    }
    output
}

#[allow(dead_code)]
pub fn decode(bytes: &[u8]) -> IResult<&[u8], u64> {
    nom::branch::alt((decode_1b, decode_2b, decode_4b, decode_8b))(bytes)
}

fn get_tag(bytes: &[u8]) -> IResult<&[u8], u8> {
    let (bytes, t) = take(1usize)(bytes)?;
    Ok((bytes, t[0]))
}

fn decode_1b(bytes: &[u8]) -> IResult<&[u8], u64> {
    let (bytes, t) = get_tag(bytes)?;
    if t <= 0xfc {
        return Ok((bytes, t as u64));
    }
    Err(nom::Err::Error(make_error(bytes, ErrorKind::Tag)))
}

fn decode_2b(bytes: &[u8]) -> IResult<&[u8], u64> {
    let (bytes, t) = get_tag(bytes)?;
    if t == 0xfd {
        return le_u16(bytes).map(|(i, o)| (i, o as u64));
    }
    Err(nom::Err::Error(make_error(bytes, ErrorKind::LengthValue)))
}

fn decode_4b(bytes: &[u8]) -> IResult<&[u8], u64> {
    let (bytes, t) = get_tag(bytes)?;
    if t == 0xfe {
        return le_u32(bytes).map(|(i, o)| (i, o as u64));
    }
    Err(nom::Err::Error(make_error(bytes, ErrorKind::LengthValue)))
}

fn decode_8b(bytes: &[u8]) -> IResult<&[u8], u64> {
    let (bytes, t) = get_tag(bytes)?;
    if t == 0xff {
        return le_u64(bytes);
    }
    Err(nom::Err::Error(make_error(bytes, ErrorKind::LengthValue)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(0xfc), vec![0xfc]);
        assert_eq!(encode(0xfd), vec![0xfd, 0xfd, 0x00]);
        assert_eq!(encode(0x1234), vec![0xfd, 0x34, 0x12]);
        assert_eq!(encode(0x0226), vec![0xfd, 0x26, 0x02]);
        assert_eq!(encode(0x000f3a70), vec![0xfe, 0x70, 0x3a, 0x0f, 0x00]);
        assert_eq!(
            encode(0xfffffffffffffffe),
            vec![0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,]
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(&[0xfc]).unwrap().1, 0xfc);
        assert_eq!(decode(&[0xfd, 0xfd, 0x00]).unwrap().1, 0xfd);
        assert_eq!(decode(&[0xfd, 0x34, 0x12]).unwrap().1, 0x1234);
        assert_eq!(decode(&[0xfd, 0x26, 0x02]).unwrap().1, 0x0226);
        assert_eq!(
            decode(&[0xfe, 0x70, 0x3a, 0x0f, 0x00]).unwrap().1,
            0x000f3a70
        );
        assert_eq!(
            decode(&[0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]).unwrap(),
            (vec![0x01_u8].as_ref(), 0xfffffffffffffffe_u64)
        );
        assert!(decode(&[0xff, 0xfe]).is_err());
    }
}
