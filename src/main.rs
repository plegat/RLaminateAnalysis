mod material;
mod matrix;

fn main() {
    println!("Hello, world!");

    
    let mat=material::OrthoMaterial::new("UD carbon PEKK".to_string(),
        135000f32,
        8500f32,
        4500f32,
        0.05f32,
        0.128f32);

    let result=mat.get_data_string();

    print!("description du matériau: ");
    println!("{}",result);

    println!("initialisation de matrice:");
    let rows:u32=6;
    let cols:u32=6;
    
    let ply1=material::Ply::new(&mat,45.0,0.128);

    let stack:Vec<f32>=vec!(45.0,0.0,-45.0,90.0,-45.0,0.0,45.0);
    let mut lam1=material::Laminate::new(&mat, stack, 0.128);

    lam1.print();
    lam1.calculate_abd();
    lam1.print_abd_matrix();

}
