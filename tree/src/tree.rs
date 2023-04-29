static mut MAX_ID: usize = 0;
use crate::color::Color;

pub struct Tree {
    root: Option<Box<Node>>
}

pub struct Node {
    pub id: usize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    background_color: Color,
    node_vec: Vec<Box<Node>>
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: None
        }
    }

    pub fn get_root(&mut self) -> Option<&mut Node> {
        self.root.as_deref_mut().map(|mut root| { root })
    }

    pub fn set_root(&mut self, root: Node) {
        self.root = Some(Box::new(root));
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: unsafe {
                MAX_ID += 1;
                MAX_ID
            },
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            background_color: Color::from_argb(100, 0, 0, 0),
            node_vec: Vec::new()
        }
    }

    pub fn append(&mut self, node: Node) {
        self.node_vec.push(Box::new(node));
    }

    pub fn append_boxed(&mut self, boxed_node: Box<Node>) {
        self.node_vec.push(boxed_node);
    }

    pub fn remove_by_id(&mut self, node_id: usize) {
        self.node_vec.retain(|t| t.id != node_id);
    }

    pub fn get_children_len(&self) -> usize {
        self.node_vec.len()
    }

    pub fn get_child_by_index(&mut self, index: usize) -> Option<&mut Node> {
        Some(self.node_vec[index].as_mut())
    }

    pub fn get_background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::Tree;
    use super::Node;
    use super::Color;
    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        let mut root = Node::new();

        assert_eq!(root.id, 1);
        assert_eq!(root.get_children_len(), 0);

        tree.set_root(root);

        let node_0 = Node::new();
        let node_1 = Node::new();
        assert_eq!(node_0.id, 2);
        assert_eq!(node_1.id, 3);

        let root: &mut Node = tree.get_root().unwrap();
        assert_eq!(root.id, 1);
        assert_eq!(root.get_children_len(), 0);
        root.append(node_0);
        assert_eq!(root.get_children_len(), 1);
        root.append(node_1);
        assert_eq!(root.get_children_len(), 2);

        let node_2 = Node::new();
        assert_eq!(node_2.id, 4);

        let root: &mut Node = tree.get_root().unwrap();
        root.append(node_2);
        assert_eq!(root.get_children_len(), 3);

        let root_child_1 = root.get_child_by_index(1).unwrap();
        assert_eq!(root_child_1.id, 3);
        let id = root_child_1.id;
        root.remove_by_id(id);
        assert_eq!(root.get_children_len(), 2);

        root.remove_by_id(5);
        root.remove_by_id(5);
        root.remove_by_id(5);
        assert_eq!(root.get_children_len(), 2);

        let mut boxed_node_4 = Box::new(Node::new());
        assert_eq!(boxed_node_4.id, 5);
        assert_eq!(boxed_node_4.get_background_color().a(), 100);
        assert_eq!(boxed_node_4.get_background_color().r(), 0);
        assert_eq!(boxed_node_4.get_background_color().g(), 0);
        assert_eq!(boxed_node_4.get_background_color().b(), 0);

        boxed_node_4.set_background_color(Color::from_argb(99, 255, 255, 255));
        assert_eq!(boxed_node_4.get_background_color().a(), 99);
        assert_eq!(boxed_node_4.get_background_color().r(), 255);
        assert_eq!(boxed_node_4.get_background_color().g(), 255);
        assert_eq!(boxed_node_4.get_background_color().b(), 255);

        assert_eq!(boxed_node_4.get_width(), 0);
        boxed_node_4.set_width(100);
        assert_eq!(boxed_node_4.get_width(), 100);

        assert_eq!(boxed_node_4.get_height(), 0);
        boxed_node_4.set_height(50);
        assert_eq!(boxed_node_4.get_height(), 50);

        root.append_boxed(boxed_node_4);
        assert_eq!(root.get_children_len(), 3);
    }
}