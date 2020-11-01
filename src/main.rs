mod material;
mod matrix;

fn main() {
    println!("Hello, world!");

    
    let mut mat=material::OrthoMaterial::new("UD carbon PEKK".to_string(),
        130000f32,
        8000f32,
        4000f32,
        0.35f32,
        0.2f32);

    mat.set_stress_data(800.0, 1200.0, 120.0, 40.0, 100.0);

    let result=mat.get_data_string();

    print!("description du matériau: ");
    println!("{}",result);

    println!("initialisation de matrice:");
    let rows:u32=6;
    let cols:u32=6;
    
    let ply1=material::Ply::new(&mat,45.0,0.2);

    let stack:Vec<f32>=vec!(45.0,0.0,-45.0,90.0,-45.0,0.0,45.0);
    let mut lam1=material::Laminate::new(&mat, stack, 0.2);

    lam1.print();
    lam1.calculate_abd();
    lam1.print_abd_matrix();
    println!("");
    
    lam1.calculate_equivalent_properties();

    println!("membrane properties:");
    let memb=lam1.get_membrane_properties();
    let bend=lam1.get_bending_properties();

    for i in memb {
        print!("{} /",i);
    }
    println!("");
    println!("bending properties:");
    for i in bend {
        print!("{} /",i);
    }
    println!("");
    println!("");
    
    println!("Laminate strains");

    let strains=lam1.calculate_strains(100.0, 0.0,0.0,0.0,0.0,0.0);

    for i in 0..strains.len() {
        let ply_strains:&material::laminate_strain=&strains[i];

        println!{"ply #{}: {:.6} / {:.6} / {:.6} / {:.6} / {:.6} / {:.6} ",i+1,ply_strains.eps_x,ply_strains.eps_y,ply_strains.eps_xy,ply_strains.eps_l,ply_strains.eps_t,ply_strains.eps_lt};

    }

}
