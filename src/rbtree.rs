use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Clone)]
struct Node<T> {
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    key: Option<T>,
    color: Option<Color>,
}

impl<T> Node<T> {
    fn new(key: T, color: Color) -> Self {
        Node {
            parent: None,
            left: None,
            right: None,
            key: Some(key),
            color: Some(color),
        }
    }
    fn sentinel() -> Rc<RefCell<Self>> {
        let n = Node {
            parent: None,
            left: None,
            right: None,
            key: None,
            color: Some(Color::Black),
        };

        let r = Rc::new(RefCell::new(n));

        r.borrow_mut().left = Some(Rc::clone(&r));
        r.borrow_mut().right = Some(Rc::clone(&r));
        r.borrow_mut().parent = Some(Rc::clone(&r));

        return r;
    }
}

pub struct RBtree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
    sentinel: Rc<RefCell<Node<T>>>,
    size: i64,
}

impl<T> RBtree<T>
where
    T: PartialOrd + PartialEq + Clone,
{
    pub fn new() -> Self {
        let sentinel = Node::sentinel();

        RBtree {
            root: Some(Rc::clone(&sentinel)),
            sentinel: Rc::clone(&sentinel),
            size: 0,
        }
    }

    pub fn size(&self) -> i64 {
        self.size
    }
    fn rb_left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        let y: Rc<RefCell<Node<T>>> = {
            let borrowed_x = x.borrow();
            Rc::clone(borrowed_x.right.as_ref().unwrap())
        };
        x.borrow_mut().right = Some(Rc::clone(y.borrow().left.as_ref().unwrap()));

        if Rc::as_ptr(y.borrow().left.as_ref().unwrap()) != Rc::as_ptr(&self.sentinel) {
            let borrowed_y = y.borrow();
            borrowed_y.left.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(&x));
        }

        let y_ref = Rc::clone(&y);
        y.borrow_mut().parent = Some(Rc::clone(x.borrow().parent.as_ref().unwrap()));
        if Rc::as_ptr(x.borrow().parent.as_ref().unwrap()) == Rc::as_ptr(&self.sentinel) {
            self.root = Some(y);
        } else if Rc::as_ptr(&x) == Rc::as_ptr(x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) {
            x.borrow().parent.as_ref().unwrap().borrow_mut().left = Some(y);
        } else {
            x.borrow().parent.as_ref().unwrap().borrow_mut().right = Some(y);
        }

        y_ref.borrow_mut().left = Some(Rc::clone(&x));
        x.borrow_mut().parent = Some(Rc::clone(&y_ref));
    }

    fn rb_right_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        let y: Rc<RefCell<Node<T>>> = {
            let borrowed_x = x.borrow();
            Rc::clone(borrowed_x.left.as_ref().unwrap())
        };

        x.borrow_mut().left = Some(Rc::clone(y.borrow().right.as_ref().unwrap()));

        if Rc::as_ptr(y.borrow().right.as_ref().unwrap()) != Rc::as_ptr(&self.sentinel) {
            let borrowed_y = y.borrow();
            borrowed_y.right.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(&x));
        }

        y.borrow_mut().parent = Some(Rc::clone(x.borrow().parent.as_ref().unwrap()));
        let y_ref = Rc::clone(&y);

        if Rc::as_ptr(x.borrow().parent.as_ref().unwrap()) == Rc::as_ptr(&self.sentinel) {
            self.root = Some(y);
        } else if Rc::as_ptr(&x) == Rc::as_ptr(x.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap()) {
            x.borrow().parent.as_ref().unwrap().borrow_mut().right = Some(y);
        } else {
            x.borrow().parent.as_ref().unwrap().borrow_mut().left = Some(y);
        }

        y_ref.borrow_mut().right = Some(Rc::clone(&x));
        x.borrow_mut().parent = Some(Rc::clone(&y_ref));
    }

    fn rb_insert_fixup(&mut self, mut z: Rc<RefCell<Node<T>>>) {
        while *(z.borrow().parent.as_ref().unwrap().borrow().color.as_ref().unwrap()) == Color::Red {
            let z_parent_ptr = Rc::as_ptr(z.borrow().parent.as_ref().unwrap());
            let z_parent_parent_left_ptr = { Rc::as_ptr(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) }; // this is grotesque
            if z_parent_ptr == z_parent_parent_left_ptr {
                let y = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap());

                if *(y.borrow().color.as_ref().unwrap()) == Color::Red {
                    z.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    y.borrow_mut().color = Some(Color::Black);
                    z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let z_new = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap());
                    z = z_new;
                } else {
                    if Rc::as_ptr(&z) == Rc::as_ptr(z.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap()) {
                        let z_new = Rc::clone(z.borrow().parent.as_ref().unwrap());
                        z = z_new;
                        self.rb_left_rotate(Rc::clone(&z));
                    }
                    z.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let z_parent_parent = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap());
                    self.rb_right_rotate(z_parent_parent);
                }
            } else {
                let y = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap());

                if *(y.borrow().color.as_ref().unwrap()) == Color::Red {
                    z.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    y.borrow_mut().color = Some(Color::Black);
                    z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let z_new = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap());
                    z = z_new;
                } else {
                    if Rc::as_ptr(&z) == Rc::as_ptr(z.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) {
                        let z_new = Rc::clone(z.borrow().parent.as_ref().unwrap());
                        z = z_new;
                        self.rb_right_rotate(Rc::clone(&z));
                    }
                    z.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let z_parent_parent = Rc::clone(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap());
                    self.rb_left_rotate(z_parent_parent);
                }
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
    }

    fn rb_insert(&mut self, z: Rc<RefCell<Node<T>>>) {
        let mut y: Rc<RefCell<Node<T>>> = Rc::clone(&self.sentinel);
        let mut x: Rc<RefCell<Node<T>>> = Rc::clone(&self.root.as_ref().unwrap());

        while Rc::as_ptr(&x) != Rc::as_ptr(&self.sentinel) {
            y = Rc::clone(&x);
            if *z.borrow().key.as_ref().unwrap() < *x.borrow().key.as_ref().unwrap() {
                x = {
                    let borrowed_x = x.borrow();
                    Rc::clone(borrowed_x.left.as_ref().unwrap_or(&self.sentinel))
                };
            } else {
                x = {
                    let borrowed_x = x.borrow();
                    Rc::clone(borrowed_x.right.as_ref().unwrap_or(&self.sentinel))
                };
            }
        }

        z.borrow_mut().parent = Some(Rc::clone(&y));

        let z_ref = Rc::clone(&z);

        if Rc::as_ptr(&y) == Rc::as_ptr(&self.sentinel) {
            self.root = Some(z);
        } else if *z.borrow().key.as_ref().unwrap() < *y.borrow().key.as_ref().unwrap() {
            y.borrow_mut().left = Some(z);
        } else {
            y.borrow_mut().right = Some(z);
        }

        z_ref.borrow_mut().left = Some(Rc::clone(&self.sentinel));
        z_ref.borrow_mut().right = Some(Rc::clone(&self.sentinel));
        z_ref.borrow_mut().color = Some(Color::Red);

        self.rb_insert_fixup(z_ref);
    }

    fn rb_search(&self, item: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut node = Rc::clone(self.root.as_ref().unwrap());
        while Rc::as_ptr(&node) != Rc::as_ptr(&self.sentinel) {
            let key: T = (*node.borrow().key.as_ref().unwrap()).clone();
            if key == item {
                let left_key = node.borrow().left.as_ref().unwrap().borrow().key.clone();
                let right_key = node.borrow().right.as_ref().unwrap().borrow().key.clone();
                if Rc::as_ptr(node.borrow().left.as_ref().unwrap()) != Rc::as_ptr(&self.sentinel) && left_key == Some(item.clone()) {
                    node = {
                        let borrowed_node = node.borrow();
                        Rc::clone(borrowed_node.left.as_ref().unwrap())
                    }
                } else if Rc::as_ptr(node.borrow().right.as_ref().unwrap()) != Rc::as_ptr(&self.sentinel) && right_key == Some(item.clone()) {
                    node = {
                        let borrowed_node = node.borrow();
                        Rc::clone(borrowed_node.right.as_ref().unwrap())
                    }
                } else {
                    return Some(Rc::clone(&node));
                }
            } else if key < item {
                node = {
                    let borrowed_node = node.borrow();
                    Rc::clone(borrowed_node.right.as_ref().unwrap())
                }
            } else {
                node = {
                    let borrowed_node = node.borrow();
                    Rc::clone(borrowed_node.left.as_ref().unwrap())
                }
            }
        }

        None
    }

    fn rb_tree_minimum(&self, mut node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let mut last_node = Rc::clone(&node);
        while Rc::as_ptr(&node) != Rc::as_ptr(&self.sentinel) {
            last_node = Rc::clone(&node);
            let next = Rc::clone(node.borrow().left.as_ref().unwrap());
            node = next;
        }
        last_node
    }

    fn rb_tree_maximum(&self, mut node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let mut last_node = Rc::clone(&node);
        while Rc::as_ptr(&node) != Rc::as_ptr(&self.sentinel) {
            last_node = Rc::clone(&node);
            let next = Rc::clone(node.borrow().right.as_ref().unwrap());
            node = next;
        }
        last_node
    }

    fn rb_transplant(&mut self, u: Rc<RefCell<Node<T>>>, v: Rc<RefCell<Node<T>>>) {
        if Rc::as_ptr(u.borrow().parent.as_ref().unwrap()) == Rc::as_ptr(&self.sentinel) {
            self.root = Some(Rc::clone(&v));
        } else if Rc::as_ptr(&u) == Rc::as_ptr(u.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap()) {
            u.borrow().parent.as_ref().unwrap().borrow_mut().left = Some(Rc::clone(&v));
        } else {
            u.borrow().parent.as_ref().unwrap().borrow_mut().right = Some(Rc::clone(&v));
        }
        v.borrow_mut().parent = Some(Rc::clone(u.borrow().parent.as_ref().unwrap()));
    }

    fn rb_delete_fixup(&mut self, mut x: Rc<RefCell<Node<T>>>) {
        let mut x_color = x.borrow().color.as_ref().unwrap().clone();
        while Rc::as_ptr(&x) != Rc::as_ptr(&self.root.as_ref().unwrap()) && x_color != Color::Black {
            let x_parent_left_ptr = Rc::as_ptr(x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap());
            if Rc::as_ptr(&x) == x_parent_left_ptr {
                let mut w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap());
                if w.borrow().color.as_ref().unwrap().clone() == Color::Red {
                    w.borrow_mut().color = Some(Color::Black);
                    x.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    self.rb_left_rotate(x_parent);
                    w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap());
                }
                let w_left_color = w.borrow().left.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                let w_right_color = w.borrow().right.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                if w_left_color == Color::Black && w_right_color == Color::Black {
                    w.borrow_mut().color = Some(Color::Red);
                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    x = x_parent;
                } else {
                    let w_right_color = w.borrow().right.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                    if w_right_color == Color::Black {
                        w.borrow().left.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                        w.borrow_mut().color = Some(Color::Red);
                        self.rb_right_rotate(Rc::clone(&w));
                        w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap());
                    }
                    let x_parent_color = x.borrow().parent.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                    w.borrow_mut().color = Some(x_parent_color);
                    x.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    w.borrow().right.as_ref().unwrap().borrow_mut().color = Some(Color::Black);

                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    self.rb_left_rotate(x_parent);
                    x = Rc::clone(&self.root.as_ref().unwrap());
                }
            } else {
                let mut w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap());
                if w.borrow().color.as_ref().unwrap().clone() == Color::Red {
                    w.borrow_mut().color = Some(Color::Black);
                    x.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Red);
                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    self.rb_right_rotate(x_parent);
                    w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap());
                }
                let w_left_color = w.borrow().left.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                let w_right_color = w.borrow().right.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                if w_left_color == Color::Black && w_right_color == Color::Black {
                    w.borrow_mut().color = Some(Color::Red);
                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    x = x_parent;
                } else {
                    let w_left_color = w.borrow().left.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                    if w_left_color == Color::Black {
                        w.borrow().right.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                        w.borrow_mut().color = Some(Color::Red);
                        self.rb_left_rotate(Rc::clone(&w));
                        w = Rc::clone(x.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap());
                    }
                    let x_parent_color = x.borrow().parent.as_ref().unwrap().borrow().color.as_ref().unwrap().clone();
                    w.borrow_mut().color = Some(x_parent_color);
                    x.borrow().parent.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    w.borrow().left.as_ref().unwrap().borrow_mut().color = Some(Color::Black);
                    let x_parent = Rc::clone(x.borrow().parent.as_ref().unwrap());
                    self.rb_right_rotate(x_parent);
                    x = Rc::clone(&self.root.as_ref().unwrap());
                }
            }
            x_color = x.borrow().color.as_ref().unwrap().clone();
        }
        x.borrow_mut().color = Some(Color::Black);
    }

    fn rb_delete(&mut self, z: Rc<RefCell<Node<T>>>) {
        let mut y: Rc<RefCell<Node<T>>> = Rc::clone(&z);
        let x: Rc<RefCell<Node<T>>>;
        let mut y_original_color: Color = y.borrow().color.as_ref().unwrap().clone();

        if Rc::as_ptr(z.borrow().left.as_ref().unwrap()) == Rc::as_ptr(&self.sentinel) {
            x = Rc::clone(z.borrow().right.as_ref().unwrap());
            let z_right = Rc::clone(z.borrow().right.as_ref().unwrap());
            self.rb_transplant(Rc::clone(&z), z_right);
        } else if Rc::as_ptr(z.borrow().right.as_ref().unwrap()) == Rc::as_ptr(&self.sentinel) {
            x = Rc::clone(z.borrow().left.as_ref().unwrap());
            let z_left = Rc::clone(z.borrow().left.as_ref().unwrap());
            self.rb_transplant(Rc::clone(&z), z_left);
        } else {
            let z_right = Rc::clone(z.borrow().right.as_ref().unwrap());
            y = self.rb_tree_minimum(z_right);
            y_original_color = y.borrow().color.as_ref().unwrap().clone();
            x = Rc::clone(y.borrow().right.as_ref().unwrap());
            if Rc::as_ptr(x.borrow().parent.as_ref().unwrap()) == Rc::as_ptr(&z) {
                x.borrow_mut().parent = Some(Rc::clone(&y));
            } else {
                let y_right = Rc::clone(y.borrow().right.as_ref().unwrap());
                self.rb_transplant(Rc::clone(&y), y_right);
                y.borrow_mut().right = Some(Rc::clone(z.borrow().right.as_ref().unwrap()));
                y.borrow().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(&y));
            }

            self.rb_transplant(Rc::clone(&z), Rc::clone(&y));
            y.borrow_mut().left = Some(Rc::clone(z.borrow().left.as_ref().unwrap()));
            y.borrow().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(&y));
            y.borrow_mut().color = Some(z.borrow().color.as_ref().unwrap().clone());
        }

        if y_original_color == Color::Black {
            self.rb_delete_fixup(x)
        }
    }

    pub fn min(&self) -> Option<T> {
        let root = Rc::clone(self.root.as_ref().unwrap());
        if let Some(x) = self.rb_tree_minimum(root).borrow().key.clone() {
            Some(x)
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<T> {
        let root = Rc::clone(self.root.as_ref().unwrap());
        if let Some(x) = self.rb_tree_maximum(root).borrow().key.clone() {
            Some(x)
        } else {
            None
        }
    }

    pub fn search(&self, item: T) -> Option<T> {
        if let Some(x) = self.rb_search(item) {
            Some(x.borrow().key.as_ref().unwrap().clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, item: T) {
        let mut node = Node::new(item, Color::Red);
        node.left = Some(Rc::clone(&self.sentinel));
        node.right = Some(Rc::clone(&self.sentinel));
        node.parent = Some(Rc::clone(&self.sentinel));
        if Rc::as_ptr(&self.root.as_ref().unwrap_or(&self.sentinel)) != Rc::as_ptr(&self.sentinel) {
            self.rb_insert(Rc::new(RefCell::new(node)));
        } else {
            node.color = Some(Color::Black);
            self.root = Some(Rc::new(RefCell::new(node)));
        }
        self.size += 1;
    }

    pub fn delete(&mut self, item: T) -> Result<T, &'static str> {
        let node = self.rb_search(item.clone());
        if let Some(ref x) = node {
            self.rb_delete(Rc::clone(x));
            Ok(item)
        } else {
            Err("not found")
        }
    }
}

#[cfg(test)]
mod test {

    use super::RBtree;
    use crate::rand::Rand;
    use std::collections::HashSet;

    #[test]
    fn insert_works_str() {
        let mut t: RBtree<String> = RBtree::new();

        let n1 = String::from("foo");
        let n2 = String::from("bar");
        let n3 = String::from("baz");

        t.insert(n1);
        t.insert(n2);
        t.insert(n3);
        assert_eq!(t.size(), 3);
    }

    #[test]
    fn insert_works_int() {
        let mut t: RBtree<i32> = RBtree::new();
        let size = 10000;

        for i in 0..size {
            let n = i;
            t.insert(n);
        }

        for i in (0..size).rev() {
            assert_eq!(t.search(i), Some(i));
        }

        assert_eq!(t.size(), size as i64);
    }

    #[test]
    fn insert_works_randint() {
        let mut t: RBtree<i32> = RBtree::new();
        let size = 10000;

        let mut r = Rand::srand(69);

        let mut values = Vec::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for _ in 0..size {
            let n = (r.rand() % size) as i32;
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            values.push(n);
            t.insert(n);
        }

        for i in 0..size {
            let n = values[i as usize];
            assert_eq!(t.search(n), Some(n));
        }

        assert_eq!(t.min(), Some(min));
        assert_eq!(t.max(), Some(max));

        assert_eq!(t.size(), size as i64);
    }

    #[test]
    fn delete_works_randint() {
        let mut t: RBtree<i32> = RBtree::new();
        let size = 1000;

        let mut r = Rand::srand(69);

        let mut values = HashSet::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for _ in 0..size {
            let mut n = (r.rand() % size) as i32;
            while values.contains(&n) {
                n = (r.rand() % size) as i32;
            }
            values.insert(n);
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            t.insert(n);
        }

        for (_idx, item) in values.iter().enumerate() {
            println!("{}", _idx);
            let actual = t.delete(*item);
            assert_eq!(t.search(*item), None);
            assert_eq!(actual, Ok(*item));
        }

        assert_eq!(t.min(), None);
        assert_eq!(t.max(), None);

        assert_eq!(t.size(), 0);
    }
}
