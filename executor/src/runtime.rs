use super::Executor;

#[cfg(feature = "tokio")]
pub mod tokio {
    use std::future::Future;
    pub use tokio::runtime::Runtime;

    impl super::Executor for Runtime {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            self.spawn(fu);
        }
    }
}

#[cfg(feature = "async-std")]
pub mod async_std {
    use std::future::Future;

    pub struct Runtime;

    impl super::Executor for Runtime {
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

#[cfg(feature = "blocking")]
pub mod blocking {
    use std::future::Future;
    use std::thread;

    pub struct Runtime;

    impl super::Executor for Runtime {
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
    use futures::executor::ThreadPool;
    use std::future::Future;

    impl super::Executor for ThreadPool {
        fn spawn<F>(&self, fu: F)
        where
            F: Future + Send,
            F: 'static,
            F::Output: Send + 'static,
        {
            let fu = async {
                fu.await;
            };
            self.spawn_ok(fu);
        }
    }
}
