pub enum BinaryTree<N, L> {
    Node {
        value: N,
        a: Box<BinaryTree<N, L>>,
        b: Box<BinaryTree<N, L>>,
    },
    Leaf {
        value: L,
    },
}

use BinaryTree::{Leaf, Node};

impl<N, L> BinaryTree<N, L>
where
    N: Clone,
    L: Clone,
{
    pub fn map_nodes(&self, func: &dyn Fn(&N) -> N) -> Self {
        match self {
            Leaf { value } => Leaf {
                value: value.clone(),
            },
            Node { value, a, b } => Node {
                value: func(value),
                a: Box::new(a.map_nodes(func)),
                b: Box::new(b.map_nodes(func)),
            },
        }
    }
}
