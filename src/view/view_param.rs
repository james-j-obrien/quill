use std::sync::{Arc, Mutex};

use crate::node_span::NodeSpan;
use crate::{View, ViewContext};

/// A wrapper view around a view which makes it possible to pass a view as a parameter to
/// other views.
///
/// Currently, this will cause the view to which this parameter is passed to unconditionally
/// render whenever it's parent renders, because we don't do proper equality comparisons.
pub struct ViewParam<V: View> {
    inner: Arc<Mutex<V>>,
}

impl<V: View> ViewParam<V> {
    /// Construct a new ViewParam that references the given View.
    pub fn new(inner: V) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

impl<V: View> View for ViewParam<V> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.lock().unwrap().nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        self.inner.lock().unwrap().build(vc)
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.lock().unwrap().update(vc, state);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.lock().unwrap().assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.lock().unwrap().raze(vc, state);
    }
}

impl<V: View> PartialEq for ViewParam<V> {
    fn eq(&self, other: &Self) -> bool {
        // For now, we're just comparing pointers. However, we should probably do better.
        &*self.inner.lock().unwrap() as *const _ == &*other.inner.lock().unwrap() as *const _
    }
}

impl<V: View> Clone for ViewParam<V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
