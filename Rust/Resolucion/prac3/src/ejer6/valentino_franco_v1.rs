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

struct Examenes{
    nombre_materia:String,nota:f32
}
struct Estudiante{
    nombre:String, id:u16, calificaciones: Vec<Examenes>
}


//estructura que uso para el informe del entregable.
struct Informe{
    nombre:String,
    cant_examenes_rendidos:u32,
    promedio_general: f64,
    nota_mas_alta:u32,
    nombre_materia_mas_alta:String,
    nota_mas_baja:u32,
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

        //mi idea seria retornar una struc de informe el cual devuelva todo lo que pide el enunciado.
        //lo que devulevo es un opcion para manejar los casos de error que puede llegar a tener a la hora de
        //generar un Informe.

        //este if me sirve para poder controlar la cant de examenes rendidos.
        //lo que se me ocurrio es ver mi vector de calificaciones, le cual tiene examenes adentro.
        //si la longitud de mi vector es 0 . Quiere decir que no tengo ningun examen cargado
        //por lo que tendria que devoler none.

        if self.calificaciones.len()==0{
            return None;
        }else { //si esto no pasa empiezo a generar el informe

            // se me ocurrio crear mas funciones
            //las cuales se encarguen de darme el nombre de la materia mas baja. ya que la original no lo hace.

            Informe{
                nombre: self.nombre,
                cant_examenes_rendidos:self.calificaciones.len(),
                promedio_general: self.obtener_promedio(),
                nota_mas_alta: self.obtener_calificacion_mas_alta(),
                nombre_materia_mas_alta: obtener_nombre_materia_mas_alta(),
                nota_mas_baja: self.obtener_calificacion_mas_baja(),
                nombre_materia_mas_baja:self.obtener_nombre_materia_mas_baja(),
            }
        }

    }
    fn obtener_nombre_materia_mas_alta(&self)->String{

        if (self.calificaciones.len()==0) {return "no hay calificaciones".to_string()}

        let mut nota_mas_alta:f32 = -99.0;
        let mut nombre:String;

        for examen in &self.calificaciones {
            if(examen.nota > nota_mas_alta){
                nota_mas_alta = examen.nota;
                nombre:examen.nombre.clone();
            }
        }

        nombre
    }
    fn obtener_nombre_materia_mas_baja(&self)->String{

        if (self.calificaciones.len()==0) {return "no hay calificacion".to_string();}
        let mut nota_mas_baja:f32 = 99.0;
        let mut nombre: String;
        for examen in &self.calificaciones {
            if(examen.nota < nota_mas_baja){
                nota_mas_baja = examen.nota;
                nombre = examen.nombre.clone();
            }
        }
        nombre
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

    //no llegue por le tiempo pero tengo que hacer dos test . uno en el cual ande todo perfecto y pruebe la funcinalidad correcta de  informe
    //y otro para poder probar el caso en los que los examenes son 0).
}
fn main() {}
