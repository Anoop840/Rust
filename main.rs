fn main() {
    println!("Hello, world!");
    let a: u64 = 10;
    let b: i32 = 100;
    let c: i32 = 10000;
    println!("the number is {} {} {}",a,b,c);
    let char:char = 'G';
    println!("character is {}",char);
    let float_num : f64 = 3.14;
    println!("float number is {}",float_num);
    const xy:i32 = 10;
    println!("const value {}",xy);
    let my_name:String = String::from("ANOOP");
    let lower_case: String= "Anoop".to_string();
    println!("My name {} and lower case is {}",my_name,lower_case);
    let mut my_vec: Vec<u64>=Vec::new();
    my_vec.push(1);
    my_vec.push(2);
    let length_vector: usize=my_vec.len();
    println!("my vector/array is {:?} and its length is {}",my_vec,length_vector);
    let my_bool:bool = true;
    println!("my boolean value is {}",my_bool);
    struct Rectangle{
        length : u32,
        width : u32,
    }
    let my_rectangle: Rectangle=Rectangle{
        length:10,
        width:15,
    };
    println!("length and width of my rectangle is {} and {}",my_rectangle.length,my_rectangle.width
    );
    let c: u32 = sum(10,20);
    println!("sum of two number is {}",c);
    
    let sub_return:u32 = sub(30,10);
    println!("Sub return is {}",sub_return);

    let mul_return:u32 = mul(10,5);
    println!("Multiplication is {}",mul_return);

    let mod_return:u32 = modulo(9,5);
    println!("Modulus of {}",mod_return);
    
    let my_string : String="abcd".to_string();
    let other_string : String=my_string;
    println!("print my_string {}",my_string);

}

fn sum(a:u32 , b:u32)-> u32{
    let c:u32 = a+b;
    return c;
}

fn sub(a:u32 , b:u32)-> u32{
    a-b
}

fn mul(a:u32 , b:u32) -> u32{
    a*b
}

fn modulo(a:u32,b:u32)-> u32{
    a%b
}