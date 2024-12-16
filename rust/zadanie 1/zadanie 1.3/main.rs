impl Solution { 
    pub fn max_area(height: Vec<i32>) -> i32 { 
        let mut max_area = 0; 
        let mut left = 0; 
        let mut right = height.len() - 1; 

        while left < right { 

            let min_height = height[left].min(height[right]); 
            let width = (right - left) as i32; 
            let area = min_height * width; 
            max_area = max_area.max(area); 

            if height[left] < height[right] { 
                left += 1; 
            } else { 
                right -= 1; 
            } 
        } 
        max_area 
    } 
}