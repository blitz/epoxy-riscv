//! # Vector Utilities

/// Turn a vector of [u32] into its byte representation.
pub fn vec_u32_to_bytes(v: &[u32]) -> Vec<u8> {
    v.iter().fold(vec![], |mut acc, &v| {
        acc.extend_from_slice(&v.to_le_bytes());
        acc
    })
}

/// Turn a vector of [u64] into its byte representation.
pub fn vec_u64_to_bytes(v: &[u64]) -> Vec<u8> {
    v.iter().fold(vec![], |mut acc, &v| {
        acc.extend_from_slice(&v.to_le_bytes());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_u32_works() {
        assert_eq!(vec_u32_to_bytes(&[]), vec![]);
        assert_eq!(
            vec_u32_to_bytes(&[0x76543210, 0xcafed00d]),
            vec![0x10, 0x32, 0x54, 0x76, 0x0d, 0xd0, 0xfe, 0xca]
        );
    }

    #[test]
    fn combine_u64_works() {
        assert_eq!(vec_u64_to_bytes(&[]), vec![]);
        assert_eq!(
            vec_u64_to_bytes(&[0xcafed00d76543210]),
            vec![0x10, 0x32, 0x54, 0x76, 0x0d, 0xd0, 0xfe, 0xca]
        );
    }
}
