struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: std::collections::HashMap<Vec<i32>, Vec<String>> =
            std::collections::HashMap::new();
        for s in strs {
            let mut words = vec![0; 26];
            for c in s.chars() {
                words[c as usize - 97] += 1;
            }
            if let Some(v) = map.get_mut(&words) {
                v.push(s);
            } else {
                map.insert(words, vec![s]);
            }
        }
        map.values()
            .map(|x| x.clone())
            .collect::<Vec<Vec<String>>>()
    }
}

#[test]
fn check() {
    assert_eq!('a' as usize, 97);
    assert_eq!('c' as usize - 97, 2);
    assert_eq!('z' as usize - 97, 25);
    // assert_eq!(
    //     Solution::group_anagrams(
    //         vec!["eat", "tea", "tan", "ate", "nat", "bat"]
    //             .into_iter()
    //             .map(|x| x.to_owned())
    //             .collect::<Vec<_>>()
    //     ),
    //     vec![
    //         vec!["ate", "eat", "tea"].to_owned(),
    //         vec!["nat", "tan"].to_owned(),
    //         vec!["bat"].to_owned()
    //     ]
    //     .to_owned()
    // );
}
