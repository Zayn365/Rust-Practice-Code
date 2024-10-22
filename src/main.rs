
// Functionality and reference string practice
// fn hello_world (name: &str) {
//    println!("Hello World from {}", name);
// }
// fn main() {
//     println!("Hello, world!");
//     hello_world("Zayn");
// }

// Bank Account Demo using struct, impl, pointer
// fn main () {
//     let mut account: BankAccount = BankAccount{
//         owner: "Alice".to_string(),
//         balance: 555.34
//     };
//     // Before Withdrawl
//     account.check_balance();
// // Withdrawl
//     account.withdraw_balance(32.2);
//     // After Withdrawl 
//     account.check_balance();
// }


// struct BankAccount {
//     owner: String,
//     balance: f32
// }


// impl BankAccount {
//     // Withdrawl function
//     fn withdraw_balance(&mut self , amount: f32) {
//         self.balance -= amount;
//         println!("Withdrew ${} from {}'s Account", amount, self.owner);
//     }
//     // Balance Checker Funcion 
//     fn check_balance(&self) {
//        println!("This Account is owned by {} and has a balance of ${}", self.owner , self.balance);
//     } 
// }


// reference and pointer
// fn main () {
//     // this is the x value
//     let _x: i32 = 20;
//     // this points towards x and it's value meaning it's changable
//     let _y: &i32 = &_x ;
//     // this just changes the value for z not x
//     let _z: i32 = _y - 10;
//     // This pointer towards why which is a ref to x will change x
//     let changer: i32 = *_y - 5;

//     println!("x is {_x} and y is {_y}");
//     println!("z is {_z}");
//     println!("x is {_x} when un-mutated by z");
//     println!("x is {changer} when mutated");
// }

// If and Else Condition in Rust
// fn main () {
//     let x: i64 = 18;
//     let y: i64 = 18;
// println!("Check if there are two adults");
//     if x >= 18 && y >= 18 {
//         println!("This PersonX({x}) and PersonY({y}) are both adults");
//     }
//     else {
//         println!("Somebody is lying");
//         println!("x's age {x}");
//         println!("y's age {y}");
//     }
// }

fn main() {
    println!("Rust Pratice")
}