/*

6- Escribir un programa que defina una estructura Estudiante que tenga campos para el nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja
Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen.

*/
mod valentino_franco_v2;

struct Examenes{
    nombre_materia:String,nota:f32
}
struct Estudiante{
    nombre:String, id:u16, calificaciones: Vec<Examenes>
}


//estructura que uso para el informe del entregable.
#[derive(Clone)]
struct Informe{
    nombre:String,
    cant_examenes_rendidos:u32,
    promedio_general: f64,
    nota_mas_alta:f32,
    nombre_materia_mas_alta:String,
    nota_mas_baja:f32,
    nombre_materia_mas_baja:String,
}
// entregable

impl Examenes{
    fn new (nombre_materia:String, nota:f32 ) -> Examenes{
        Examenes{
            nombre_materia,
            nota,
        }
    }
}
impl Estudiante{

    fn new( nombre: String, id:u16, calificaciones: Vec<Examenes> ) -> Estudiante{
        Estudiante{
            nombre,id,calificaciones
        }
    }

    fn obtener_promedio(&self)->f32{

        let mut suma:f32= 0.0;

        for examen in &self.calificaciones {
            suma += examen.nota;
        }

        let promedio = suma/ self.calificaciones.len() as f32;

        promedio

    }

    fn obtener_calificacion_mas_alta(&self)->f32{

        if (self.calificaciones.len()==0) {return 0.0}
        
        let mut nota_mas_alta:f32 = -99.0;

        for examen in &self.calificaciones {
            if(examen.nota > nota_mas_alta){
                nota_mas_alta = examen.nota;
            }
        }

        nota_mas_alta
    }

    fn obtener_calificacion_mas_baja(&self)->f32{

        if (self.calificaciones.len()==0) {return 0.0}
        let mut nota_mas_baja:f32 = 99.0;
        for examen in &self.calificaciones {
            if(examen.nota < nota_mas_baja){
                nota_mas_baja = examen.nota;
            }
        }
        nota_mas_baja
    }

    // entregable
    fn generar_informe(&self)-> Option <Informe>{


        if self.calificaciones.len()==0{
            return None;
        }else {
            let encontrar_max_min = self.encontrar_max_min();
            Some(Informe{
                nombre: self.nombre.clone(),
                cant_examenes_rendidos:self.calificaciones.len() as u32,
                promedio_general: self.obtener_promedio() as f64,
                nota_mas_alta: encontrar_max_min.0,
                nombre_materia_mas_alta: encontrar_max_min.1,
                nota_mas_baja: encontrar_max_min.2,
                nombre_materia_mas_baja:encontrar_max_min.3,
                }
            )
        }
    }
    fn encontrar_max_min(&self)->(f32,String,f32,String){
        let mut max = f32::MIN;
        let mut min= f32::MAX;
        let mut nombre_max = "".to_string();
        let mut nombre_min = "".to_string();

        for examen in &self.calificaciones {
            if(examen.nota > max){
                max = examen.nota;
                nombre_max = examen.nombre_materia.clone();
            }
            if(examen.nota < min){
                min = examen.nota;
                nombre_min = examen.nombre_materia.clone();
            }
        }
        (max,nombre_max,min,nombre_min)

    }
    // entregable


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_crear_estudiante() {

        let examen1 = Examenes::new("OC".to_string(), 8.0);
        let examen2 = Examenes::new("Matematica 2".to_string(), 9.0);
        let nuevo_estudiante = Estudiante::new("Valen".to_string(), 50, vec![examen1, examen2]);

        assert_eq!(nuevo_estudiante.nombre, "Valen");
        assert_eq!(nuevo_estudiante.id, 50);
        assert_eq!(nuevo_estudiante.obtener_calificacion_mas_alta(), 9.0);
        assert_eq!(nuevo_estudiante.obtener_calificacion_mas_baja(), 8.0);
        assert_eq!(nuevo_estudiante.obtener_promedio(), 8.5);
    }
    #[test]
    fn test_generar_informe() {
        let examen1 = Examenes::new("OC".to_string(), 8.0);
        let examen2 = Examenes::new("Matematica 2".to_string(), 9.0);
        let examen3 = Examenes::new("Rust".to_string(), 7.0); //ojala!
        let nuevo_estudiante = Estudiante::new("Valen".to_string(), 50, vec![examen1, examen2, examen3]);
        let informe = nuevo_estudiante.generar_informe();

        assert!(informe.is_some());
        
        let informe = informe.unwrap();
        
        assert_eq!(informe.clone().nombre, "Valen", "deberia ser valen");
        assert_eq!(informe.clone().cant_examenes_rendidos, 3, "deberian ser 3 examenes rendidos");
        assert_eq!(informe.clone().nombre_materia_mas_alta, "Matematica 2", "deberian ser Matematica 2");
        assert_eq!(informe.clone().nombre_materia_mas_baja, "Rust", "deberian ser Rust");
        assert_eq!(informe.clone().promedio_general, 8.0, "deberian ser 8 " );
        
    }
    #[test]
    fn test_generar_informe_sin_examenes() {
        let nuevo_estudiante = Estudiante::new("Valen".to_string(), 50, vec![]);
        let informe = nuevo_estudiante.generar_informe();
        assert!(informe.is_none());
    }

}
fn main() {}
