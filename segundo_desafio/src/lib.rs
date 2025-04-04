#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>, // Corrigido de 'rigth' para 'right'
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None, // Corrigido de 'rigth' para 'right'
                    }));
                    return;
                }

                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right; // Corrigido de 'rigth' para 'right'
                    } else {
                        return;
                    }
                }
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right; // Corrigido de 'rigth' para 'right'
            }
        }

        false
    }
}

#[cfg(test)]
mod bst_tests {
    use super::BST;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));

        assert!(!bst.search(20));
        assert!(!bst.search(2));

        assert!(!bst.is_empty());
    }
}

pub fn main() {
    let mut bst = BST::new();

    if bst.is_empty() {
        println!("A árvore está vazia.");
    }
    
    bst.insert(10);
    bst.insert(27);
    bst.insert(23);
    bst.insert(7);

    println!("10: {}", bst.search(10));
    println!("7: {}", bst.search(7));
    println!("24: {}", bst.search(24));
}
