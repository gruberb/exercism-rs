use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let points: Vec<i32> = h.keys().cloned().collect();
    let mut res = BTreeMap::new();

    for p in points {
        let chars: Vec<char> = h.get(&p).unwrap().into_iter().map(|c| c.to_ascii_lowercase()).collect();

        for c in chars {
            res.insert(c, p);
        }
    }
    
    res
}
