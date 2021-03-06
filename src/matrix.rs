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

    pub fn new(nb_row:u32, nb_col:u32)->Matrix {
        Matrix {
            nb_row,
            nb_col,
            val:null_matrix(nb_row as usize,nb_col as usize),
        }
    }
    
    pub fn print(&self) {
        for n in 0..self.nb_row {
            for c in 0..self.nb_col {
                print!("{} ",&self.val[(n*&self.nb_col+c) as usize]);
            }
            println!();
        }
        println!("--");
    }

    pub fn get_val(&self,row:u32,col:u32)->f32 {
        return self.val[((row-1)*self.nb_col+(col-1)) as usize];
    }

    pub fn set_val(&mut self,row:u32,col:u32,value:f32) {
        self.val[((row-1)*self.nb_col+(col-1)) as usize]=value;
    }

    pub fn add_val(&mut self,row:u32,col:u32,value:f32) {
        let old_value=self.get_val(row, col);
        self.val[((row-1)*self.nb_col+(col-1)) as usize]=old_value+value;
    }


    pub fn fill_diag(&mut self,val:f32) {
        for i in 0..std::cmp::min(self.nb_row,self.nb_col) {
            self.val[(&self.nb_col*i+i) as usize]=val;
        }
    }

    pub fn mult_scalar(&mut self,val:f32) {
        for n in 0..self.nb_row {
            for c in 0..self.nb_col {
                let coef=self.get_val(n+1, c+1);
                self.set_val(n+1,c+1,coef*val);
            }
        }
    }

    pub fn mult(&self,mat:&Matrix)->Option<Matrix> {

        if self.nb_col!=mat.nb_row {
            return None;
        }

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
                }
                res.set_val(i+1, j+1, som);
            }
        }

        return Some(res);
    }

    pub fn mult_vector(&self,vect:&Vec<f32>)->Option<Vec<f32>> {

        if self.nb_col!=vect.len() as u32 {
            return None;
        }

        let mut res:Vec<f32>=Vec::new();

        for i in 0..self.nb_row {
                let mut som:f32=0.0;
                for k in 0..self.nb_col {
                    som=som+self.get_val(i+1, k+1)*vect[k as usize];
                }
                res.push(som);
        }

        return Some(res);
    }

    pub fn get_determinant(&self)->Option<f32> {

        let mut result:Option<f32>=Some(0.0);

        if self.nb_col==self.nb_row {
            let mut det:f32=0.0;

            if self.nb_row==2 {
                det=self.get_val(1,1)*self.get_val(2,2)-self.get_val(2, 1)*self.get_val(1,2);
            } else {
                for i in 0_u32..self.nb_row {
                    //println!("loop #{}",i+1);
                    let temp:Matrix=self.extract_matrix(i+1, 1);
                    //temp.print();

                    let det_temp:Option<f32>=temp.get_determinant();
                    
                    match det_temp {
                        Some(x)=>{
                            //println!("det inter: {}",x);

                            match result {
                                Some(_y)=>{
                                    /* println!("i={}",i);
                                    println!("self.get_val(i+1, 1)={}",self.get_val(i+1, 1));
                                    println!("(-1.0_f32.powi((i+2) as i32))={}",((-1.0_f32).powi((i+2) as i32)));
                                    println!("(i+2) as i32={}",(i+2) as i32);
                                    println!("x={}",x); */
                                    det=det+self.get_val(i+1, 1)*((-1.0_f32).powi((i+2) as i32))*x;
                                    //println!("det={}",det);
                                    //result=Some(y+self.get_val(i+1, 1)*(-1f32.powi((i+2) as i32))*x)
                                },
                                None=>result=None,
                            };
                        },
                        None=>result=None,
                    };
                }
            }

            result=Some(det);
        } else {
            result=None;
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
        let mut decal_j:u32;


        for i in 0..self.nb_row-1 {
            if i==row-1 {
                decal_i=1;
            }
            decal_j=0;

            for j in 0..self.nb_col-1 {
                if j==col-1 {
                    decal_j=1;
                }
                res.set_val(i+1, j+1, self.get_val(i+decal_i+1, j+decal_j+1));
            }

        }
   
    return res;
    }

    pub fn transpose(&self)->Matrix {
        let mut res=Matrix{
            nb_row:self.nb_col,
            nb_col:self.nb_row,
            val:null_matrix((self.nb_col) as usize,(self.nb_row) as usize),
        };

        for i in 0..self.nb_row {
            for j in 0..self.nb_col {
                res.set_val(j+1,i+1,self.get_val(i+1,j+1));
            }
        }

        res

    }

    pub fn invert(&self)->Option<Matrix> {

        let mut result:Option<Matrix>=None;

        if self.nb_col==self.nb_row {

            let mut inverse=Matrix{
                nb_row: self.nb_col,
                nb_col: self.nb_row,
                val:null_matrix((self.nb_col) as usize,(self.nb_row) as usize),
            };

            match self.get_determinant() {
                Some(det)=> {
                    for i in 0_u32..self.nb_row {
                        for j in 0_u32..self.nb_col {
                            let mat_temp=self.extract_matrix(i+1, j+1);
                            

                            match mat_temp.get_determinant() {
                                Some(det_temp)=> {
                                    inverse.set_val(i+1,j+1,(-1.0_f32).powi((i+j) as i32)*det_temp);
                                },

                                None=>panic!("problem while calculating inverse matrix"),
                            }

                            
                        }
                    }

                    inverse=inverse.transpose();

                    inverse.mult_scalar(1.0/det);

                    result=Some(inverse);

                },

                None=>result=None,
            }

            

        }

        return result;
    }

}