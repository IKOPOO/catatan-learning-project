use std::{
    cell::RefCell,
    fs::{self, File},
    io::{self, Write},
    process::Command,
    rc::{Rc, Weak},
};

// struct untuk folder
#[derive(Debug)]
struct Folder {
    name: String,
}

// struct untuk file
#[derive(Debug)]
struct Filee {
    name: String,
    // size: usize,
}

// enum untuk masuk apakah file atau folder
#[derive(Debug)]
enum FilesystemEntry {
    Folder(Folder),
    File(Filee),
}

// struct untuk membuat node tree
#[derive(Debug)]
struct TreeNode {
    entry: FilesystemEntry,
    child: Option<Vec<Rc<RefCell<TreeNode>>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    // fungsi ini hanya untuk membuat node root/folder root
    // fn new_folder(name: String) -> Rc<RefCell<TreeNode>> {
    //     let folder = Folder { name };
    //     Rc::new(RefCell::new(TreeNode {
    //         entry: FilesystemEntry::Folder(folder),
    //         child: Some(Vec::new()),
    //         parent: None,
    //     }))
    // }

    // fungsi untuk membuat sebuah folder baru
    fn create_folder(
        parent: &Rc<RefCell<TreeNode>>,
        name: String,
        path: &str,
    ) -> Rc<RefCell<TreeNode>> {
        // menggabungkan path sekarang dan nama folder menjadi path lengkap
        let full_path = format!("{}/{}", path, name);

        // membuat path folder secara fisik
        match fs::create_dir(&full_path) {
            Ok(_) => println!("directory berhasil dibuat"),
            Err(_e) => println!("gagal membuat folder"),
        }

        let new_folder = Folder { name: name };
        let new_node = Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::Folder(new_folder),
            child: Some(Vec::new()),
            parent: Some(Rc::downgrade(parent)),
        }));

        let mut parent_borrow = parent.borrow_mut();
        if let Some(ref mut children) = parent_borrow.child {
            children.push(Rc::clone(&new_node));
        }

        Rc::clone(&new_node)

        //  membuat folder di system
    }

    // fungsi untuk membuat sebuah file baru
    fn create_file(
        parent: &Rc<RefCell<TreeNode>>,
        name: String,
        path: &str,
    ) -> Rc<RefCell<TreeNode>> {
        // mendapatkan path lengkap
        let full_path = format!("{}/{}", path, name);

        // membuat file di system
        match File::create(&full_path) {
            Ok(mut file) => {
                println!("file berhasil di simpan");
                writeln!(file, "Ini adalah teks yang ditulis ke dalam file.")
                    .expect("Gagal menulis ke file");
            }
            Err(e) => println!("gagal membuat file {}", e),
        }
        let new_file = Filee { name };
        let new_leaf = Rc::new(RefCell::new(TreeNode {
            entry: FilesystemEntry::File(new_file),
            child: None,
            parent: Some(Rc::downgrade(parent)),
        }));

        let mut parent_borrow = parent.borrow_mut();
        if let Some(ref mut children) = parent_borrow.child {
            children.push(Rc::clone(&new_leaf));
        }

        // membuka file yang dibuat menggunakan text editor
        Command::new("gnome-text-editor")
            .arg(&full_path)
            .status()
            .expect("tidak bisa membuka file");
        Rc::clone(&new_leaf)
    }
    // fungsi menambahkan anak
    fn add_child(
        parent: &Rc<RefCell<TreeNode>>,
        what: String,
        name: String,
        path: &str,
    ) -> Rc<RefCell<TreeNode>> {
        match what.as_str() {
            "folder" => TreeNode::create_folder(parent, name, path),
            "file" => TreeNode::create_file(parent, name, path),
            _ => {
                println!("error blog");
                Rc::clone(parent)
            }
        }
    }

    // fungsi untuk mendapatkan path lengkap dari node/leaf saat ini ke root
    fn get_current_or_full_path(node: &Rc<RefCell<TreeNode>>) -> String {
        let mut path = Vec::new(); // vector untuk menyimpan sementara path lengkap
        let mut current_node = Some(Rc::clone(node)); // variabel sementara untuk menyimpan current path

        // melakukan perulangan ke atas dari current path ke root dan menambahkannya sehingga menjadi path lengkap
        while let Some(n) = current_node.take() {
            let node_borrow = n.borrow();
            match &node_borrow.entry {
                FilesystemEntry::File(file) => {
                    path.push(file.name.clone());
                }
                FilesystemEntry::Folder(folder) => {
                    path.push(folder.name.clone());
                }
            }

            // pindah ke parent selanjutnya
            if let Some(parent_weak) = &node_borrow.parent {
                if let Some(parent_ref) = parent_weak.upgrade() {
                    current_node = Some(parent_ref);
                } else {
                    current_node = None;
                }
            } else {
                current_node = None;
            }
        }

        path.reverse();
        path.join("/")
    }
}

