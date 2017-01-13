// variable bindings example.
fn main() {
    let x = 5; // x is a binding with value five. Its an immutable binding.
   // bellow will cause error.
   // x = 10;

   let mut y = 10; // y is a binding with value ten. Its mutable binding.
   //will works fine.
   //y = 20;

   // immutable variable binding with explict type
   let x1: i32 = 5; //x is a binding with the type i32 and the value five.
 
   // mutable variable binding with explict type
   let mut y1: i32 = 10; //y is a binding with the type i32 and the value ten.

   println!("The values are x={} y={} and x1={} y1={}",x,y,x1,y1);
}
