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
        return self.val[((row-1)*self.nb_col+(col-1)) as usize];
    }

    pub fn set_val(&mut self,row:u32,col:u32,value:f32) {
        self.val[((row-1)*self.nb_col+(col-1)) as usize]=value;
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

    pub fn get_determinant(&self)->Option<f32> {

        let mut result:Option<f32>=None;

        if self.nb_col==self.nb_row {
            let mut det:f32=0.0;

            if self.nb_row==2 {
                det=self.get_val(1,1)*self.get_val(2,2)-self.get_val(2, 1)*self.get_val(1,2);
            } else {
                for i in 0..self.nb_row-1 {
                    let temp:Matrix=self.extract_matrix(i+1, 1);

                    let det:Option<f32>=temp.get_determinant();
                    match det {
                        Some(x)=>{
                            match result {
                                Some(y)=>result=Some(y+self.get_val(i+1, 1)*(-1f32.powi(i as i32))*x),
                                None=>result=None,
                            };
                        },
                        None=>result=None,
                    };
                    
                    
                }
            }

            result=Some(det);
        }



        return result;
    } 

    pub fn extract_matrix(&self,row:u32,col:u32)->Matrix {
        let mut res=Matrix{
            nb_row:self.nb_row-1,
            nb_col:self.nb_col-1,
            val:null_matrix((self.nb_row-1) as usize,(self.nb_col-1) as usize),
        };


        let mut decal_i:u32=0;
        let mut decal_j:u32=0;


        for i in 0..self.nb_row-2 {
            if i==row-1 {
                decal_i=1;
            }
            decal_j=0;

            for j in 0..self.nb_col-2 {
                if j==col-1 {
                    decal_j=1;
                }
                res.set_val(i+1, j+1, self.get_val(i+decal_i+1, j+decal_j+1));
            }

        }
    
   
    return res;
    }
}