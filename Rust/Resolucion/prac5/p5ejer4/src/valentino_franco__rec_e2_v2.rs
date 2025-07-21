

use std::cmp::{PartialOrd};
use std::collections::{HashMap};

use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};


const NOMBRE_MESES: [&str; 12]=["Enero", "Febrero", "Marzo",
    "Abril", "Mayo", "Junio","Julio","Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Clone, Debug, Default, PartialOrd ,PartialEq,Serialize,Deserialize)]
pub struct Fecha{
    pub dia:u8, pub mes:u8, pub anio:u16
}

fn main() {

}

impl Fecha {
    pub fn new(dia: u8, mes: u8, anio: u16) -> Fecha {

        Fecha {
            dia,
            mes,
            anio
        }

    }

    pub fn es_fecha_valida(&self) -> bool {

        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

        true
    }

    pub fn sumar_dias(&mut self, mut dias: u8) {

        while dias > 0 {
            let dias_restantes_mes = self.dias_mes_actual() - self.dia;

            if dias > dias_restantes_mes {
                dias -= dias_restantes_mes + 1;
                self.dia = 1;
                self.mes += 1;

                if self.mes > 12 {
                    self.mes = 1;
                    self.anio += 1;
                }

            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u8) {
        while dias > 0 {

            if dias >= self.dia {
                dias -= self.dia;

                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                self.dia = self.dias_mes_actual();

            } else {
                self.dia -= dias;
                dias = 0;
            }
        }
    }

    pub fn es_mayor(&self, otra: Fecha) -> bool {
        if self.anio > otra.anio ||
            (self.anio == otra.anio && self.mes > otra.mes) ||
            (self.anio == otra.anio && self.mes == otra.mes && self.dia > otra.dia) {
            return true;
        }
        false
    }

    pub fn dias_mes_actual(&self) -> u8 {
        match self.mes  {
            4| 6 | 9| 11 => 30, // si es alguno de estos meses tiene 30 dias
            2=> if self.es_bisiesto() {29} else {28},
            _ => 31,
        }
    }

    pub fn es_bisiesto(&self)-> bool{
        self.anio % 4 == 0}
}

#[cfg(test)]
mod tests_fecha {
    use super::*;

    #[test]
    fn test_fecha_valida() {
        let f = Fecha::new(29, 2, 2024); // bisiesto
        assert!(f.es_fecha_valida());
    }

    #[test]
    fn test_fecha_invalida_mes() {
        let f = Fecha::new(10, 13, 2024);
        assert!(!f.es_fecha_valida());
    }

    #[test]
    fn test_fecha_invalida_dia() {
        let f = Fecha::new(31, 4, 2024); // abril tiene 30 días
        assert!(!f.es_fecha_valida());
    }

    #[test]
    fn test_sumar_dias_con_salto_de_mes() {
        let mut f = Fecha::new(28, 2, 2023); // no bisiesto
        f.sumar_dias(3);
        assert_eq!(f.dia, 3);
        assert_eq!(f.mes, 3);
    }

    #[test]
    fn test_sumar_dias_con_salto_de_anio() {
        let mut f = Fecha::new(31, 12, 2023);
        f.sumar_dias(1);
        assert_eq!(f.dia, 1);
        assert_eq!(f.mes, 1);
        assert_eq!(f.anio, 2024);
    }

    #[test]
    fn test_restar_dias_simple() {
        let mut f = Fecha::new(10, 3, 2024);
        f.restar_dias(5);
        assert_eq!(f.dia, 5);
        assert_eq!(f.mes, 3);
    }

    #[test]
    fn test_restar_dias_con_salto_de_mes() {
        let mut f = Fecha::new(1, 3, 2024); // bisiesto
        f.restar_dias(1);
        assert_eq!(f.dia, 29);
        assert_eq!(f.mes, 2);
    }

    #[test]
    fn test_es_mayor_true() {
        let f1 = Fecha::new(10, 5, 2024);
        let f2 = Fecha::new(5, 5, 2024);
        assert!(f1.es_mayor(f2));
    }

    #[test]
    fn test_es_mayor_false() {
        let f1 = Fecha::new(5, 5, 2024);
        let f2 = Fecha::new(10, 5, 2024);
        assert!(!f1.es_mayor(f2));
    }
}

const BASE_FOLDER: &str ="D:/Universidad/Rust/Resolucion/prac5/p5ejer4/";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct  Biblioteca{
    path: String,
    nombre:String,
    direccion:String,
    copia_libros: HashMap<u64 , u64>,
    prestamos:Vec<Prestamo>,
}
#[derive(PartialEq, Clone, Debug,Serialize, Deserialize)]
enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
enum Estado{
    Devuelto,
    Prestado,
}
#[derive(PartialEq, Clone, Debug,Serialize, Deserialize)]
struct Libro{
    isnb: u64,
    titulo: String,
    autor: String,
    numero_paginas: u32,
    genero: Genero,
    cant_copia: u64
}


#[derive(PartialEq, Clone, Debug,Serialize, Deserialize)]
struct Prestamo{
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct  Cliente{
    dni:u128,
    nombre: String,
    telefono: String,
    direccion: String,
    correo:String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorObtenerCantCopias{
    LibroNoTieneCopias,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorDecrementarCantCopias{
    LibroNoEncontrado,
    NoTieneCopiasParaDecrementar,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorIncrementarCantCopias{
    LibroNoEncontrado,
}

pub enum ErrorContarPrestamo{
   ClienteSinPrestamos,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorRealizarPrestamo{
    ElClienteTieneElMaximoDePrestamos,
    LibroNoCuentaConCopias,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorPrestamos{
    NoHayPrestamos,
    PrestamoNoExiste,

}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorDevolverLibro{
    
    LibroNoEncontrado,
}


#[derive(Debug)]
pub enum ErrorHistorialPrestamos{
    ClienteSinPrestamos,
}
pub enum FiltroPrestamos{
    todos,
    SoloDevueltos,
    SoloPrestados,
}


impl Biblioteca{
    fn reescribir_archivo_json(&self, path: String)-> bool {

        let mut file = match File::create(format!("{}biblioteca.json", path)) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        let json_data = match serde_json::to_string_pretty(self) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        match file.write(json_data.as_bytes()) {
            Ok(res) => { true },
            Err(_) => { false}
        }
    }
    
    fn obtener_cant_copias(&self, libro: &Libro) -> Result<u64, ErrorObtenerCantCopias> {
        match self.copia_libros.get(&libro.isnb) {
            Some(&cantidad) => Ok(cantidad),
            None => Err(ErrorObtenerCantCopias::LibroNoTieneCopias),
        }
    }
    fn decrementar_cant_copias(&mut self, libro: &Libro) -> Result<u64, ErrorDecrementarCantCopias> {

        let nuevas_copias = {
            match self.copia_libros.get_mut(&libro.isnb) {
                Some(encontrado) => {
                    if *encontrado == 0 {
                        return Err(ErrorDecrementarCantCopias::NoTieneCopiasParaDecrementar);
                    } else {
                        *encontrado -= 1;
                        Ok(*encontrado)
                    }
                }
                None => return Err(ErrorDecrementarCantCopias::LibroNoEncontrado),
            }
        }?;

        self.reescribir_archivo_json(self.path.to_string());
        Ok(nuevas_copias)
    }
    fn incrementar_cant_copias(&mut self, libro: &Libro)-> Result<u64, ErrorIncrementarCantCopias> {
        let nueva_copias = {
            match self.copia_libros.get_mut(&libro.isnb) {
                Some(encontrado) => {
                    *encontrado += 1;
                    Ok(*encontrado)
                }
                None => Err(ErrorIncrementarCantCopias::LibroNoEncontrado)
            }
        }?;
        self.reescribir_archivo_json(self.path.to_string());
        Ok(nueva_copias)
    }
    fn contar_prestamos(&self, nombre:String, telefono:String, direccion:String) -> u32 {
        let mut cant_p = 0;
        for i in &self.prestamos {
            if i.cliente.nombre == nombre &&
                i.cliente.telefono == telefono &&
                i.cliente.direccion == direccion &&
                i.estado == Estado::Prestado {
                cant_p += 1;
            }
        }
        cant_p
    }
    fn realizar_prestamo(&mut self, cliente_p: Cliente, libro_p: &Libro, fecha_pv: Fecha)-> Result <bool, ErrorRealizarPrestamo> {

        if self.contar_prestamos(cliente_p.clone().nombre, cliente_p.clone().telefono, cliente_p.clone().direccion) >= 5 {
            return Err(ErrorRealizarPrestamo::ElClienteTieneElMaximoDePrestamos);
        }

        let cantcopias = self.copia_libros.get(&libro_p.isnb);


        if (cantcopias.is_some()) && (*cantcopias.unwrap() == 0) {
            return Err(ErrorRealizarPrestamo::LibroNoCuentaConCopias)
        }

        let nuevo_prestamo = Prestamo {
            libro: libro_p.clone(),
            cliente: cliente_p,
            fecha_vencimiento: fecha_pv,
            fecha_devolucion: None,
            estado: Estado::Prestado,
        };

        self.prestamos.push(nuevo_prestamo);
        self.decrementar_cant_copias(&libro_p);
        Ok(true)
    }
    fn ver_prestamos_a_vencer(&self, hoy:Fecha, dias:u8)-> Result< Vec<&Prestamo>, ErrorPrestamos>{

        let mut  prestamos_por_vencer = Vec::<&Prestamo>::new();
        let mut fecha_limite = hoy;
        fecha_limite.sumar_dias(dias);
        //cuando yo recorro un vec, esto me da el "prestamo" de lo que tiene adentro.
        for prestamo in &self.prestamos {
            if prestamo.estado == Estado::Prestado {
                let mut vencimiento = prestamo.fecha_vencimiento.clone();
                if vencimiento <= fecha_limite {
                    prestamos_por_vencer.push(prestamo);
                }
            }
        }
        if prestamos_por_vencer.is_empty(){
            return Err(ErrorPrestamos::NoHayPrestamos)
        }
        Ok(prestamos_por_vencer)
    }
    fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Result<Vec<&Prestamo>,ErrorPrestamos>{

        let mut prestamos_vencidos = Vec::<&Prestamo>::new();

        for prestamo in &self.prestamos {
            if (prestamo.fecha_vencimiento < fecha_actual){
                prestamos_vencidos.push(prestamo);
            }
        }
        if prestamos_vencidos.is_empty(){
            return Err(ErrorPrestamos::NoHayPrestamos)
        }
        Ok(prestamos_vencidos)
    }
    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) ->Result<Prestamo , ErrorPrestamos> {
        match  self.prestamos.iter().find(|&prestamo| prestamo.cliente == *cliente && prestamo.libro.isnb == libro.isnb) { 
            Some(prestamo) => {
                Ok(prestamo.clone())
            }
            None=>Err(ErrorPrestamos::PrestamoNoExiste)
        }      
    }
    fn devolver_libro(&mut self, libro: Libro, cliente: Cliente, hoy:Fecha)-> Result <bool, ErrorDevolverLibro> {
        for prestamo in self.prestamos.iter_mut() {
            if prestamo.cliente == cliente && prestamo.libro.isnb == libro.isnb{
                prestamo.estado = Estado::Devuelto;
                prestamo.fecha_devolucion = Some(hoy.clone());
                self.reescribir_archivo_json(self.path.parse().unwrap());
                return Ok(true)
            }
        }

        Err(ErrorDevolverLibro::LibroNoEncontrado)
    }

     fn get_historial_prestamos(&self, id_cliente: u128, filtro_estado: FiltroPrestamos) ->  Result<Vec<Prestamo>, ErrorHistorialPrestamos> {

         let prestamos_filtrados: Vec<Prestamo> = self.prestamos.iter().filter(|p| {
                 p.cliente.dni == id_cliente && match filtro_estado {
                     FiltroPrestamos::todos => true,
                     FiltroPrestamos::SoloPrestados => p.estado == Estado::Prestado,
                     FiltroPrestamos::SoloDevueltos => p.estado == Estado::Devuelto,
                 }
             })
             .map(|p| Prestamo {
                 libro: p.libro.clone(),
                 cliente: p.cliente.clone(),
                 fecha_vencimiento: p.fecha_vencimiento.clone(),
                 fecha_devolucion: p.fecha_devolucion.clone(),
                 estado: p.estado.clone()
             })
             .collect();

         //se me hace mas como devolver todo el prestamo de un cliente.

         if prestamos_filtrados.is_empty(){
             return Err(ErrorHistorialPrestamos::ClienteSinPrestamos)
         }
         Ok(prestamos_filtrados)

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn biblioteca_de_prueba() -> Biblioteca {
    let cliente = Cliente {
        dni:4909834,
        nombre: "Valentino".to_string(),
        telefono: "123456".to_string(),
        direccion: "Calle falsa 123".to_string(),
        correo: "valen@gmail.com".to_string(),
    };

    let libro1 = Libro {
        isnb: 111,
        titulo: "El Principe".to_string(),
        autor: "Nicolas Maquiavelo".to_string(),
        numero_paginas: 300,
        genero: Genero::Novela,
        cant_copia: 3,
    };
    let libro2 = Libro {
        isnb: 222,
        titulo: "Clean Code".to_string(),
        autor: "Robert C. Martin".to_string(),
        numero_paginas: 50,
        genero: Genero::Tecnico,
        cant_copia: 2,
    };

    let fecha_venc = Fecha::new(25, 6, 2025);
    let prestamo = Prestamo {
        libro: libro1.clone(),
        cliente: cliente.clone(),
        fecha_vencimiento: fecha_venc,
        fecha_devolucion: None,
        estado: Estado::Prestado,
    };

    let mut copia_libros = HashMap::new();
    copia_libros.insert(libro1.isnb, 2);
    copia_libros.insert(libro2.isnb, 2);

    Biblioteca {
        path : BASE_FOLDER.to_string(),
        nombre: "Biblio Unlp".to_string(),
        direccion: "Av 50".to_string(),
        copia_libros,
        prestamos: vec![prestamo],
    }
}
    fn biblioteca_de_prueba_path_error() -> Biblioteca {
        let cliente = Cliente {
            dni:4909834,
            nombre: "Valentino".to_string(),
            telefono: "123456".to_string(),
            direccion: "Calle falsa 123".to_string(),
            correo: "valen@gmail.com".to_string(),
        };

        let libro1 = Libro {
            isnb: 111,
            titulo: "El Principe".to_string(),
            autor: "Nicolas Maquiavelo".to_string(),
            numero_paginas: 300,
            genero: Genero::Novela,
            cant_copia: 3,
        };
        let libro2 = Libro {
            isnb: 222,
            titulo: "Clean Code".to_string(),
            autor: "Robert C. Martin".to_string(),
            numero_paginas: 50,
            genero: Genero::Tecnico,
            cant_copia: 2,
        };

        let fecha_venc = Fecha::new(25, 6, 2025);
        let prestamo = Prestamo {
            libro: libro1.clone(),
            cliente: cliente.clone(),
            fecha_vencimiento: fecha_venc,
            fecha_devolucion: None,
            estado: Estado::Prestado,
        };

        let mut copia_libros = HashMap::new();
        copia_libros.insert(libro1.isnb, 2);
        copia_libros.insert(libro2.isnb, 2);

        Biblioteca {
            path : "noexiste/hola".to_string(),
            nombre: "Biblio Unlp".to_string(),
            direccion: "Av 50".to_string(),
            copia_libros,
            prestamos: vec![prestamo],
        }
    }

    #[test]
    fn test_obtener_cant_copias() {
        let biblio = biblioteca_de_prueba();
        let libro = biblio.prestamos[0].libro.clone();

        let res = biblio.obtener_cant_copias(&libro);
        assert_eq!(res, Ok(2));

        let libro_falso = Libro {
            isnb: 999,
            ..libro.clone()
        };
        let res2 = biblio.obtener_cant_copias(&libro_falso);
        assert!(matches!(res2, Err(ErrorObtenerCantCopias::LibroNoTieneCopias)));
    }

    #[test]
    fn test_realizar_prestamo() {
        let mut biblio = biblioteca_de_prueba();
        let cliente = biblio.prestamos[0].cliente.clone();
        let libro = Libro {
            isnb: 222,
            titulo: "Otro libro".to_string(),
            autor: "Autor".to_string(),
            numero_paginas: 200,
            genero: Genero::Novela,
            cant_copia: 1,
        };
        biblio.copia_libros.insert(libro.isnb, 1);
        let fecha = Fecha::new(1, 7, 2025);
        let res = biblio.realizar_prestamo(cliente.clone(), &libro, fecha.clone());
        assert_eq!(res, Ok(true));

        // Simular cliente con 5 préstamos
        for _ in 0..5 {
            biblio.prestamos.push(Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha::new(5, 7, 2025),
                fecha_devolucion: None,
                estado: Estado::Prestado,
            });
        }

        let res2 = biblio.realizar_prestamo(cliente.clone(), &libro, fecha);
        assert_eq!(res2, Err(ErrorRealizarPrestamo::ElClienteTieneElMaximoDePrestamos));
    }

    #[test]
    fn test_ver_prestamos_vencidos() {
        let biblio = biblioteca_de_prueba();
        let fecha_actual = Fecha::new(26, 6, 2025);

        let res = biblio.ver_prestamos_vencidos(fecha_actual);
        assert!(matches!(res, Ok(prestamos) if prestamos.len() == 1));

        let fecha_temprana = Fecha::new(1, 6, 2025);
        let res2 = biblio.ver_prestamos_vencidos(fecha_temprana);
        assert_eq!(res2, Err(ErrorPrestamos::NoHayPrestamos));
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblio = biblioteca_de_prueba();
        let prestamo = biblio.prestamos[0].clone();
        let res = biblio.devolver_libro(prestamo.libro.clone(), prestamo.cliente.clone(), Fecha::new(26, 6, 2025));
        assert_eq!(res, Ok(true));

        let libro_falso = Libro {
            isnb: 999,
            ..prestamo.libro.clone()
        };
        let res2 = biblio.devolver_libro(libro_falso, prestamo.cliente.clone(), Fecha::new(26, 6, 2025));
        assert!(matches!(res2, Err(ErrorDevolverLibro::LibroNoEncontrado)));
    }

    #[test]
    fn test_ver_prestamos_a_vencer() {
        let biblio = biblioteca_de_prueba();
        let hoy = Fecha::new(20, 6, 2025);
        let res = biblio.ver_prestamos_a_vencer(hoy.clone(), 10);
        assert!(matches!(res, Ok(p) if p.len() == 1));

        let hoy_tarde = Fecha::new(10, 7, 2025);
        let res2 = biblio.ver_prestamos_a_vencer(hoy_tarde.clone(), 2);
        assert_eq!(res2, Err(ErrorPrestamos::NoHayPrestamos));
    }
    #[test]
    fn test_buscar_prestamo() {
        let biblio = biblioteca_de_prueba();
        let p = &biblio.prestamos[0];
        let res = biblio.buscar_prestamo(&p.libro, &p.cliente);
        assert!(res.is_ok());

        let libro_falso = Libro {
            isnb: 999,
            ..p.libro.clone()
        };
        let res2 = biblio.buscar_prestamo(&libro_falso, &p.cliente);
        assert_eq!(res2, Err(ErrorPrestamos::PrestamoNoExiste));
    }

    #[test]
    fn test_incrementar_y_decrementar_cant_copias() {
        let mut biblio = biblioteca_de_prueba();
        let libro = biblio.prestamos[0].libro.clone();

        let res_inc = biblio.incrementar_cant_copias(&libro);
        assert_eq!(res_inc, Ok(3));

        let res_dec = biblio.decrementar_cant_copias(&libro);
        assert_eq!(res_dec, Ok(2));

        let libro_falso = Libro {
            isnb: 999,
            ..libro.clone()
        };
        let res_dec2 = biblio.decrementar_cant_copias(&libro_falso);
        assert_eq!(res_dec2, Err(ErrorDecrementarCantCopias::LibroNoEncontrado));
    }

    #[test]
    fn test_error_path() {
        let biblio = biblioteca_de_prueba_path_error();
        let result = biblio.reescribir_archivo_json(biblio.path.to_string());
        assert!(!result)

    }

    //
    // nuevos test para entregable 2
    //

    fn crear_cliente(dni: u128) -> Cliente {
        Cliente {
            dni,
            nombre: format!("Cliente {}", dni),
            telefono: "123456789".to_string(),
            direccion: "Calle Falsa 123".to_string(),
            correo: format!("cliente{}@mail.com", dni),
        }
    }

    fn crear_libro(isnb: u64, titulo: &str) -> Libro {
        Libro {
            isnb,
            titulo: titulo.to_string(),
            autor: "Autor Ejemplo".to_string(),
            numero_paginas: 100,
            genero: Genero::Novela,
            cant_copia: 5,
        }
    }

    fn crear_fecha(dia: u8, mes: u8, anio: u16) -> Fecha {
        Fecha { dia, mes, anio }
    }

    fn crear_prestamo(libro: Libro, cliente: Cliente, estado: Estado, fecha_v: Fecha, fecha_d: Option<Fecha>) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento: fecha_v,
            fecha_devolucion: fecha_d,
            estado,
        }
    }

    fn biblioteca_con_prestamos() -> Biblioteca {
        let cliente1 = crear_cliente(1);
        let cliente2 = crear_cliente(2);

        let libro1 = crear_libro(100, "Rust en Acción");
        let libro2 = crear_libro(200, "El Principito");

        let fecha1 = crear_fecha(1, 1, 2025);
        let fecha2 = crear_fecha(15, 1, 2025);
        let fecha_devolucion = Some(crear_fecha(20, 1, 2025));

        let prestamos = vec![
            crear_prestamo(libro1.clone(), cliente1.clone(), Estado::Prestado, fecha1.clone(), None),
            crear_prestamo(libro2.clone(), cliente1.clone(), Estado::Devuelto, fecha2.clone(), fecha_devolucion.clone()),
            crear_prestamo(libro1.clone(), cliente2.clone(), Estado::Prestado, fecha1.clone(), None),
        ];

        Biblioteca {
            path: "".to_string(),
            nombre: "Biblioteca Central".to_string(),
            direccion: "Av. Siempre Viva 742".to_string(),
            copia_libros: HashMap::new(),
            prestamos,
        }
    }
    #[test]
    fn test_historial_todos_los_prestamos_cliente1() {
        let biblio = biblioteca_con_prestamos();
        let resultado = biblio.get_historial_prestamos(1, FiltroPrestamos::todos);
        assert!(resultado.is_ok());
        let prestamos = resultado.unwrap();
        assert_eq!(prestamos.len(), 2);
    }
    #[test]
    fn test_historial_solo_prestados_cliente1() {
        let biblio = biblioteca_con_prestamos();
        let resultado = biblio.get_historial_prestamos(1, FiltroPrestamos::SoloPrestados);
        assert!(resultado.is_ok());
        let prestamos = resultado.unwrap();
        assert_eq!(prestamos.len(), 1);
        assert_eq!(prestamos[0].estado, Estado::Prestado);
    }
    #[test]
    fn test_historial_cliente_sin_prestamos() {
        let biblio = biblioteca_con_prestamos();
        let resultado = biblio.get_historial_prestamos(999, FiltroPrestamos::todos);
        assert!(matches!(resultado, Err(ErrorHistorialPrestamos::ClienteSinPrestamos)));
    }
    #[test]
    fn test_historial_cliente_sin_prestamos_con_filtro() {
        let mut biblio = biblioteca_con_prestamos();

        // Borrar todos los préstamos devueltos de cliente 1
        biblio.prestamos.retain(|p| !(p.cliente.dni == 1 && p.estado == Estado::Devuelto));

        let resultado = biblio.get_historial_prestamos(1, FiltroPrestamos::SoloDevueltos);
        assert!(matches!(resultado, Err(ErrorHistorialPrestamos::ClienteSinPrestamos)));
    }
}







