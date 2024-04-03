
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! Error {
    ($($arg:tt)*) => { FatalError(format!($($arg)*)); }
}

macro_rules! FatalError {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }}
}

type ElementType = i32;

#[derive(Clone)]
struct TreeNode {
    element: ElementType,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type SearchTree = Option<Rc<RefCell<TreeNode>>>;

fn make_empty(T: SearchTree) -> SearchTree {
    if let Some(t) = T {
        make_empty(t.borrow().left.clone());
        make_empty(t.borrow().right.clone());
    }
    None
}

fn find(X: ElementType, T: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match T {
        Some(t) => {
            if X < t.borrow().element {
                find(X, t.borrow().left.clone())
            } else if X > t.borrow().element {
                find(X, t.borrow().right.clone())
            } else {
                Some(t)
            }
        }
        None => None,
    }
}

fn find_min(T: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    match T {
        Some(t) => {
            if t.borrow().left.is_none() {
                Some(t)
            } else {
                find_min(t.borrow().left.clone())
            }
        }
        None => None,
    }
}

fn find_max(T: SearchTree) -> Option<Rc<RefCell<TreeNode>>> {
    let mut t = T;
    while let Some(tt) = t {
        if tt.borrow().right.is_none() {
            break;
        }
        t = tt.borrow().right.clone();
    }
    t
}

fn insert(X: ElementType, T: SearchTree) -> SearchTree {
    match T {
        Some(t) => {
            if X < t.borrow().element {
                Some(Rc::new(RefCell::new(TreeNode {
                    element: t.borrow().element,
                    left: insert(X, t.borrow().left.clone()),
                    right: t.borrow().right.clone(),
                })))
            } else if X > t.borrow().element {
                Some(Rc::new(RefCell::new(TreeNode {
                    element: t.borrow().element,
                    left: t.borrow().left.clone(),
                    right: insert(X, t.borrow().right.clone()),
                })))
            } else {
                Some(t)
            }
        }
        None => {
            let new_node = TreeNode {
                element: X,
                left: None,
                right: None,
            };
            Some(Rc::new(RefCell::new(new_node)))
        }
    }
}

fn delete(X: ElementType, T: SearchTree) -> SearchTree {
    if let Some(t) = T {
        if X < t.borrow().element {
            Some(Rc::new(RefCell::new(TreeNode {
                element: t.borrow().element,
                left: delete(X, t.borrow().left.clone()),
                right: t.borrow().right.clone(),
            })))
        } else if X > t.borrow().element {
            Some(Rc::new(RefCell::new(TreeNode {
                element: t.borrow().element,
                left: t.borrow().left.clone(),
                right: delete(X, t.borrow().right.clone()),
            })))
        } else {
            if t.borrow().left.is_some() && t.borrow().right.is_some() {
                let min_right = find_min(t.borrow().right.clone());
                let min_right_value = min_right.as_ref().unwrap().borrow().element;
                Some(Rc::new(RefCell::new(TreeNode {
                    element: min_right_value,
                    left: t.borrow().left.clone(),
                    right: delete(min_right_value, t.borrow().right.clone()),
                })))
            } else if t.borrow().left.is_some() {
                t.borrow().left.clone()
            } else if t.borrow().right.is_some() {
                t.borrow().right.clone()
            } else {
                None
            }
        }
    } else {
        Error!("Element not found");
        None
    }
}

fn retrieve(P: Option<Rc<RefCell<TreeNode>>>) -> ElementType {
    P.map(|p| p.borrow().element).unwrap_or(0)
}

fn main() {
    let mut T = None;
    let mut P;
    let mut i;
    let mut j = 0;

    T = make_empty(T);
    for i in 0..50 {
        j = (j + 7) % 50;
        T = insert(j, T);
    }
    for i in 0..50 {
        P = find(i, T.clone());
        if P.is_none() ||
        // token 超长