
use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

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

fn main() {
    // Read the input.
    let input: Vec<u8> = env::read();
    let input = bincode::deserialize::<ClientExecutorInput>(&input).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    env::commit(&block_hash);
}
