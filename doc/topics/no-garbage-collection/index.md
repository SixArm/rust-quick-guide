# No garbage collection

When a program creates objects or data in memory, the program must manage the memory. Some languages such as C rely on the developer to allocate memory and free it. Some languages such as Java use garbage collection. Rust has a unique approach that uses no garbage collection.

## What is garbage collection?

Garbage collection (GC) is a mechanism that automatically frees up memory that is no longer being used.

Garbage collection works by periodically scanning the memory used by a program to identify objects that are no longer being used. Once identified, the garbage collector frees up the memory used by these objects, making it available for future use by the program.

There are different types of garbage collection algorithms, such as reference counting, mark and sweep, and copying. Each algorithm has its strengths and weaknesses, depending on context.


## Rust doesn't use garbage collection

One of Rust’s key innovations is guaranteeing memory safety (meaning no segfaults) without requiring garbage collection. Rust avoids GC by tracking memory ownership and enforcing safety via the borrow checker.

By avoiding GC, Rust can offer numerous benefits: predictable cleanup of resources, lower overhead for memory management, and essentially no runtime system. These benefits make it easier to embed Rust into arbitrary contexts, and also easier to integrate Rust with languages that do have a GC.

For when single ownership does not suffice, Rust programs can use the standard library reference-counting smart pointer types: `Rc` for single-thread reference counting, and Arc for multi-thread reference counting.

