impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        
        let mut prefix = String::from(&strs[0]);
        
        for i in 1..strs.len() {
            let mut j = 0;
            let bytes1 = prefix.as_bytes();
            let bytes2 = strs[i].as_bytes();
            
            while j < bytes1.len() && j < bytes2.len() && bytes1[j] == bytes2[j] {
                j += 1;
            }
            
            prefix = prefix[..j].to_string();
            
            if prefix.is_empty() {
                break;
            }
        }
        
        prefix
    }
}