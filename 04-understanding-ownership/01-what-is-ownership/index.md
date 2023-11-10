# What is Ownership?

## Stack and Heap

### Stack

- LIFO (Last In First Out)
- pushing onto the stack, and popping off the stack
- All data stored on the stack must have a known, fixed size
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead

### Heap

- The heap is less organized: when you put data on the heap, you request a certain amount of space
- The memory allocator finds empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location
  - This is called "allocating on the heap"
- Pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer

> When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack.
> When the function is over, those values get popped off the stack.

## Ownership Rules

### Key Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

<!-- See ownership project -->