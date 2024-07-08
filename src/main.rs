use allocator::Allocator;

mod allocator;
mod memory;

fn main() {
    println!("Hello, world!");

    let mut mem_allocator: Allocator = Allocator::new();

    match mem_allocator.malloc(1024 * 1024 / 3 * 2 + 1 + 1) {
        Some(ptr) => println!("Allocated 1024 bytes at {:?}", ptr),
        None => println!("Failed to allocate memory"),
    }
}

// test cases for allocating 1024 * 1024 / 3 * 2 + 1 (will work) and 1024 * 1024 / 3 * 2 + 2 (which won't work)