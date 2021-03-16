/*
力扣公司的员工都使用员工卡来开办公室的门。每当一个员工使用一次他的员工卡，安保系统会记录下员工的名字和使用时间。如果一个员工在一小时时间内使用员工卡的次数大于等于三次，这个系统会自动发布一个 警告 。

给你字符串数组 keyName 和 keyTime ，其中 [keyName[i], keyTime[i]] 对应一个人的名字和他在 某一天 内使用员工卡的时间。

使用时间的格式是 24小时制 ，形如 "HH:MM" ，比方说 "23:51" 和 "09:49" 。

请你返回去重后的收到系统警告的员工名字，将它们按 字典序升序 排序后返回。

请注意 "10:00" - "11:00" 视为一个小时时间范围内，而 "23:51" - "00:10" 不被视为一小时内，因为系统记录的是某一天内的使用情况。



来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
struct Solution;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map: std::collections::HashMap<String, std::collections::VecDeque<String>> =
            std::collections::HashMap::new();
        let mut res = std::collections::BTreeSet::new();
        let mut tt = key_name.into_iter().zip(key_time).collect::<Vec<_>>();
        tt.sort();
        for (k, t) in tt {
            if let Some(v) = map.get_mut(&k) {
                v.push_back(t.clone());
                if v.len() == 1 {
                    continue;
                }
                let head = v.front().unwrap();
                if !Self::grater_then_one_hour(&t, head) {
                    if v.len() == 3 {
                        res.insert(k);
                    }
                    continue;
                } else {
                    v.pop_front();
                }
            } else {
                let mut v = std::collections::VecDeque::new();
                v.push_back(t);
                map.insert(k.to_owned(), v);
            }
        }
        res.into_iter().collect::<_>()
    }

    fn grater_then_one_hour(l: &str, r: &str) -> bool {
        let (mut ls, mut rs) = (l.split(':'), r.split(":"));
        let (lh, lm) = (
            ls.next().unwrap().parse::<u8>().unwrap(),
            ls.next().unwrap().parse::<u8>().unwrap(),
        );
        let (rh, rm) = (
            rs.next().unwrap().parse::<u8>().unwrap(),
            rs.next().unwrap().parse::<u8>().unwrap(),
        );
        if (lh > rh && lm > rm) || (lh - rh > 1) {
            return true;
        }
        false
    }
}

#[test]
fn check() {
    assert!(!Solution::grater_then_one_hour("12:13", "12:12"));
    assert!(Solution::grater_then_one_hour("13:13", "12:12"));
    assert!(!Solution::grater_then_one_hour("13:13", "12:13"));
    assert!(!Solution::grater_then_one_hour("13:13", "12:14"));
    assert!(Solution::grater_then_one_hour("15:13", "12:14"));

    let key_name = vec!["a", "a", "a", "a", "a", "a", "b", "b", "b", "b", "b"]
        .into_iter()
        .map(|t| t.to_owned())
        .collect::<Vec<String>>();
    let key_time = vec![
        "23:27", "03:14", "12:57", "13:35", "13:18", "21:58", "22:39", "10:49", "19:37", "14:14",
        "10:41",
    ]
    .into_iter()
    .map(|t| t.to_owned())
    .collect::<Vec<String>>();
    assert_eq!(
        Solution::alert_names(key_name, key_time),
        vec!["a".to_owned()]
    );

    let key_name = vec!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
        .into_iter()
        .map(|t| t.to_owned())
        .collect::<Vec<String>>();
    let key_time = vec![
        "10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00",
    ]
    .into_iter()
    .map(|t| t.to_owned())
    .collect::<Vec<String>>();
    assert_eq!(
        Solution::alert_names(key_name, key_time),
        vec!["daniel".to_owned()]
    );

    let key_name = vec![
        "leslie", "leslie", "leslie", "clare", "clare", "clare", "clare",
    ]
    .into_iter()
    .map(|t| t.to_owned())
    .collect::<Vec<String>>();
    let key_time = vec![
        "13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49",
    ]
    .into_iter()
    .map(|t| t.to_owned())
    .collect::<Vec<String>>();
    assert_eq!(
        Solution::alert_names(key_name, key_time),
        vec!["clare".to_owned(), "leslie".to_owned()]
    );
}
