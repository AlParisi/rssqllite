use sqlparser::ast::DataType;

#[derive(Debug, Clone)]
pub struct Row {
    pub id: u64,
    pub data: Vec<DataType>
}

impl Row {
    pub fn new(data: Vec<DataType>) -> Self {
        let mut i: u64 = 0;
        Row {
            id: i + 1,
            data
        }
    }
}

pub const P: usize = 3;
#[derive(Debug, Clone, Default)]
pub struct BTreeNode{
    pub is_leaf: bool,
    pub keys: Vec<Row>,
    pub children: Vec<Box<BTreeNode>>,
}

impl BTreeNode{
    pub(crate) fn search(&self, id: u64) -> Option<&Row> {
        let mut i = 0;
        while i < self.keys.len() && id > self.keys[i].id {
            i += 1;
        }

        if i < self.keys.len() && self.keys[i].id == id {
            return Some(&self.keys[i]);
        }

        if self.is_leaf {
            None
        } else {
            self.children[i].search(id)
        }
    }

    pub fn insert_non_full(&mut self, row: Row) {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && row.id < self.keys[i - 1].id {
                i -= 1;
            }
            self.keys.insert(i, row);
        } else {
            while i > 0 && row.id < self.keys[i - 1].id {
                i -= 1;
            }

            if self.children[i].keys.len() == 2 * P - 1 {
                let mut child = std::mem::take(&mut self.children[i]);
                self.split_child(i, &mut child);
                self.children[i] = child;

                if row.id > self.keys[i].id {
                    i += 1;
                }
            }

            self.children[i].insert_non_full(row);
        }
    }


    pub fn split_child(&mut self, i: usize, node: &mut BTreeNode) {
        let new_node = BTreeNode {
            is_leaf: node.is_leaf,
            keys: node.keys.split_off(3 - 1),
            children: if node.is_leaf {
                Vec::new()
            } else {
                node.children.split_off(P)
            },
        };
        self.keys.insert(i, node.keys.pop().unwrap());
        self.children.insert(i + 1, Box::new(new_node));
    }
}


