use parser;

fn wrap_idx(idx: isize, value: isize, len: isize) -> isize {
    (idx + value).rem_euclid(len)
}

pub fn create_list(c: &parser::Content) -> Vec<isize> {
    c.content.lines().map(|x| x.parse::<isize>().unwrap()).collect()
}

pub fn execute(list: &Vec<isize>, idxs: &Vec<usize>, num_mix: usize, key: isize) -> isize {
    // Enumerate the list to avoid duplicate numbers in list
    let list  = list.iter().enumerate().collect::<Vec<(usize, &isize)>>();
    let mut v = list.clone();
    let len   = list.len() as isize;

    for _ in 0..num_mix {
        list.iter().for_each(|x| {
            let p   = v.iter().position(|y| y == x).unwrap();
            let np  = wrap_idx(p as isize, v[p].1*key, len-1) as usize;
            let ele = v.remove(p);
            v.insert(np, ele);
        });
    }

    let p = v.iter().position(|&(_, &y)| y == 0).unwrap();
    idxs.iter()
        .map(|&x| v[wrap_idx(p as isize, x as isize, len) as usize].1*key)
        .sum()
}
