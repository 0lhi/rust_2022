#[derive(Debug, Clone)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
    fsize: u64,
    tsize: u64,
}

impl Folder {
    fn iter(&self) -> FolderIterator {
        FolderIterator {
            cabinet: vec![self],
        }
    }
}

struct FolderIterator<'a> {
    cabinet: Vec<&'a Folder>,
}

impl<'a> Iterator for FolderIterator<'a> {
    type Item = &'a Folder;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cabinet.pop() {
            Some(f) => {
                for sf in f.folders.iter() {
                    self.cabinet.push(sf);
                }
                Some(f)
            }
            None => None,
        }
    }
}

// impl Folder {
//     fn get_size(s: Self, &mut l: &mut Location, &mut m: &mut HashMap<Location, DirIndex>) -> u64 {
//         let refer: &mut Folder = find_folder(&mut s, m.get(&l).unwrap().clone());
//         64
//     }
// }

// #[derive(Debug)]
// struct Example {
//     v: Vec<RcEx>,
// }

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct File {
    size: u64,
    name: String,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Terminal {
    cd(String),
    ls,
    dir(String),
    NUM(File),
}

type Location = Vec<String>;
// type RcEx = Rc<RefCell<Example>>;
// type RcLoc = Rc<RefCell<Location>>;
type DirIndex = Vec<usize>;

// struct Key {
//     loc: RcLoc,
//     dir: Option<String>,
// }

// use std::rc::Rc;
use std::collections::HashMap;
use Terminal::{cd, dir, ls, NUM};

// fn main() {
//     let mut vec_tor: Vec<Example> = vec![];
//     vec_tor.insert(101, Example { v: vec![] });
//     vec_tor[101].v.insert(2, Example { v: vec![] });
//     vec_tor[101].v[2].v.insert(6, Example { v: vec![] });
//     let sth = "vec_tor[101].shitty[2].shitty[6]".to_string();
//     //println!("{:?}", vec_tor[101].v[2].v[6])
// }

// fn maintwo() {
//     let mut example: Example = Example { v: vec![] };
//     let example2: Example = Example { v: vec![] };
//     let example3: RcEx = Rc::new(RefCell::new(example2));
//     example.v.push(example3.clone());
//     let value_string = "This is what needs to be printed.";
//     let sth = "value_string".to_string();
//     //println!("\n{example3:?}\n\n{example:?}",)
// }

fn main() {
    let itemlist: Vec<String> = std::fs::read_to_string("Input.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.to_string().replace("$ ", ""))
        .collect();
    //println!("{itemlist:?}");
    let mut terminal: Vec<Terminal> = vec![];
    for line in itemlist {
        terminal.push(match line.chars().next().unwrap() {
            'c' => cd(line.split(' ').nth(1).unwrap().to_string()),
            'l' => ls,
            'd' => dir(line.split(' ').nth(1).unwrap().to_string()),
            _ => NUM(File {
                size: line.split(' ').next().unwrap().to_string().parse().unwrap(),
                name: line.split(' ').nth(1).unwrap().to_string(),
            }),
        })
    }
    let mut root: Vec<Folder> = vec![Folder {
        name: "/".to_string(),
        files: vec![],
        folders: vec![],
        fsize: 0,
        tsize: 0,
    }];
    // let mut was_ls = false;
    let mut location: Location = vec!["/".to_string()];
    // let mut current_dir = root
    let mut map: HashMap<Location, DirIndex> = HashMap::new();
    // let mut sum: usize = 0;
    map.insert(location.clone(), vec![root.len() - 1]);
    for line in terminal {
        match line {
            cd(c) => {
                (if c != ".." && c != "/" {
                    // dbg!(&location, &map);
                    let refer: &mut Folder =
                        find_folder(&mut root[0], map.get(&location).unwrap().clone());

                    refer.folders.push(Folder {
                        name: c.clone(),
                        files: vec![],
                        folders: vec![],
                        fsize: 0,
                        tsize: 0,
                    });

                    let mut parent = map.get(&location).unwrap().clone();
                    parent.push(refer.folders.len() - 1);

                    location.push(c.clone());

                    map.insert(location.clone(), parent);
                } else if c == ".." {
                    let mut folder: &mut Folder =
                        find_folder(&mut root[0], map.get(&location).unwrap().clone());
                    let tsize = folder.tsize;
                    // let location_clone = location.clone();
                    location.pop();
                    folder = find_folder(&mut root[0], map.get(&location).unwrap().clone());
                    folder.tsize += tsize;
                    //println!("Location: {location:?}, Clone: {location_clone:?}");

                    //println!("Folder: {folder:?}")
                });
                //println!("{c} hahaha {location:?}")
            }
            ls => (), //was_ls = true,
            dir(_) => (),
            NUM(f) => {
                let refer: &mut Folder =
                    find_folder(&mut root[0], map.get(&location).unwrap().clone());
                refer.files.push(f.clone());
                refer.fsize += f.size;
                refer.tsize += f.size
            }
        }
    }
    //println!("This is root: {root:?}");
    println!("This is map: {map:?} ");
    for _ in 0..location.len() - 1 {
        let mut folder: &mut Folder =
            find_folder(&mut root[0], map.get(&location).unwrap().clone());
        let tsize = folder.tsize;
        //println!("{tsize}");
        // let location_clone = location.clone();
        location.pop();
        folder = find_folder(&mut root[0], map.get(&location).unwrap().clone());
        folder.tsize += tsize;
        //println!("Location: {location:?}, Clone: {location_clone:?}");

        //println!("Folder: {folder:?}")
    }
    // println!("Location: {:?} Tsize Root: {:?}", location, root[0].tsize,);
    println!("{root:?}");

    let root_readonly = root[0].clone();
    let mut total_less: u64 = 0;
    let mut options: Vec<u64> = vec![];
    for folder in root_readonly.iter() {
        let total = folder.tsize;
        if total <= 100000 {
            total_less += total;
            println!("Small enough: {:?} -> {}", folder.name, total);
            options.push(total);
        } else {
            println!("Too big: {:?} -> {}", folder.name, total);
            options.push(total);
        }
    }
    println!("Result should be: {:?}", total_less);
    options.sort();
    // Start Part 2:
    let root_size = options[options.len() - 1];
    let free_space = TOTAL - root_size;
    let to_free = LEAST - free_space;
    println!(
        "index1: {} size root: {}, free space {}, to free {}",
        options[1], root_size, free_space, to_free
    );
    // options.reverse();
    // loop {
    //     if options.last().unwrap() < &to_free {
    //         options.pop();
    //     } else {
    //         println!("Result is: {}", options.last().unwrap());
    //         break;
    //     }
    // }
    for option in options {
        if option >= to_free {
            println!("Result is: {}", option);
            break;
        }
    }
}

const TOTAL: u64 = 70000000;
const LEAST: u64 = 30000000;

fn find_folder(mut r: &mut Folder, mut d: DirIndex) -> &mut Folder {
    d.remove(0);
    for u in d {
        //println!("OOOOOOOO {u:?}");
        r = r.folders.get_mut(u).unwrap();
    }
    r
}

// fn recurse(f: Folder) -> DirIndex {
//     let folder: Folder = f;
//     folder
// }

// #[derive(Debug)]
// enum Folder {
//     Directory(Vec<(String, Box<Folder>)>),
//     File(Vec<(String, i32)>),
//     Empty,
// }

// use Folder::{Directory, Empty, File};

// fn main() {
//     let itemlist = std::fs::read_to_string("Example.txt").unwrap();
//     //println!("{itemlist}");
//     let mut hdd: Folder = Directory(vec![("a".to_string(), Box::new(Empty))]);
//     let test: Vec<&str> = itemlist.split("\n").collect();
//     //println!("{test:?}");
//     //println!("{hdd:?}");
// }

// fn main_3() {
//     let itemlist: Vec<String> = std::fs::read_to_string("Example.txt")
//         .unwrap()
//         .split("\n")
//         .map(|x| x.to_string().replace("$ ", ""))
//         .collect();
//     //println!("{itemlist:?}");
//     let mut terminal: Vec<Terminal> = vec![];
//     for line in itemlist {
//         terminal.push(match line.chars().next().unwrap() {
//             'c' => cd(line.split(' ').nth(1).unwrap().to_string()),
//             'l' => ls,
//             'd' => dir(line.split(' ').nth(1).unwrap().to_string()),
//             _ => NUM(File {
//                 size: line.split(' ').next().unwrap().to_string().parse().unwrap(),
//                 name: line.split(' ').nth(1).unwrap().to_string(),
//             }),
//         })
//     }
//     let mut root: Vec<Folder> = vec![];
//     let mut was_ls: bool;
//     let mut location: Location = vec![];
//     let mut cursor = Folder {
//         name: "".to_string(),
//         files: vec![],
//         folders: vec![],
//     };
//     let mut map: HashMap<Location, DirIndex> = HashMap::new();
//     for line in terminal {
//         match line {
//             cd(c) => {
//                 (if c != ".." {
//                     // root.push(Folder {
//                     //     name: c.clone(),
//                     //     files: vec![],
//                     //     folders: vec![],
//                     // });
//                     if cursor.name != "" {
//                         location.push(cursor)
//                     }
//                     cursor = Folder {
//                         name: c.clone(),
//                         files: vec![],
//                         folders: vec![],
//                     };
//                     map.insert(location.clone(), vec![root.len()]);
//                 } else {
//                     location.pop();
//                 });
//                 //println!("{c} hahaha {location:?}")
//             }
//             ls => (),
//             dir(_) => (),
//             NUM(f) => (//println!("This is a file: {f:?} in {location:?}")),
//         }
//     }
//     //println!("This is root: {root:?}");
// }
