use crate::btree::btreenode::{BTreeNode, T};
use crate::btree::btreenode::Row;

#[derive(Debug, Clone)]
pub struct BTree {
    pub root: Option<Box<BTreeNode>>,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }

    fn search(&self, id: u32) -> Option<&Row> {
        self.root.as_ref()?.search(id)
    }

    pub(crate) fn insert(&mut self, row: Row) {
        if let Some(root) = self.root.as_mut() {
            if root.keys.len() == 2 * T - 1 {
                // Se il nodo radice è pieno, dobbiamo dividerlo
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
            // Creare un nuovo nodo radice se l'albero è vuoto
            self.root = Some(Box::new(BTreeNode {
                is_leaf: true,
                keys: vec![row],
                children: Vec::new(),
            }));
        }
    }
}
