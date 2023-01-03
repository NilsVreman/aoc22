use std::cell::RefCell;
use std::rc::Rc;

/// I know that this solution is currently memory-unsafe. Haven't remade it though.
pub struct Dir {
    files: Vec<File>,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    name: String,
}

impl Dir {
    pub fn new(n: &str) -> Dir {
        Dir {
            files: Vec::new(),
            children: Vec::new(),
            parent: None,
            name: n.to_string(),
        }
    }

    pub fn size(&self) -> u32 {
        self.files.iter().fold(0, |acc, x| acc + x.size) 
            + self.children.iter().fold(0, |acc, x| acc + x.borrow().size())
    }
}

pub struct File {
    pub size: u32,
    pub name: String,
}

impl File {
    pub fn new(line: &str) -> File {
        let s: Vec<&str> = line.split(' ').collect();
        File { 
            size: s[0].parse::<u32>().unwrap(),
            name: s[1].to_string(),
        }
    }
}

pub fn build_tree(c: &parser::Content) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::new("/")));
    let mut current = Rc::clone(&root);

    for cmd in c.content.split("$").skip(2) {
        if &cmd[..3] == " cd" { // if we have a 'cd' command
            let p = cmd.trim().split(' ').collect::<Vec<&str>>();

            if p[1] == ".." {
                let current_clone = Rc::clone(&current); // Clone reference to current
                current = Rc::clone(current_clone
                                    .borrow()
                                    .parent.as_ref()
                                    .unwrap()); // Borrow reference and set current to borrowed
                                                // references parent (i.e., our parent)
            } else {
                let child = Rc::new(RefCell::new(Dir::new(p[1]))); // Create a new reference to a Dir
                current.borrow_mut().children.push(Rc::clone(&child)); // Mutuably borrow the
                                                                       // reference to the current
                                                                       // node, extract children
                                                                       // vec and push a clone of
                                                                       // reference to our child.
                child.borrow_mut().parent = Some(Rc::clone(&current));
                current = child;
            }
            
        } else { // if we have a 'ls' command 
            for line in cmd.lines().skip(1) {
                let p = line.trim().split(' ').collect::<Vec<&str>>();
                if let Ok(x) = p[0].parse::<u32>() {
                    current.borrow_mut()
                        .files.push(File { size: x, name: p[1].to_string() });
                }
            }
        }
    }

    root
}

pub fn size_list(root: &Rc<RefCell<Dir>>, res: &mut Vec<u32>) {
    res.push(root.borrow().size());
    for child in &root.borrow().children {
        size_list(child, res);
    }
}

pub fn sum_below_x(res: &Vec<u32>, x: u32) -> u32 {
    res.iter().filter(|&y| y <= &x).sum()
}

pub fn size_of_dir_to_remove(res: &Vec<u32>, diskspace: u32, unused: u32) -> &u32 {
    let resmax = res.iter().max().unwrap();
    let needed = resmax-(diskspace-unused);
    let mut v: Vec<&u32> = res.iter().filter(|&x| x >= &needed).collect();
    v.sort();
    v[0]
}
