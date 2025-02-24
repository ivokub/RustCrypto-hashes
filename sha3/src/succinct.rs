use crate::state::{DEFAULT_ROUND_COUNT, PLEN};
use core::convert::TryInto;

#[inline]
pub(crate) fn keccak_permute(state: &mut [u64; PLEN]) {
    sp1_zkvm::lib::unconstrained!({
        // we cannot modify the memory in the unconstrained mode. We first need to copy the state
        // mutate it and then read it back from the hint
        let mut state_copy = state.clone();
        keccak::p1600(&mut state_copy, DEFAULT_ROUND_COUNT);
        // for some reason the automatic serialized doesn't work well. Manually serialize into bytes
        let mut state_bytes = [0u8; PLEN * 8];
        for (chunk, &value) in state_bytes.chunks_mut(8).zip(state_copy.iter()) {
            chunk.copy_from_slice(&value.to_le_bytes());
        }
        sp1_zkvm::io::hint_slice(&state_bytes);
    });
    // read the result from the hint and set the state
    let mutated_state_bytes = sp1_zkvm::io::read_vec();
    let mut mutated_state = [0u64; PLEN];
    for (chunk, value) in mutated_state_bytes
        .chunks_exact(8)
        .zip(mutated_state.iter_mut())
    {
        *value = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    state.copy_from_slice(&mutated_state);
}