fn garis() {
    let mut a = 0;
    while a < 60 {
        print!("=");
        a += 1;
    }
    println!("");
}
fn single() {
    let mut a = 0;
    while a < 60 {
        print!("_");
        a += 1;
    }
    println!("");
}
fn main() {
    match fs::create_dir("root") {
        Ok(_) => println!("direktori root berhasil dibuat"),
        Err(_e) => println!("direktori root gagal dibuat"),
    }

    let root = Rc::new(RefCell::new(TreeNode {
        entry: FilesystemEntry::Folder(Folder {
            name: "root".to_string(),
        }),
        child: Some(Vec::new()),
        parent: None,
    }));
    let mut current_node = Rc::clone(&root);
        println!("welcome to .......");
        println!("whatever i don't care what you call this program");
        println!("but rust always like a dick");
        println!("fuckkkkkkk youuuuuu");
        garis();

        println!("we have menu : ");
        println!("[-]catatan \n[-]pengingat \n[-]keluar");
        single();
        println!("input : ");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("inputanmu rak genah su");

        let input = user_input.trim();
        loop {
            match input {
                // start match input
                "catatan" => {
                    println!("Buat Catatan \nLihat catatan \nkeluar");
                    single();
                    println!("input : ");

                    // user input
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("ra ono cok jo gendeng o koe su");
                    let user_input = input.trim();

                    // matching user input to pick proses
                    match user_input {
                        // start match user_input
                        "buat" => {
                            println!("Folder \nFile ");
                            let mut input = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .expect("ra ono cok jo gendeng o koe su");
                            let user_input = input.trim();
                            match user_input {
                                "folder" => {
                                    let mut input = String::new();
                                    println!("Masukkan nama folder : ");
                                    io::stdin().read_line(&mut input).expect("cannot read line");
                                    let folder_name = input.trim().to_string();
                                    let current_path =
                                        TreeNode::get_current_or_full_path(&current_node);
                                    current_node = TreeNode::add_child(
                                        &current_node,
                                        "folder".to_string(),
                                        folder_name,
                                        &current_path,
                                    );
                                    let path = TreeNode::get_current_or_full_path(&current_node);
                                    println!("path sekarang : {}", path);
                                }
                                "file" => {
                                    let mut input = String::new();
                                    println!("masukkan nama file {}", input);
                                    io::stdin().read_line(&mut input).expect("cannot read line");
                                    let file_name = input.trim().to_string();
                                    let current_path =
                                        TreeNode::get_current_or_full_path(&current_node);
                                    current_node = TreeNode::add_child(
                                        &current_node,
                                        "file".to_string(),
                                        file_name,
                                        &current_path,
                                    );

                                    let path = TreeNode::get_current_or_full_path(&current_node);
                                    println!("path sekarang : {}", path);
                                }
                                _ => println!("kontrakan"),
                            }
                        }
                        "keluar" => {
                            println!("Semoga kamu tidak lupa :)");
                            break;
                        }
                        _ => println!("kontols"),
                    } // start match user_input
                }
                "keluar" => {
                    println!("SILAHKAN BERKUNJUNG KEMBALI!!");
                    break;
                }
                _ => println!("goblok su aku capek anjg rust memek"),
            } // end match input        
    }
}
