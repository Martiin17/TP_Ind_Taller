fn main() {
    let vector_strings  = vec!["ola".to_string(), "7".to_string(), "ok".to_string(), "chau".to_string(), "india".to_string()];
    
    let mut v1: Vec<&String> = vec![&vector_strings[0], &vector_strings[1]];

    let eliminar: String = String::from("eliminar");
    
    let mut v2: Vec<&String> = vec![&vector_strings[2], &vector_strings[3], &eliminar];

    println!("Al principio: {:?}", v1);
    println!("Al principio: {:?}", v2);

    for i in 0..v2.len(){
        if **v2[i] == eliminar{
            v2.remove(i);
            for elem in &v1{
                v2.push(elem);
            }
        }
    }
    /* for elem in &v1{
        v2.push(elem);
    } */

    /* println!("{:?}", v1);
    let nueva_palabra = "coca".to_string();
    v1[0] = &nueva_palabra; */

    println!("{:?}", v1);
    println!("{:?}", v2);
}
