pub struct OrthoMaterial {
    pub name: String ,
    pub ex:f64,
    pub ey:f64,
    pub gxy:f64,
    pub nuxy:f64,
    pub th:f64,
}

impl OrthoMaterial {
    pub fn get_data_string(&self) -> String {
        //return [&self.name,&self.ex.to_string()].join(", ");

        /*
        let mut chaine=self.name.to_string();
        chaine.push_str(", ");
        chaine.push_str(&self.ex.to_string());
        return chaine.to_string()
        */

        return format!("{}, {:.1}, {:.1}, {:.1}",&self.name,&self.ex,&self.ey,&self.gxy)
    }
}
