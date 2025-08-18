
use reth_primitives::B256;
use rsp_client_executor::{io::AggregationInput, ClientExecutor, EthereumVariant};

use risc0_zkvm::guest::env;

use std::alloc::{alloc, handle_alloc_error};
use std::{alloc::Layout, ffi::c_void};

#[no_mangle]
// TODO ideally this is c_size_t, but not stabilized (not guaranteed to be usize on all archs)
unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(size, 4).expect("unable to allocate more memory");
    let ptr = alloc(layout);

    if ptr.is_null() {
        handle_alloc_error(layout);
    }

    ptr as *mut c_void
}

#[no_mangle]
// TODO shouldn't need to zero allocated bytes since the zkvm memory is zeroed, might want to zero anyway
unsafe extern "C" fn calloc(nobj: usize, size: usize) -> *mut c_void {
    malloc(nobj * size)
}

#[no_mangle]
unsafe extern "C" fn free(_size: *const c_void) {
    // Intentionally a no-op, since the zkvm allocator is a bump allocator
}

pub fn main() {
    // Read the input.
    println!("cycle-tracker-start: deserialize");
    // Read the public values, vkey(image_id), and aggregation input.
    let public_values: Vec<Vec<u8>> = env::read();
    // let vkey = env::read::<[u32; 8]>();
    println!("cycle-tracker-start: deserialize aggregation input");
    let aggregation_input = env::read::<AggregationInput>();
    println!("cycle-tracker-end: deserialize aggregation input");

    let parent_state_root = env::read::<B256>();
    env::commit(&parent_state_root);
    env::commit(&aggregation_input.current_block);
    println!("cycle-tracker-end: deserialize");

    let client = ClientExecutor;

    let header = client
        .execute_aggregation::<EthereumVariant>(
            public_values,
            // vkey,
            aggregation_input,
            parent_state_root,
        )
        .expect("failed to execute aggregation");

    let hash = header.hash_slow();

    env::commit(&hash);
}
