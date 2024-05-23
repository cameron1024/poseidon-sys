/// A 256-bit unsigned integer in little-endian order
pub type U256 = [u8; 32];

extern "C" {
    fn poseidon_hash_impl(ptr: *const u8, len: usize) -> *mut u8;

    fn free_ptr(ptr: *mut u8);
}

pub fn poseidon_hash(nums: &[U256]) -> U256 {
    match nums.len() {
        0 => panic!("at least one u256 must be provided"),
        7.. => panic!("no more than 6 256s must be provided"),
        _ => {}
    }

    let bytes = bytemuck::cast_slice::<U256, u8>(nums);

    let ptr = bytes.as_ptr();
    let len = bytes.len();

    let ptr = unsafe { poseidon_hash_impl(ptr, len) };
    let u256 = unsafe { u256_from_ptr(ptr) };
    unsafe { free_ptr(ptr) };

    u256
}

unsafe fn u256_from_ptr(ptr: *mut u8) -> U256 {
    let len = 32;
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    let slice = bytemuck::cast_slice::<u8, U256>(slice);
    slice[0]
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use super::*;

    const fn u256(x: u8) -> U256 {
        let mut result = [0; 32];
        result[31] = x;
        result
    }

    #[test]
    fn simple_example() {
        let result = poseidon_hash(&[u256(0), u256(1)]);
        println!("{result:?}");
    }

    #[proptest]
    fn can_hash_any_pair_of_elements(a: U256, b: U256) {
        let _ = poseidon_hash(&[a, b]);
    }
}
