


struct SearchTree<T> {
    children: Vec<Node<T>>,
}
impl<T> SearchTree<T> {
    fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, value: String, data: T){}
    pub fn remove(){}
    pub fn remove_next(){}
    
}
//apro

struct Node<T> {
    key: String,
    value: String,
    data: T,
    children: Vec<Node<T>>,
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut tree = SearchTree::new();
        tree.add("hello".to_string(), "hello world");
        assert_eq!(tree.children[0].key, "hello");
        assert_eq!(tree.children[0].value, "hello world");
        tree.add("hello".to_string(), "hello world2");
        assert_eq!(tree.children[0].value, "hello world2");
        tree.add("apple".to_string(), "apple");
        assert_eq!(tree.children[1].key, "apple");
        assert_eq!(tree.children[0].key, "hello");
        tree.add("hihi".to_string(), "hihi");
        assert_eq!(tree.children[1].key, "apple");
        assert_eq!(tree.children[0].key, "h");
        assert_eq!(tree.children[0].children[0].key, "ello");
        assert_eq!(tree.children[0].children[1].key, "ihi");
    }
}
