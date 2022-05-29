use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
enum Color {
    Red,
    Black,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Node<T> {
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    key: Option<T>,
    color: Option<Color>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct RBtree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
    sentinel: Rc<RefCell<Node<T>>>,
    size: i64,
}

#[allow(dead_code)]
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

    pub fn search(&self, item: T) -> Option<T> {
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
                    return Some(key);
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
}

#[cfg(test)]
mod test {

    use super::RBtree;

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
}
