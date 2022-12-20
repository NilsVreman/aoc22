use parser;

fn wrap_idx(idx: isize, value: isize, len: isize) -> isize {
    (idx + value).rem_euclid(len)
}

pub fn create_list(c: &parser::Content) -> Vec<isize> {
    c.content.lines().map(|x| x.parse::<isize>().unwrap()).collect()
}

pub fn execute(list: &Vec<isize>, idxs: &Vec<usize>, num_mix: usize, key: isize) -> isize {
    // Enumerate the list to avoid duplicate numbers in list
    let len   = list.len() as isize;
    let mut v = (0..list.len()).collect::<Vec<_>>();

    for _ in 0..num_mix {
        list.iter().enumerate().for_each(|(i, &x)| {
            let p   = v.iter().position(|&y| y == i).unwrap();
            v.remove(p);
            let np  = wrap_idx(p as isize, x*key, len-1) as usize;
            v.insert(np, i);
        });
    }

    let p = list.iter().position(|&y| y == 0).unwrap();
    let np = v.iter().position(|&y| y == p).unwrap();
    idxs.iter()
        .map(|&x| list[v[wrap_idx(np as isize, x as isize, len) as usize]]*key)
        .sum()
}
