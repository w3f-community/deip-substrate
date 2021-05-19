initSidebarItems({"constant":[["ONCE_INIT","Initialization value for static [`Once`] values."]],"enum":[["TryLockError","An enumeration of possible errors associated with a [`TryLockResult`] which can occur while trying to acquire a lock, from the `try_lock` method on a [`Mutex`] or the `try_read` and `try_write` methods on an [`RwLock`]."]],"mod":[["atomic","Atomic types"],["mpsc","Multi-producer, single-consumer FIFO queue communication primitives."]],"struct":[["Arc","A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’."],["Barrier","A barrier enables multiple threads to synchronize the beginning of some computation."],["BarrierWaitResult","A `BarrierWaitResult` is returned by [`Barrier::wait()`] when all threads in the [`Barrier`] have rendezvoused."],["Condvar","A Condition Variable"],["Mutex","A mutual exclusion primitive useful for protecting shared data"],["MutexGuard","An RAII implementation of a “scoped lock” of a mutex. When this structure is dropped (falls out of scope), the lock will be unlocked."],["Once","A synchronization primitive which can be used to run a one-time global initialization. Useful for one-time initialization for FFI or related functionality. This type can only be constructed with [`Once::new()`]."],["OnceState","State yielded to [`Once::call_once_force()`]’s closure parameter. The state can be used to query the poison status of the [`Once`]."],["PoisonError","A type of error which can be returned whenever a lock is acquired."],["RwLock","A reader-writer lock"],["RwLockReadGuard","RAII structure used to release the shared read access of a lock when dropped."],["RwLockWriteGuard","RAII structure used to release the exclusive write access of a lock when dropped."],["WaitTimeoutResult","A type indicating whether a timed wait on a condition variable returned due to a time out or not."],["Weak","`Weak` is a version of [`Arc`] that holds a non-owning reference to the managed allocation. The allocation is accessed by calling `upgrade` on the `Weak` pointer, which returns an [`Option`]`<`[`Arc`]`<T>>`."]],"type":[["LockResult","A type alias for the result of a lock method which can be poisoned."],["TryLockResult","A type alias for the result of a nonblocking locking method."]]});