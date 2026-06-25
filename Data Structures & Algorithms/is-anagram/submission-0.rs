impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { // dose not include the same number of letters. not anagram
            return false
        }
        
        let mut counts = HashMap::new();

        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1; // need to add one to the value of the insert pointer
        }

        for ch in t.chars() {
            let count = counts.entry(ch).or_insert(0);
            * count -=1;

            if *count < 0 { // char does not exist in other string. not anagram
                return false
            }
        }

        counts.values().all(|&count| count == 0) // use the hashmap lookup to see if all values are 0. returns a bool

    }
}
