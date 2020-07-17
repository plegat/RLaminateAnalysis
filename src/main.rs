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
    let rows:u32=10;
    let cols:u32=6;
    

    let mut mat=matrix::Matrix {
        name:String::from("test_matrix"),
        nb_row:rows,
        nb_col:cols,
        val:matrix::null_matrix(rows as usize,cols as usize),
    };

    mat.print();

    println!("remplissage diagonale:");
    mat.fill_diag(1.234);
    mat.print();

}
