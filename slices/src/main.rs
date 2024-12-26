fn main() {
    //present in heap memory
  let hero: String = String::from("Salman khan");
  let first_name: &str = &hero[0..6];
  println!("{first_name}");

  let last_name: &str = &hero[7..11];
  println!("{last_name}");

  //present in executable
  let name = {
    let full_name = "Jai Srivastava";
    &full_name[0..3]
  };
  println!("{}",name);

  let emoji: &str = "😂";
  let word: &str = "s";
  println!("emoji takes {}bytes and word take {}byte",emoji.len(), word.len());
}
