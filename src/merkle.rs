#![allow(dead_code)]

use sha2::{Digest, Sha256};

pub fn calculate(hashes: Vec<Vec<u8>>) -> Vec<u8> {
    let mut hashes = hashes;
    while hashes.len() > 1 {
        hashes = hashes.chunks(2).map(|f| double_hash(f)).collect()
    }
    hashes.remove(0)
}

fn concat(data: &[Vec<u8>]) -> Vec<u8> {
    let mut init = data[0].to_vec();
    if data.len() == 1 {
        init.extend(data[0].iter());
    } else {
        init.extend(data[1].iter());
    }
    init
}

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn double_hash(data: &[Vec<u8>]) -> Vec<u8> {
    hash(&hash(&concat(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // bitcoin-cli getblock 0000000000013b8ab2cd513b0261a14096412195a72a0c4827d229dcc7e0f7af
    // https://mempool.space/block/0000000000013b8ab2cd513b0261a14096412195a72a0c4827d229dcc7e0f7af
    static TRX: [&'static str; 9] = [
        "ef1d870d24c85b89d92ad50f4631026f585d6a34e972eaf427475e5d60acf3a3",
        "f9fc751cb7dc372406a9f8d738d5e6f8f63bab71986a39cf36ee70ee17036d07",
        "db60fb93d736894ed0b86cb92548920a3fe8310dd19b0da7ad97e48725e1e12e",
        "220ebc64e21abece964927322cba69180ed853bb187fbc6923bac7d010b9d87a",
        "71b3dbaca67e9f9189dad3617138c19725ab541ef0b49c05a94913e9f28e3f4e",
        "fe305e1ed08212d76161d853222048eea1f34af42ea0e197896a269fbf8dc2e0",
        "21d2eb195736af2a40d42107e6abd59c97eb6cffd4a5a7a7709e86590ae61987",
        "dd1fd2a6fc16404faf339881a90adbde7f4f728691ac62e8f168809cdfae1053",
        "74d681e0e03bafa802c8aa084379aa98d9fcd632ddc2ed9782b586ec87451f20",
    ];
    static ROOT: &'static str = "2fda58e5959b0ee53c5253da9b9f3c0c739422ae04946966991cf55895287552";

    fn transactions() -> Vec<Vec<u8>> {
        TRX.map(|t| hex::decode(t).unwrap().into_iter().rev().collect())
            .to_vec()
    }

    fn root() -> Vec<u8> {
        hex::decode(ROOT).unwrap().into_iter().rev().collect()
    }

    #[test]
    fn test_calculate() {
        let mr = calculate(transactions());
        assert_eq!(mr, root());
    }

    #[test]
    fn test_calculate_with_one_transaction() {
        let trx = hex::decode("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098")
            .unwrap()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        let mr = calculate(vec![trx.clone()]);
        assert_eq!(trx, mr);
    }
}
