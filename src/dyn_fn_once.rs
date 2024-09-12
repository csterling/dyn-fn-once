use std::marker::PhantomData;
use std::mem::forget;

/// A dynamically-typed version of an [FnOnce]-closure (because [FnOnce] is not object-safe).
/// 
/// - `'capture` - the lifetime of the captured state of the closure.
/// - `Args` - the type of the arguments to the closure. Unfortunately the
///            [Tuple](core::marker::Tuple) trait is not stable, so if multiple arguments are
///            needed for a closure, they must be packed into a single tuple argument.
/// - `Return` - the type of the value returned from the closure.
pub struct DynFnOnce<
    'capture,
    Args,
    Return
> {
    /// Pointer to the source `impl FnOnce`, typed-erased
    ptr: *mut (),
    /// Function which knows how to call the source closure
    call_from_ptr: fn(ptr: *mut (), args: Args) -> Return,
    /// Function which knows how to drop the source closure
    drop: fn(ptr: *mut ()),
    /// Marker for the closure's lifetime, covariant as a closure with a longer lifetime
    /// can be treated as a closure with a shorter lifetime
    marker: PhantomData<&'capture ()>
}

impl<'capture, Args, Return> DynFnOnce<'capture, Args, Return> {
    /// Calls the closure.
    /// 
    /// - `args` - the arguments to the closure.
    pub fn call(self, args: Args) -> Return {
        let DynFnOnce {
            ptr,
            call_from_ptr,
            ..
        } = self;

        // Required to prevent double-free, as ptr/call_from_ptr are both Copy, so self is still
        // available here. call_from_ptr (below) handles freeing the closure.
        forget(self);

        call_from_ptr(ptr, args)
    }
}

impl<'capture, Args, Return> Drop for DynFnOnce<'capture, Args, Return> {
    fn drop(&mut self) {
        let DynFnOnce {
            ptr,
            drop,
            ..
        } = self;

        drop(*ptr)
    }
}

impl<
    'capture,
    Args,
    Return,
    F: 'capture + FnOnce(Args) -> Return
> From<F> for DynFnOnce<'capture, Args, Return> {
    fn from(value: F) -> Self {
        let ptr = Box::into_raw(Box::new(value)).cast::<()>();
        Self {
            ptr,
            call_from_ptr: |ptr, args| {
                let f = *unsafe { Box::<F>::from_raw(ptr.cast()) };
                f(args)
            },
            drop: |ptr| {
                drop(unsafe { Box::<F>::from_raw(ptr.cast()) })
            },
            marker: PhantomData
        }
    }
}
