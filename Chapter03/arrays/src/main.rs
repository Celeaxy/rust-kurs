// Arrays: fixed length at compile time.


fn main() {
    let _array = [1,2,3];

    // Default formating
    println!("{:?}", _array);

    // Pretty formatting
    println!("{:#?}", _array);

    println!("{}", _array[0]);
    println!("{}", _array[1]);
    println!("{}", _array[2]);


    // array initializing
    let _array2 = [0_i32; 10];
    let _array3: [i32; 100] = [1337; 100];

    println!("{:?}", _array2);
    println!("{:?}", _array3);
    
    let _bool_array = [true; 10];
    println!("{:?}", _bool_array);
    
}
