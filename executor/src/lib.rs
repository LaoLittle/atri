pub mod runtime;

use std::future::Future;

pub trait Executor {
    fn spawn<F>(&self, fu: F)
    where
        F: Future + Send,
        F: 'static,
        F::Output: Send + 'static;
}

impl<E: Executor> Executor for std::rc::Rc<E> {
    fn spawn<F>(&self, fu: F)
    where
        F: Future + Send,
        F: 'static,
        F::Output: Send + 'static,
    {
        (**self).spawn(fu)
    }
}

impl<E: Executor> Executor for std::sync::Arc<E> {
    fn spawn<F>(&self, fu: F)
    where
        F: Future + Send,
        F: 'static,
        F::Output: Send + 'static,
    {
        (**self).spawn(fu)
    }
}
