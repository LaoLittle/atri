#[cfg(feature = "tokio")]
pub mod tokio {
    use crate::executor::Executor;
    use std::future::Future;
    pub use tokio::runtime::Runtime;

    impl Executor for Runtime {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            (*self).spawn(fu);
        }
    }
}

#[cfg(feature = "async-std")]
pub mod async_std {
    use crate::executor::Executor;
    use std::future::Future;

    pub struct Runtime;

    impl Executor for Runtime {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            async_std::task::spawn(fu);
        }
    }
}

#[cfg(feature = "smol")]
pub mod smol {
    use crate::executor::Executor;
    use std::future::Future;

    pub struct Runtime;

    impl Executor for Runtime {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            smol::spawn(fu).detach();
        }
    }

    impl Executor for smol::Executor<'_> {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            (*self).spawn(fu).detach();
        }
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use crate::executor::Executor;
    use std::future::Future;
    use std::thread;

    pub struct Runtime;

    impl Executor for Runtime {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            thread::spawn(move || {
                futures::executor::block_on(fu);
            });
        }
    }
}

#[cfg(feature = "thread-pool")]
pub mod thread_pool {
    use crate::executor::Executor;
    use futures::executor::ThreadPool;
    use std::future::Future;

    impl Executor for ThreadPool {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            let fu = async {
                fu.await;
            };
            (*self).spawn_ok(fu);
        }
    }
}
