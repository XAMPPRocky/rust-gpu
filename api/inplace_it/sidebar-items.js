initSidebarItems({"fn":[["alloc_array","`alloc_array` is used when `inplace_or_alloc_array` realize that the size of requested array of `T` is too large and should be replaced in the heap."],["inplace_or_alloc_array","`inplace_or_alloc_array` is a central function of this crate. It's trying to place an array of `T` on the stack and pass the guard of memory into the `consumer` closure. `consumer`'s result will be returned."],["try_inplace_array","`try_inplace_array` trying to place an array of `T` on the stack and pass the guard of memory into the `consumer` closure. `consumer`'s result will be returned as `Ok(result)`."]],"struct":[["SliceMemoryGuard","Guard-struct used for correctly initialize uninitialized memory and `drop` it when guard goes out of scope. Usually, you should not use this struct to handle your memory."],["UninitializedSliceMemoryGuard","Guard-struct used to own uninitialized memory and provide functions for initializing it. Usually, you should not use this struct to handle your memory."]],"trait":[["FixedArray","This trait is a extended copy of unstable core::array::FixedSizeArray."]]});