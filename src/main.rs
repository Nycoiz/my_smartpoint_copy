use crate::List::{Cons,Nil};
use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};
#[derive(Debug)]
enum List
{
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List
{
    fn tail(&self) ->Option<&RefCell<Rc<List>>>
    {
        match self
        {
            Cons(_, item)=>Some(item),
            Nil=>None,
        }
    }
}

// enum List
// {
//     Cons(Rc<RefCell<i32>>,Rc<List>),
//     Nil,
// }

#[derive(Debug)]
struct Node
{
    value: i32,
    parent: RefCell<Weak<Node>>,
    childern: RefCell<Vec<Rc<Node>>>,
}

fn main() 
{
    // let value=Rc::new(RefCell::new(5));
    // let a=Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b=Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c=Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    // *value.borrow_mut() +=10;

    // println!("a after = {:?}",a);
    // println!("b after = {:?}",b);
    // println!("c after = {:?}",c);

    let a=Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}",Rc::strong_count(&a));
    println!("a next item is {:?}",a.tail());

    let b=Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}",Rc::strong_count(&a));
    println!("b initial rc count = {}",Rc::strong_count(&b));
    println!("b next item is {:?}",b.tail());

    if let Some(link)=a.tail()
    {
        *link.borrow_mut()=Rc::clone(&b);
    }

    println!("b rc count after changing a = {}",Rc::strong_count(&b));
    println!("a rc count after changing a = {}",Rc::strong_count(&a));

    let leaf=Rc::new(Node
    {
        value: 3,
        parent: RefCell::new(Weak::new()),
        childern: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    {
        let branch=Rc::new(Node
        {
            value: 5,
            parent: RefCell::new(Weak::new()),
            childern: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut()=Rc::downgrade(&branch);
        println!("leaf praent = {:?}",leaf.parent.borrow().upgrade());
        println!("branch strong = {}, weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    }
    println!("leaf praent = {:?}",leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
}