impl Solution { 
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) { 
        matrix.reverse(); 

        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            } 
        } 
    } 
}
