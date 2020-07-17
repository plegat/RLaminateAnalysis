pub struct Matrix {
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

    pub fn get_val(&self,row:u32,col:u32)->f32 {
        return self.val[((row-1)*col+(col-1)) as usize];
    }

    pub fn set_val(&mut self,row:u32,col:u32,value:f32) {
        self.val[((row-1)*col+(col-1)) as usize]=value;
    }

    pub fn fill_diag(&mut self,val:f32) {
        for i in 0..std::cmp::min(self.nb_row,self.nb_col) {
            self.val[(&self.nb_col*i+i) as usize]=val;
        }
    }

    pub fn mult(&self,mat:&Matrix)->Matrix {
        let mut res=Matrix{
            nb_row:self.nb_row,
            nb_col:mat.nb_col,
            val:null_matrix(self.nb_row as usize,mat.nb_col as usize),
        };

        for i in 0..self.nb_row {
            for j in 0..mat.nb_col {
                let mut som:f32=0.0;
                for k in 0..self.nb_col {
                    som=som+self.get_val(i+1, k+1)*mat.get_val(k+1, j+1);
                    println!("{}/{}/{}:{}, {} = {}",i,j,k,self.get_val(i+1, k+1),mat.get_val(k+1, j+1),som);
                }
                res.set_val(i+1, j+1, som);
            }
        }


        return res;
    }

}