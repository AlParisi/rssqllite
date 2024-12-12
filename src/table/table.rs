use std::collections::HashMap;
use sqlparser::ast::DataType;
use crate::btree::btree::BTree;
use crate::btree::btreenode::{BTreeNode, Row};

#[derive(Debug, Clone, Default)]
pub struct Table{
    pub name: String,
    pub column: HashMap<String, DataType>,
    pub btree: BTree
}

impl Table{
    pub fn new(name: String, column: HashMap<String, DataType>) -> Self {
        Table { name,
                column,
                btree: BTree::new()
        }
    }

    pub fn get_table(name: String)-> Table {
        Table { name,
                column: HashMap::new(),
                btree: BTree::new()
        }
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
                println!("({}, {:?})", n.keys[i].id, n.keys[i]);
            }
            if !n.is_leaf {
                self.print_inorder(&Some(n.children[n.keys.len()].clone()));
            }
        }
    }
}
