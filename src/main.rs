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
    mat.print();
    //println!("val[1,1]={}",mat.get_val(1, 1));
    //mat.set_val(1, 1, 3.14);
    //println!("val[1,1]={}",mat.get_val(1, 1));
    //mat.print();
   
    let mat2=&mat.mult(&mat);
    mat2.print();



}
