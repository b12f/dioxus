use dioxus_core::{ScopeState, TaskId};
use std::future::Future;
use std::{cell::Cell, rc::Rc};
/*



let g = use_coroutine(&cx, || {
    // clone the items in
    async move {

    }
})



*/
pub fn use_coroutine<F>(cx: &ScopeState, create_future: impl FnOnce() -> F) -> CoroutineHandle<'_>
where
    F: Future<Output = ()> + 'static,
{
    let state = cx.use_hook(move |_| {
        let f = create_future();
        let id = cx.push_future(f);
        State {
                running: Default::default(),
                _id: id
                // pending_fut: Default::default(),
                // running_fut: Default::default(),
            }
    });

    // state.pending_fut.set(Some(Box::pin(f)));

    // if let Some(fut) = state.running_fut.as_mut() {
    //     cx.push_future(fut);
    // }

    // if let Some(fut) = state.running_fut.take() {
    // state.running.set(true);
    // fut.resume();
    // }

    // let submit: Box<dyn FnOnce() + 'a> = Box::new(move || {
    //     let g = async move {
    //         running.set(true);
    //         create_future().await;
    //         running.set(false);
    //     };
    //     let p: Pin<Box<dyn Future<Output = ()>>> = Box::pin(g);
    //     fut_slot
    //         .borrow_mut()
    //         .replace(unsafe { std::mem::transmute(p) });
    // });

    // let submit = unsafe { std::mem::transmute(submit) };
    // state.submit.get_mut().replace(submit);

    // if state.running.get() {
    //     // let mut fut = state.fut.borrow_mut();
    //     // cx.push_task(|| fut.as_mut().unwrap().as_mut());
    // } else {
    //     // make sure to drop the old future
    //     if let Some(fut) = state.fut.borrow_mut().take() {
    //         drop(fut);
    //     }
    // }
    CoroutineHandle { cx, inner: state }
}

struct State {
    running: Rc<Cell<bool>>,
    _id: TaskId,
    // the way this is structure, you can toggle the coroutine without re-rendering the comppnent
    // this means every render *generates* the future, which is a bit of a waste
    // todo: allocate pending futures in the bump allocator and then have a true promotion
    // pending_fut: Cell<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>,
    // running_fut: Option<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    // running_fut: Rc<RefCell<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>>
}

pub struct CoroutineHandle<'a> {
    cx: &'a ScopeState,
    inner: &'a State,
}

impl Clone for CoroutineHandle<'_> {
    fn clone(&self) -> Self {
        CoroutineHandle {
            cx: self.cx,
            inner: self.inner,
        }
    }
}
impl Copy for CoroutineHandle<'_> {}

impl<'a> CoroutineHandle<'a> {
    #[allow(clippy::needless_return)]
    pub fn start(&self) {
        if self.is_running() {
            return;
        }

        // if let Some(submit) = self.inner.pending_fut.take() {
        // submit();
        // let inner = self.inner;
        // self.cx.push_task(submit());
        // }
    }

    pub fn is_running(&self) -> bool {
        self.inner.running.get()
    }

    pub fn resume(&self) {
        // self.cx.push_task(fut)
    }

    pub fn stop(&self) {}

    pub fn restart(&self) {}
}
