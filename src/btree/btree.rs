use crate::btree::btreenode::{BTreeNode, P};
use crate::btree::btreenode::Row;

#[derive(Debug, Clone, Default)]
pub struct BTree {
    pub root: Option<Box<BTreeNode>>,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }

    fn search(&self, id: u64) -> Option<&Row> {
        self.root.as_ref()?.search(id)
    }

    pub(crate) fn insert(&mut self, row: Row) {
        if let Some(root) = self.root.as_mut() {
            if root.keys.len() == 2 * P - 1 {
                let mut new_root = BTreeNode {
                    is_leaf: false,
                    keys: Vec::new(),
                    children: vec![Box::new(*root.clone())],
                };
                new_root.split_child(0, root);
                self.root = Some(Box::new(new_root));
            }
            self.root.as_mut().unwrap().insert_non_full(row);
        } else {
            self.root = Some(Box::new(BTreeNode {
                is_leaf: true,
                keys: vec![row],
                children: Vec::new(),
            }));
        }
    }
}
