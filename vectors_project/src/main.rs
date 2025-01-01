#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    content: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        let new_file = File { name };
        self.content.push(new_file);
    }

    fn delete_file(&mut self, index: usize) -> Option<File> {
        if index < self.content.len() {
            Some(self.content.remove(index))
        } else {
            None
        }
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        if index < self.content.len() {
            // Some(&self.content[index])
            self.content.get(index)
        } else {
            None
        }
    }
}

fn main() {
    let mut folder = Folder::new(String::from("My Folder"));

    // Create files in the folder
    folder.create_file(String::from("file1.txt"));
    folder.create_file(String::from("file2.txt"));

    // Print the folder in debug format
    println!("Folder after file creation: {:?}", folder);

    // Delete a file from the folder
    if let Some(deleted_file) = folder.delete_file(1) {
        println!("Deleted file: {:?}", deleted_file);
    } else {
        println!("No file at the specified index to delete.");
    }

    // Print the folder in debug format after deletion
    println!("Folder after file deletion: {:?}", folder);

    // Get a file from the folder
    match folder.get_file(0) {
        Some(file) => println!("Retrieved file: {:?}", file),
        None => println!("There was no file."),
    }
}
