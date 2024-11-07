#[derive(Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

pub const T: usize = 3;
#[derive(Debug, Clone, Default)]
pub struct BTreeNode {
    pub is_leaf: bool,
    pub keys: Vec<Row>,
    pub children: Vec<Box<BTreeNode>>,
}

impl BTreeNode {
    pub(crate) fn search(&self, id: u32) -> Option<&Row> {
        // Cerca l'id in questo nodo
        let mut i = 0;
        while i < self.keys.len() && id > self.keys[i].id {
            i += 1;
        }

        // Se l'ID è trovato in questo nodo, ritorna la riga
        if i < self.keys.len() && self.keys[i].id == id {
            return Some(&self.keys[i]);
        }

        // Altrimenti, ricerca nel figlio appropriato
        if self.is_leaf {
            None
        } else {
            self.children[i].search(id)
        }
    }

    pub fn insert_non_full(&mut self, row: Row) {
        let mut i = self.keys.len();

        if self.is_leaf {
            // Inserisci direttamente se è una foglia
            while i > 0 && row.id < self.keys[i - 1].id {
                i -= 1;
            }
            self.keys.insert(i, row);
        } else {
            // Individua il figlio dove inserire
            while i > 0 && row.id < self.keys[i - 1].id {
                i -= 1;
            }

            // Preleva temporaneamente il figlio completo
            if self.children[i].keys.len() == 2 * T - 1 {
                // Rimuove il figlio temporaneamente e lo divide
                let mut child = std::mem::take(&mut self.children[i]);
                self.split_child(i, &mut child);
                self.children[i] = child; // Reinserisce il figlio

                // Regola `i` per puntare al figlio corretto dopo la divisione
                if row.id > self.keys[i].id {
                    i += 1;
                }
            }

            // Ora possiamo accedere al figlio senza doppio prestito mutabile
            self.children[i].insert_non_full(row);
        }
    }


    pub fn split_child(&mut self, i: usize, node: &mut BTreeNode) {
        let mut new_node = BTreeNode {
            is_leaf: node.is_leaf,
            keys: node.keys.split_off(T - 1),
            children: if node.is_leaf {
                Vec::new()
            } else {
                node.children.split_off(T)
            },
        };
        self.keys.insert(i, node.keys.pop().unwrap());
        self.children.insert(i + 1, Box::new(new_node));
    }
}


