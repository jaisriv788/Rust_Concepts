use std::vec;

fn main() {
    let _vector_type1: Vec<i32> = Vec::new();
    let _vector_type2 = Vec::<i32>::new();
    let mut vector_type3= vec![1, 2, 3, 4, 5];
    vector_type3.push(6);
    vector_type3.insert(2, 9);
    vector_type3.pop();
    vector_type3.remove(3);
    println!("{:? }", vector_type3);

    let vector = vector_type3.get(8);

    match vector {
        Option::Some(value) => println!("The value is present and the value is {value}"),
        Option::None => println!("The value is not present in the vector."),
    }

    //If you want to have a definite set of elements inside the vector then we have to use a function with_capacity as below
    let mut new_vector: Vec<i8> = Vec::with_capacity(4);
    println!("{} {}", new_vector.len(), new_vector.capacity());
    new_vector.push(1);
    new_vector.push(2);
    new_vector.push(3);
    new_vector.push(4);
    println!("{} {}", new_vector.len(), new_vector.capacity());
    new_vector.push(4);
    println!("{} {}", new_vector.len(), new_vector.capacity());
}
