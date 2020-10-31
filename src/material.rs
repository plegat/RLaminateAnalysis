use crate::matrix;

pub struct OrthoMaterial {
    pub name: String ,
    pub ex:f32,
    pub ey:f32,
    pub gxy:f32,
    pub nuxy:f32,
    pub th:f32,
    pub xc:f32,
    pub xt:f32,
    pub yc:f32,
    pub yt:f32,
    pub s:f32,
}

impl OrthoMaterial {

    pub fn new(name:String,ex:f32,ey:f32,gxy:f32,nuxy:f32,th:f32)->OrthoMaterial {
        OrthoMaterial {
            name, ex,ey,gxy,nuxy,th,
            xc:0.0,
            xt:0.0,
            yc:0.0,
            yt:0.0,
            s:0.0,
        }
    }

    pub fn get_data_string(&self) -> String {
        
        return format!("{}, {:.1}, {:.1}, {:.1}",&self.name,&self.ex,&self.ey,&self.gxy)
    }

    pub fn get_name(&self)-> String {
        return format!("{}",&self.name)
    }
}

pub struct Ply<'a> {
    pub material:&'a OrthoMaterial,
    pub orient:f32,
    pub th:f32,
}

impl<'a> Ply<'_> {
    pub fn new(material: &OrthoMaterial, orient:f32, th:f32) -> Ply {
        Ply {
            material,
            orient,
            th,
        }
    }
}

pub struct Laminate<'a> {
    nb:u32,
    stack:Vec<Ply<'a>>,
    abd_matrix: matrix::Matrix,
    pub equiv_prop:Vec<f32>,
}

impl Laminate<'_> {
    pub fn new(material: &OrthoMaterial, orient:Vec<f32>, th:f32)->Laminate {
        let mut plies:Vec<Ply>=Vec::new();
        for i in orient.iter() {
            plies.push(Ply::new(&material,*i,th));
        }

        Laminate{
            nb: orient.len() as u32,
            stack: plies,
            abd_matrix:  matrix::Matrix::new(6,6),
            equiv_prop: Vec::new(),
        }

    }

    pub fn print(&self) {
        println!("number of plies: {}", &self.nb);
        let mut compteur:u32=1;
        for ply in &self.stack {
            let material=ply.material.get_name();
            println!("Ply#{}: {} / angle: {} / thickness: {}",compteur,material,ply.orient,ply.th);
            compteur=compteur+1;
        }
    }

    pub fn print_abd_matrix(&self) {
        println!("ABD matrix:");
        for i in 1..7 {
            for j in 1..7 {
                print!("{:.1}  ",&self.abd_matrix.get_val(i,j));
            }
            println!();
        }

    }

pub fn calculate_abd(&mut self) {

    let mut h:f32=0.0;
    for i in 0..self.nb {
        let ply=&self.stack[i as usize];
        h=h+ply.th;
    }

    println!("epaisseur totale={}",h);
    h=-h/2.0;

    for i in 0..self.nb {
        //println!("calculating ply #{}",i+1);

        let ply=&self.stack[i as usize];
        let mat=ply.material;
        //println!("material {}",mat.get_name());

        // ply matrix in ply coord system

        let mut qlt:matrix::Matrix=matrix::Matrix::new(3,3);
        qlt.set_val(3,3,mat.gxy);

        let nuyx=mat.nuxy*mat.ey/mat.ex;
        qlt.set_val(1,1,mat.ex/(1.0-mat.nuxy*nuyx));
        qlt.set_val(1,2,nuyx*mat.ex/(1.0-mat.nuxy*nuyx));
        qlt.set_val(2,2,mat.ey/(1.0-mat.nuxy*nuyx));
        qlt.set_val(2,1,mat.nuxy*mat.ey/(1.0-mat.nuxy*nuyx));
        
        //qlt.print();

        // ply matrix in laminate coord system

        let mut t=matrix::Matrix::new(3,3);
        let mut t_inv=matrix::Matrix::new(3,3);

        let teta=ply.orient.to_radians();

        let c=teta.cos();
        let s=teta.sin();
        let c2=c*c;
        let s2=s*s;

        t.val=vec!(c2,s2,-2.0*c*s,s2,c2,2.0*c*s,c*s,-c*s,c2-s2);
        t_inv.val=vec!(c2,s2,c*s,s2,c2,-c*s,-2.0*c*s,2.0*c*s,c2-s2);
        
        //let mut qxy=matrix::Matrix::new(3,3);
        let  qxy:matrix::Matrix;

        match qlt.mult(&t_inv) {
            Some(qlt_tinv)=>{
                match t.mult(&qlt_tinv) {
                    Some(t_qlt_tinv)=>{
                        qxy=t_qlt_tinv
                    },
                    None=>panic!("problem while multiplying matrices"),
                };
            },
            None=>panic!("problem while multiplying matrices"),
        }
        
        // updating abd matrix

        let ep:f32=ply.th;

        for i in 1..4 {
            for j in 1..4 {
                &self.abd_matrix.add_val(i,j,((h+ep)-h)*qxy.get_val(i,j)); // update A part
                &self.abd_matrix.add_val(i,j+3,1.0/2.0*((h+ep).powi(2)-h.powi(2))*qxy.get_val(i,j)); // update B part
                &self.abd_matrix.set_val(j+3,i,self.abd_matrix.get_val(i,j+3)); // symetrize B
                &self.abd_matrix.add_val(i+3,j+3,1.0/3.0*((h+ep).powi(3)-h.powi(3))*qxy.get_val(i,j)); // update D part
            }
        }

        h=h+ep;

    }
}

pub fn calculate_equivalent_properties(&mut self) {

    let mut e:f32=0.0;
    
    for ply in &self.stack {
        e=e+ply.th;
    }

    match &self.abd_matrix.invert() {
        Some(abd_inverse)=>{
            //abd_inverse.print();

            // push membrane properties
            &self.equiv_prop.push(1.0/(e*abd_inverse.get_val(1,1)));
            &self.equiv_prop.push(1.0/(e*abd_inverse.get_val(2,2)));
            &self.equiv_prop.push(1.0/(e*abd_inverse.get_val(3,3)));
            &self.equiv_prop.push(-abd_inverse.get_val(1,2)/abd_inverse.get_val(1,1));

            //push bending properties
            &self.equiv_prop.push(12.0/(e.powi(3)*abd_inverse.get_val(4,4)));
            &self.equiv_prop.push(12.0/(e.powi(3)*abd_inverse.get_val(5,5)));
            &self.equiv_prop.push(12.0/(e.powi(3)*abd_inverse.get_val(6,6)));
            &self.equiv_prop.push(-abd_inverse.get_val(4,5)/abd_inverse.get_val(4,4));
            


        },
        None=>println!("Problem during inversion"),
    }
}

pub fn get_membrane_properties(&mut self)->Vec<f32> {
    let mut memb:Vec<f32>=Vec::new();
    for i in 0..4 {
        memb.push(self.equiv_prop[i]);
    }
    memb
}

pub fn get_bending_properties(&mut self)->Vec<f32> {
    let mut bend:Vec<f32>=Vec::new();
    for i in 0..4 {
        bend.push(self.equiv_prop[i+4]);
    }
    bend
}

}
