use crate::btree::btree::BTree;
use crate::btree::btreenode::{BTreeNode, Row};

#[derive(Debug, Clone)]
pub struct Table {
    pub btree: BTree,
}

impl Table {
    pub fn new() -> Self {
        Table { btree: BTree::new() }
    }

    pub fn insert_row(&mut self, row: Row) {
        self.btree.insert(row);
    }

    pub fn print_rows(&self) {
        self.print_inorder(&self.btree.root);
    }

    fn print_inorder(&self, node: &Option<Box<BTreeNode>>) {
        if let Some(n) = node {
            for i in 0..n.keys.len() {
                if !n.is_leaf {
                    self.print_inorder(&Some(n.children[i].clone()));
                }
                println!("({}, {}, {})", n.keys[i].id, n.keys[i].username, n.keys[i].email);
            }
            if !n.is_leaf {
                self.print_inorder(&Some(n.children[n.keys.len()].clone()));
            }
        }
    }
}
