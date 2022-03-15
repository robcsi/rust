use std::{rc::Rc, cell::RefCell};

fn main() {
   let rc: Rc<i32> = Rc::new(5); 
   println!("Reference count is: {}", Rc::strong_count(&rc));
   let rc2 = rc.clone();
   println!("Reference count 2 is: {}", Rc::strong_count(&rc2));
   rc2.as_ref();

   let rfc = RefCell::new(10);

   let rfcb = rfc.borrow();
   let rfcb2 = rfc.clone();
   let mut val = *rfcb2.borrow_mut();
   val += 1;
   println!("Value is: {}", rfcb);
   println!("Value is: {}", val);
   rfcb2.swap(&RefCell::new(val));
   println!("Value is: {}", rfcb2.borrow());

   let text: &str = "robi";
   let string= text.to_owned();
   let utf = text;
}
