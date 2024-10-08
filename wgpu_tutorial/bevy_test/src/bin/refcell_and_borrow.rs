use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

struct MyStruct {
    old: RefCell<Point>,
    new: RefCell<Point>,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let my_struct = MyStruct {
        old: RefCell::new(p1),
        new: RefCell::new(p2),
    };

    use_ref(&my_struct.old, &my_struct.new);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    my_struct.old.borrow_mut().x += 1.0;
}

fn use_ref(p1: &RefCell<Point>, p2: &RefCell<Point>) -> Point {
    p1.borrow_mut().x += 1.0;
    p1.borrow_mut().y += 1.0;
    p2.borrow_mut().x += 1.0;
    p2.borrow_mut().y += 1.0;
    Point {
        x: p1.borrow().x + p2.borrow().x,
        y: p1.borrow().y + p2.borrow().y,
    }
}
