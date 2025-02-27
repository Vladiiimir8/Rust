impl Solution { 
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 { 
        if nums.is_empty() { 
            return 0; 
        } 

        let mut k = 1; 

        for i in 1..nums.len() { 
            if nums[i] != nums[k - 1] { 
                nums[k] = nums[i]; 
                k += 1; 
            } 
        } 

        nums.truncate(k);
        
        k as i32 
    } 
}
