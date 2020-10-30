mod material;
mod matrix;

fn main() {
    println!("Hello, world!");

    
    let mat=material::OrthoMaterial {
        name:"UD carbon PEKK".to_string(),
        ex:135000f64,
        ey:8500f64,
        gxy:4500f64,
        nuxy:0.05f64,
        th:0.135f64,
    };

    let result=mat.get_data_string();

    print!("description du matériau: ");
    println!("{}",result);

    println!("initialisation de matrice:");
    let rows:u32=5;
    let cols:u32=5;
    

    let mut mat=matrix::Matrix {
        nb_row:rows,
        nb_col:cols,
        val:matrix::null_matrix(rows as usize,cols as usize),
    };

    mat.print();

    println!("remplissage diagonale:");
    mat.fill_diag(2.0);
    mat.set_val(1,cols,3.0);
    mat.set_val(rows,1,4.0);
    mat.print();
    //println!("val[1,1]={}",mat.get_val(1, 1));
    //mat.set_val(1, 1, 3.14);
    //println!("val[1,1]={}",mat.get_val(1, 1));
    //mat.print();
   
    let mat2=&mat.mult(&mat);
    println!("matrice multiplication:");
    match mat2 {
        Some(x)=>x.print(),
        None=>println!("Problem on matrices sizes, {} rows for {} lines",&mat.nb_col,&mat.nb_row),
    }
    

    let det=mat.get_determinant();
    match det {
        Some(x)=>println!("determinant: {}",x),
        None=>println!("problem calculating determinant"),
    }
    
    let mat_transpose=mat.transpose();
    mat_transpose.print();
    
    match mat.invert() {
        Some(inverse_mat)=>{
            println!("Matrice inverse:");
            inverse_mat.print();

            println!("Verification:");
            match mat.mult(&inverse_mat) {
                Some(prod)=>prod.print(),
                None=>println!("probleme de multiplication lors de la vérification..."),
            }
             
            


        },
        None=>println!("problème d'inversion de la matrice..."),
    }

}
