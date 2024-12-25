pub fn group_slice<T: PartialEq>(v: &[T]) -> Vec<(usize, &T)> {
    if v.is_empty() {
        return [].into();
    }

    let mut groups = vec![];

    let mut prev = &v[0];
    let mut count = 1;
    for item in v.iter().skip(1) {
        if item == prev {
            count += 1;
        } else {
            groups.push((count, prev));
            prev = item;
            count = 1;
        }
    }
    groups.push((count, prev));
    groups
}
