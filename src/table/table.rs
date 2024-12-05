use crate::btree::btree::BTree;
use crate::btree::btreenode::{BTreeNode, Row};

#[derive(Debug, Clone, Default)]
pub struct Table<T: std::clone::Clone> {
    pub btree: BTree<T>,
}

impl<T: std::clone::Clone + std::default::Default + std::fmt::Debug> Table<T> {
    pub fn new() -> Self {
        Table { btree: BTree::new() }
    }

    pub fn insert_row(&mut self, row: Row<T>) {
        self.btree.insert(row);
    }

    pub fn print_rows(&self) {
        self.print_inorder(&self.btree.root);
    }

    fn print_inorder(&self, node: &Option<Box<BTreeNode<T>>>) {
        if let Some(n) = node {
            for i in 0..n.keys.len() {
                if !n.is_leaf {
                    self.print_inorder(&Some(n.children[i].clone()));
                }
                println!("({}, {:?})", n.keys[i].id, n.keys[i]);
            }
            if !n.is_leaf {
                self.print_inorder(&Some(n.children[n.keys.len()].clone()));
            }
        }
    }
}
