    // let mut result : f32 = 0.0; //int
    // let x:i32 = 5; //float 
    // result = result + x as f32; // no implicit conversion

    // println!("{}",result);


fn double(x:i32) -> i32 {

    // return x*2;
    {
        let y = 10;
        x*2*y
    }

}


fn main() {
    
    println!("Double {} equals to {}", 5,double(5));

}
