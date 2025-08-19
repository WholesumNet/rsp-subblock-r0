
use rsp_client_executor::{
    io::{/*read_aligned_vec,*/ SubblockInput},
    ClientExecutor, EthereumVariant,
};
use rsp_mpt::EthereumState;

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
    println!("cycle-tracker-start: deserialize input");
    let input: SubblockInput = env::read();
    println!("cycle-tracker-end: deserialize input");

    println!("cycle-tracker-start: commit input");
    env::commit(&input);
    println!("cycle-tracker-end: commit input");

    println!("cycle-tracker-start: deserialize parent state");

    let aligned: Vec<Vec<u8>> = env::read();//read_aligned_vec::<16>();
    let mut parent_state =
        rkyv::from_bytes::<EthereumState, rkyv::rancor::BoxedError>(&aligned.concat()).unwrap();

    println!("cycle-tracker-end: deserialize parent state");

    println!("cycle-tracker-start: execute subblock");
    // Execute the block.
    let executor = ClientExecutor;
    let subblock_output = executor
        .execute_subblock::<EthereumVariant>(input, &mut parent_state)
        .expect("failed to execute client");
    println!("cycle-tracker-end: execute subblock");

    // Commit the state diff.
    env::commit(&subblock_output);
}
