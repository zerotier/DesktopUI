use std::sync::{Arc, Mutex, Condvar};

use ::{
    RtType,
    ComIid,
    Result
};

use ::windows::foundation::{
    IAsyncInfo,
    IAsyncAction,
    AsyncActionCompletedHandler,
    IAsyncActionWithProgress,
    AsyncActionWithProgressCompletedHandler,
    IAsyncOperation,
    AsyncOperationCompletedHandler,
    IAsyncOperationWithProgress,
    AsyncOperationWithProgressCompletedHandler
};

pub trait RtAsyncAction {
    fn blocking_wait(&self);
}

pub trait RtAsyncOperation: RtAsyncAction {
    type TResult;
    
    fn get_results(&self) -> Result<Self::TResult>;

    #[inline]
    fn blocking_get(&self) -> Result<Self::TResult> {
        self.blocking_wait();
        self.get_results()
    }
}

// The handler type is different for each interface, and the easiest way to share code seems to be a macro
macro_rules! impl_blocking_wait {
    ($handler:ident) => {
        #[inline]
        fn blocking_wait(&self) {
            let info = ::comptr::query_interface::<_, IAsyncInfo>(self).expect("query_interface failed");
            let status = unsafe { info.get_status().expect("get_status failed") };

            if status == ::langcompat::ASYNC_STATUS_COMPLETED {
                return;
            }
            
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            {
                let pair2 = pair.clone();
                let handler = $handler::new(move |_op, _status| {
                    let &(ref lock, ref cvar) = &*pair2;
                    let mut completed = lock.lock().expect("lock failed");
                    *completed = true;
                    cvar.notify_one();
                    Ok(())
                });
                unsafe { self.set_completed(&handler).expect("set_completed failed") };
                // local reference to `handler` is dropped here -> Release() is called
            }
            
            // use condvar to wait until handler has been called
            let &(ref lock, ref cvar) = &*pair;
            let mut completed = lock.lock().expect("lock failed");
            while !*completed {
                completed = cvar.wait(completed).expect("wait failed");
            }
        }
    }
}

impl RtAsyncAction for IAsyncAction
{
    impl_blocking_wait!{ AsyncActionCompletedHandler }
}

impl<P: RtType + 'static> RtAsyncAction for IAsyncActionWithProgress<P>
    where AsyncActionWithProgressCompletedHandler<P>: ComIid
{
    impl_blocking_wait!{ AsyncActionWithProgressCompletedHandler }
}

impl<T: RtType + 'static> RtAsyncAction for IAsyncOperation<T>
    where AsyncOperationCompletedHandler<T>: ComIid
{
    impl_blocking_wait!{ AsyncOperationCompletedHandler }
}

impl<T: RtType + 'static> RtAsyncOperation for IAsyncOperation<T>
    where AsyncOperationCompletedHandler<T>: ComIid
{
    type TResult = <T as RtType>::Out;

    #[inline]
    fn get_results(&self) -> Result<Self::TResult> {
        unsafe { self.get_results() }
    }
}

impl<T: RtType + 'static, P: RtType + 'static> RtAsyncAction for IAsyncOperationWithProgress<T, P>
    where AsyncOperationWithProgressCompletedHandler<T, P>: ComIid
{
    impl_blocking_wait!{ AsyncOperationWithProgressCompletedHandler }
}

impl<T: RtType + 'static, P: RtType + 'static> RtAsyncOperation for IAsyncOperationWithProgress<T, P>
    where AsyncOperationWithProgressCompletedHandler<T, P>: ComIid
{
    type TResult = <T as RtType>::Out;

    #[inline]
    fn get_results(&self) -> Result<Self::TResult> {
        unsafe { self.get_results() }
    }
}