fn calc_prefix_function(src: &[u8]) -> Vec<usize> {
    let src_size = src.len();

    let mut prefix_function = vec![0; src_size];

    for cur_idx in 1..src_size {
        let mut pref_size = prefix_function[cur_idx - 1];
        while pref_size > 0 && src[cur_idx] != src[pref_size] {
            pref_size = prefix_function[pref_size - 1];
        }

        if src[cur_idx] == src[pref_size] {
            pref_size += 1;
        }
        prefix_function[cur_idx] = pref_size;
    }

    prefix_function
}

pub fn knuth_morris_pratt(src: &[u8], pattern: &[u8]) -> Option<usize> {
    let pref = calc_prefix_function(&[pattern, &[0], src].concat());

    pref.iter()
        .skip(pattern.len() + 1)
        .enumerate()
        .find(|(_, &pref_size)| pref_size == pattern.len())
        .map(|(idx, _)| idx + 1 - pattern.len())
}
