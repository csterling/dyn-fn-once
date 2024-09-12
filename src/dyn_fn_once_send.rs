use crate::DynFnOnce;

/// A [DynFnOnce] with the added precondition that the internal closure is Send.
pub struct DynFnOnceSend<'capture, Args, Return> {
    /// The internal type-erased closure.
    inner: DynFnOnce<'capture, Args, Return>
}

impl<'capture, Args, Return> DynFnOnceSend<'capture, Args, Return> {
    /// Calls the closure.
    ///
    /// - `args` - the arguments to the closure.
    pub fn call(self, args: Args) -> Return {
        self.inner.call(args)
    }
}

unsafe impl<'capture, Args, Return> Send for DynFnOnceSend<'capture, Args, Return> {}

impl<
    'capture,
    Args,
    Return,
    F: 'capture + Send + FnOnce(Args) -> Return
> From<F> for DynFnOnceSend<'capture, Args, Return> {
    fn from(value: F) -> Self {
        Self {
            inner: value.into()
        }
    }
}
