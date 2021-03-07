fn main(){
    //println!("Hello, world!");
    
    //another_function();
    another_function(5, 6);

    let x = 5;

    let y =  {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function(x:i32, y:i32){
    //println!("Another function!");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
