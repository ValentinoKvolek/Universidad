//2- Dado el siguiente struct:
#[derive(PartialEq)]
struct Persona<'a> {
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}
trait VecPersonas<'a>{
    fn listado_salario_mayor(&'a self, num: f64) -> Option<Vec<&'a Persona>>; //b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
    // y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro ciudad.
    fn personas_mayores(&self, num: f64, ciudad : String ) -> Option<Vec<&'a Persona>>; //c- Escriba una función que reciba un vector de personas y un nombre de una ciudad
    // y retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
    //c- Escriba una función que reciba un vector de personas y un nombre de una ciudad
    // y retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
    fn confirmar_ciudad(&self, ciudad : String) -> bool; //d- Escriba una función que reciba un vector de personas y un nombre de una ciudad
    // y retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso contrario
    fn confirmar_almenos_ciudad(&self, ciudad: String) -> bool; //e- Escriba una función que reciba un arreglo de personas y una persona y
    // retorna true si la persona existe en el arreglo, false caso contrario
    fn confirmar_si_persona_existe(&self , persona: &Persona) -> bool; //f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las edades de las personas.\
    fn arreglo_edades(&self)-> Vec<u8>; //g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor salario y
    // la persona con el mayor salario, en caso de que haya más de una persona en cada categoría desempatar por la edad más grande
    fn menos_mayor_salario(&self) -> (Option<&Persona>, Option<&Persona>);
}
impl <'a> VecPersonas<'a> for Vec<Persona<'a>> {

    //a- Escriba una función que reciba un vector de personas y otro parámetro que indica un salario y retorna un listado de personas
    // donde el salario es mayor al parámetro recibido.

    fn listado_salario_mayor(&self, num: f64) -> Option<Vec<&'a Persona>> {
        if num < 0f64 {
            None
        } else {
            Some(self.iter().filter(|x| x.salario > num).collect())
        }
    }

    //b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
    // y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro ciudad.

    fn personas_mayores(&self, num: f64, ciudad: String) -> Option<Vec<&'a Persona>> {
        if num < 0f64 {
            None
        } else {
            Some(self.iter().filter(|x| x.edad > num as u8 && x.ciudad == ciudad).collect())
        }
    }

    //c- Escriba una función que reciba un vector de personas y un nombre de una ciudad
    // y retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
    fn confirmar_ciudad(&self,  ciudad: String)->  bool {
        self.iter().all(|x| x.ciudad == ciudad)
    }

    //d- Escriba una función que reciba un vector de personas y un nombre de una ciudad
    // y retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso contrario

    fn confirmar_almenos_ciudad(&self, ciudad: String) -> bool {
        self.iter().any(|x| x.ciudad == ciudad)
    }

    //e- Escriba una función que reciba un arreglo de personas y una persona y
    // retorna true si la persona existe en el arreglo, false caso contrario

    fn confirmar_si_persona_existe(&self , persona: &Persona) -> bool {
        self.iter().any(|x| x == persona)
    }

    //f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las edades de las personas.\
    fn arreglo_edades(&self)-> Vec<u8>{
        self.iter().filter(|x| x.edad >0).map(|x| x.edad).collect()
    }

    //g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor salario y
    // la persona con el mayor salario, en caso de que haya más de una persona en cada categoría desempatar por la edad más grande

    fn menos_mayor_salario(&self) -> (Option<&Persona>, Option<&Persona>) {

        let menor = self.iter().min_by(|a, b| {
            a.salario
                .partial_cmp(&b.salario)
                .unwrap()
                .then_with(|| b.edad.cmp(&a.edad)) // desempata con mayor edad
        });

        let mayor = self.iter().max_by(|a, b| {
            a.salario
                .partial_cmp(&b.salario)
                .unwrap()
                .then_with(|| b.edad.cmp(&a.edad)) // desempata con mayor edad
        });

        (menor, mayor)
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use crate::{Persona, VecPersonas};
    fn crear_vector_prueba<'a>() -> Vec<Persona<'a>> {
        vec![
            Persona {
                nombre: "Yoni",
                apellido: "Hiriar",
                direccion: "Calle 123",
                ciudad: "Huanguelen",
                salario: 4000.0,
                edad: 30,
            },
            Persona {
                nombre: "Valen",
                apellido: "Franco",
                direccion: "Calle Falsa",
                ciudad: "Córdoba",
                salario: 4200.5,
                edad: 23,
            },
            Persona {
                nombre: "Lucia",
                apellido: "López",
                direccion: "Ruta 9 km 12",
                ciudad: "Rosario",
                salario: 4200.5,
                edad: 21,
            },
            Persona {
                nombre: "Fermin",
                apellido: "Franco",
                direccion: "San Martín 789",
                ciudad: "Mendoza",
                salario: 2800.0,
                edad: 50,
            },
        ]
    }
    #[test]
    fn test_vec_personas(){

        let vp = crear_vector_prueba();

        let vms = vp.listado_salario_mayor(4000.0);

        assert!(vms.is_some());

        let resultado = vms.unwrap();
        assert_eq!(resultado.len(), 2,"valen y lucia");
        assert_eq!(resultado[0].nombre, "Valen", "deberia ser valen ");

        let vcye = vp.personas_mayores(20f64, "Rosario".parse().unwrap());
        assert!(vcye.is_some());
        let resultado = vcye.unwrap();
        assert_eq!(resultado[0].nombre, "Lucia","Deberia ser Lucia");

        let vtc = vp.confirmar_ciudad("Córdoba".to_string());

        assert!(!vtc, "deberia dar False.");

        let vtac = vp.confirmar_almenos_ciudad("Córdoba".to_string());
        assert!(vtac, "deberia dar True.");

        let persona_prueba = Persona {
            nombre: "Yoni",
            apellido: "Hiriar",
            direccion: "Calle 123",
            ciudad: "Huanguelen",
            salario: 4000.0,
            edad: 30,
        };
        let resultado = vp.confirmar_si_persona_existe(&persona_prueba);
        assert!(resultado);
        let persona_prueba2 = Persona {
            nombre: "Nahuel",
            apellido: "Pardo",
            direccion: "Calle 123",
            ciudad: "Lanus",
            salario: 4000.0,
            edad: 30,
        };

        let resultado = vp.confirmar_si_persona_existe(&persona_prueba2);
        assert!(!resultado);

        let edades = vp.arreglo_edades();

        assert_eq!(edades[0] , 30 , "deberia ser 30");
        assert_eq!(edades[1] , 23 , "deberia ser 23");
        assert_eq!(edades.len(), 4);

        let (menor, mayor) = vp.menos_mayor_salario();
        
        assert!(menor.is_some());
        assert!(mayor.is_some());

        let menor = menor.unwrap();
        let mayor = mayor.unwrap();
        
        assert_eq!(menor.nombre, "Fermin");
        assert_eq!(menor.salario, 2800.0);
        
        assert_eq!(mayor.nombre, "Lucia");
        assert_eq!(mayor.salario, 4200.5);
    }
}