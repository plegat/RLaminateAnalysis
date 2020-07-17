pub struct Matrix {
    pub name:String,
    pub nb_row:u32,
    pub nb_col:u32,
    pub val: Vec<f32>,
}

pub fn null_matrix(row:usize,col:usize)->Vec<f32> {
    let val=vec![0f32; row*col];
    return val;
}


impl Matrix{
    
    pub fn print(&self) {
        for n in 0..self.nb_row {
            for c in 0..self.nb_col {
                print!("{} ",&self.val[(n*&self.nb_col+c) as usize]);
            }
            println!();
        }
    }

    pub fn fill_diag(&mut self,val:f32) {
        for i in 0..std::cmp::min(self.nb_row,self.nb_col) {
            self.val[(&self.nb_col*i+i) as usize]=val;
        }
    }


}