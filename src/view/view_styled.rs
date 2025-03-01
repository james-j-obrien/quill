use crate::node_span::NodeSpan;
use crate::{ElementClasses, StyleHandle, StyleRef, View, ViewContext};
use bevy::prelude::*;
use bevy::utils::HashSet;
use impl_trait_for_tuples::*;

/// List of style objects which are attached to a given UiNode.
#[derive(Component, Default)]
pub struct ElementStyles {
    /// The collection of styles associated with this element.
    pub styles: Vec<StyleHandle>,

    // How far up the hierarchy the selectors need to search
    pub(crate) selector_depth: usize,

    // Whether any selectors use the :hover pseudo-class
    pub(crate) uses_hover: bool,
    // TODO: Inherited
}

// A wrapper view which applies styles to the output of an inner view.
pub struct ViewStyled<V: View> {
    inner: V,
    styles: Vec<StyleHandle>,
}

impl<V: View> ViewStyled<V> {
    pub fn new<S: StyleTuple>(inner: V, items: S) -> Self {
        Self {
            inner,
            styles: items.to_vec(),
        }
    }

    fn insert_styles(&self, nodes: &NodeSpan, vc: &mut ViewContext) {
        match nodes {
            NodeSpan::Empty => (),
            NodeSpan::Node(entity) => {
                let em = &mut vc.entity_mut(*entity);
                let selector_depth = self.styles.iter().map(|s| s.depth()).max().unwrap_or(0);
                let uses_hover = self
                    .styles
                    .iter()
                    .map(|s| s.uses_hover())
                    .max()
                    .unwrap_or(false);

                match em.get_mut::<ElementStyles>() {
                    Some(mut sc) => {
                        sc.styles.clone_from(&self.styles);
                        sc.selector_depth = selector_depth;
                        sc.uses_hover = uses_hover;
                    }
                    None => {
                        em.insert((ElementStyles {
                            styles: self.styles.clone(),
                            selector_depth,
                            uses_hover,
                        },));
                    }
                }

                if em.get_mut::<ElementClasses>().is_none() {
                    em.insert((ElementClasses(HashSet::new()),));
                }
            }

            NodeSpan::Fragment(ref nodes) => {
                for node in nodes.iter() {
                    // Recurse
                    self.insert_styles(node, vc);
                }
            }
        }
    }
}

impl<V: View> View for ViewStyled<V> {
    type State = V::State;

    fn nodes(&self, vc: &ViewContext, state: &Self::State) -> NodeSpan {
        self.inner.nodes(vc, state)
    }

    fn build(&self, vc: &mut ViewContext) -> Self::State {
        let state = self.inner.build(vc);
        self.insert_styles(&self.nodes(vc, &state), vc);
        state
    }

    fn update(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.update(vc, state);
        self.insert_styles(&mut self.nodes(vc, state), vc);
    }

    fn assemble(&self, vc: &mut ViewContext, state: &mut Self::State) -> NodeSpan {
        self.inner.assemble(vc, state)
    }

    fn raze(&self, vc: &mut ViewContext, state: &mut Self::State) {
        self.inner.raze(vc, state);
    }

    // Apply styles to this view.
    // TODO: Possible optimization by replacing the style object rather than wrapping it.
    // fn styled<S: StyleTuple<'a>>(&self, styles: S) -> StyledView<'a, Self> {
    //     StyledView::<'a, Self>::new(&self, styles)
    // }
}

// StyleTuple - a variable-length tuple of styles.

pub trait StyleTuple: Send {
    fn to_vec(self) -> Vec<StyleHandle>;
}

impl StyleTuple for () {
    fn to_vec(self) -> Vec<StyleHandle> {
        Vec::new()
    }
}

impl<S0: StyleRef> StyleTuple for S0 {
    fn to_vec(self) -> Vec<StyleHandle> {
        vec![self.as_handle()]
    }
}

#[impl_for_tuples(1, 16)]
impl StyleTuple for Tuple {
    for_tuples!( where #( Tuple: StyleRef )* );

    fn to_vec(self) -> Vec<StyleHandle> {
        Vec::from([for_tuples!( #( self.Tuple.as_handle() ),* )])
    }
}
