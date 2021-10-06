use std::cell::RefCell;
use std::rc::Rc;

trait Checker {
    fn check(&self) -> bool;
    fn get_name(&self) -> String;
    fn set_next(&mut self, checker: Rc<RefCell<Box<dyn Checker>>>);
    fn get_next(&self) -> Option<Rc<RefCell<Box<dyn Checker>>>>;
}

struct CheckerA {
    name: String,
    next: Option<Rc<RefCell<Box<dyn Checker>>>>,
}

impl CheckerA {
    fn new(name: String) -> Self {
        Self {
            name: name,
            next: None,
        }
    }
}

impl Checker for CheckerA {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_next(&self) -> Option<Rc<RefCell<Box<dyn Checker>>>> {
        self.next.clone()
    }

    fn set_next(&mut self, next: Rc<RefCell<Box<dyn Checker>>>) {
        self.next = Some(Rc::clone(&next));
    }

    fn check(&self) -> bool {
        println!("{} passed!", self.get_name());
        if let Some(node) = self.next.clone() {
            node.as_ref().borrow().check()
        } else {
            true
        }
    }
}

struct CheckerB {
    name: String,
    next: Option<Rc<RefCell<Box<dyn Checker>>>>,
}

impl CheckerB {
    fn new(name: String) -> Self {
        Self {
            name: name,
            next: None,
        }
    }
}

impl Checker for CheckerB {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_next(&self) -> Option<Rc<RefCell<Box<dyn Checker>>>> {
        self.next.clone()
    }

    fn set_next(&mut self, next: Rc<RefCell<Box<dyn Checker>>>) {
        self.next = Some(Rc::clone(&next));
    }

    fn check(&self) -> bool {
        println!("{} passed!", self.get_name());
        if let Some(node) = self.next.clone() {
            node.as_ref().borrow().check()
        } else {
            true
        }
    }
}

fn build_chain(
    checkers: Vec<Rc<RefCell<Box<dyn Checker>>>>,
) -> Result<Rc<RefCell<Box<dyn Checker>>>, String> {
    let length = checkers.len();
    if length < 1 {
        return Err("Empty checkers".to_string());
    }

    let head = checkers.get(0).unwrap().clone();
    if checkers.len() == 1 {
        return Ok(head);
    }

    let mut prev = head.clone();

    for i in 1..length {
        let c = checkers.get(i).unwrap().clone();
        prev.as_ref().borrow_mut().set_next(c.clone());
        prev = c.clone();
    }

    Ok(head)
}

fn main() {
    let checker_a: Rc<RefCell<Box<dyn Checker>>> = Rc::new(RefCell::new(Box::new(CheckerA::new(
        "checker_a".to_string(),
    ))));
    let checker_b: Rc<RefCell<Box<dyn Checker>>> = Rc::new(RefCell::new(Box::new(CheckerB::new(
        "checker_b".to_string(),
    ))));

    let checkers: Vec<Rc<RefCell<Box<dyn Checker>>>> = vec![checker_a, checker_b];
    let chain = build_chain(checkers).unwrap();
    chain.as_ref().borrow().check();
}
