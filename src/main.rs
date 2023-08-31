//use merkel_cbt::merkel_tree;
// use std::hash;

use sha256::{digest, try_digest, Sha256Digest};
use std::io; 
#[derive(Clone)] 
struct node {  //node struct contains data part, left node, right node
    data:String,
    left_node:Option<Box<node>>,
    right_node:Option<Box<node>>,
}
fn main() {
println!("Please enter the number of leaf nodes"); //user needs to enter the number of leaf nodes to the merkel tree. 
let mut n = String::new();
io::stdin()
       .read_line(&mut n)
       .expect("danger");
    
let n: i32 = n.trim().parse().expect("Entered number is not an integer");


let mut A:Vec <String> = Vec ::new();
for i in 0..n {
    let mut x = String::new(); //In this for loop,we are colecting all the string elements from the user and storing in the array A
    io::stdin()
       .read_line(&mut x)
       .expect("danger");
    A.push(x);

}
if A.len() %2 !=0 {
A.push(A[A.len()-1].clone())} //If length of the A is odd then, appending the last element of the array to it.
let mut B: Vec<String> = Vec::new(); // Making an array B which contains the hash of the strings of A.
for i in 0..A.len(){
    B.push(digest(A[i].clone()));
}
//println!("{:?}",B);
let mut C:Vec<node> = Vec::new();// 
for i in 0..B.len(){
    let n = node {  //creating a node n, whose data part contains hash of the string elements.
        data:B[i].clone(),
        left_node:None,
        right_node:None,
    };
    C.push(n); //C is an array of nodes, Here C contains leaf nodes so it is not having child nodes.

}

let mut D:Vec<node> = Vec::new();
while C.len()!=1 {
    for i in (0..C.len()).step_by(2) {
        let n = node{
            data : digest((C[i].data.clone() + &C[i+1].data.clone())),
            left_node : Some(Box::new(C[i].clone())),
            right_node :Some(Box::new(C[i+1].clone())),

        };
        D.push(n); //Creating an array D, which contains the nodes whose data part is H(concatenaton of data part of child nodes),
        //NOTE: Here length of the D can be even or odd as well.
        
    }
    C = D; //Array C is pointing to an array D and above same thing will repeat till length of the C will be 1. 
    if C.len()%2 ==1 &&C.len()>1{C.push(C[C.len()-1].clone())}  // If the length of the C is odd, we are making it even by appending the last element
    D = Vec::new();
}
println!("merkel root is {}",C[0].data);


}