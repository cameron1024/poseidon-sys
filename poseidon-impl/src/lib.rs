use ff::PrimeField;
use poseidon_base::primitives::{ConstantLength, Hash, P128Pow5T3};

type U256 = [u8; 32];
type Base = poseidon_base::primitives::bn256::Fp;

/// # Safety
///
/// This function must only be called via FFI
#[no_mangle]
pub unsafe extern "C" fn poseidon_hash_impl(ptr: *const u8, len: usize) -> *mut u8 {
    // SAFETY: this function must be called with a valid ptr/len combo that was obtained from
    // a safely constructed slice
    let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
    let elements = convert_slice(bytes);

    let result = match elements.len() {
        1 => hash::<1>(elements.try_into().unwrap()),
        _ => [0; 32],
    };

    Box::into_raw(Box::new(result)) as *mut u8
}


/// Create a Box from the pointer and immediately drop it
///
/// # Safety
///
/// This function requires all invariants of [`Box::from_raw`]
#[no_mangle]
pub unsafe extern "C" fn free_ptr(ptr: *mut u8) {
    let _ = unsafe { Box::from_raw(ptr) };
}

fn convert_slice(bytes: &[u8]) -> &[U256] {
    bytemuck::cast_slice(bytes)
}

fn hash<const N: usize>(elements: [U256; N]) -> U256 {
    type H<const N: usize> = Hash<Base, P128Pow5T3<Base>, ConstantLength<N>, 3, 2>;

    H::<N>::init().hash(elements.map(u256_to_base)).to_repr()
}

fn u256_to_base(u: U256) -> Base {
    Base::from_raw(u8s_to_u64(u))
}

fn u8s_to_u64(u8s: [u8; 32]) -> [u64; 4] {
    [
        u64::from_le_bytes((&u8s[0..8]).try_into().unwrap()),
        u64::from_le_bytes((&u8s[8..16]).try_into().unwrap()),
        u64::from_le_bytes((&u8s[16..24]).try_into().unwrap()),
        u64::from_le_bytes((&u8s[24..32]).try_into().unwrap()),
    ]
}
