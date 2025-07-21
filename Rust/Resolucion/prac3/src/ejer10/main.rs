use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::collections::{HashMap};

#[derive(PartialEq, Clone, Debug)]
struct  Biblioteca{
    nombre:String,
    direccion:String,
    copia_libros: HashMap<u64 , u64>,
    prestamos:Vec<Prestamo>,
}
#[derive(PartialEq, Clone, Debug)]
enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
#[derive(Clone, Debug, PartialEq)]
enum Estado{
    Devuelto,
    Prestado,
}
#[derive(PartialEq, Clone, Debug)]
struct Libro{
    isnb: u64,
    titulo: String,
    autor: String,
    numero_paginas: u32,
    genero: Genero,
    cant_copia: u64
}
#[derive(PartialEq, Clone, Debug)]
struct Fecha {
    dia: u8,
    mes: u8,
    anio: u64
}
#[derive(PartialEq, Clone, Debug)]
struct Prestamo{
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}

#[derive(PartialEq, Clone, Debug)]
struct  Cliente{
    nombre: String,
    telefono: String,
    direccion: String,
    correo:String,
}

impl PartialOrd for Fecha {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dia == other.dia
            && self.mes == other.mes
            && self.anio == other.anio
        { return Some(Ordering::Equal) }

        if self.anio > other.anio { return Some(Ordering::Greater) }
        if self.mes > other.mes { return Some(Ordering::Greater) }
        if self.dia > other.dia { return Some(Ordering::Greater) }

        Some(Less)
    }
}

impl Fecha {

    fn new(dia: u8, mes: u8, anio: u64) -> Fecha {
        Fecha {
            dia,
            mes,
            anio,
        }
    }

    fn es_fecha_valida(&self) -> bool {

        //check mes
        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

        true
    }

    fn sumar_dias(&mut self, mut dias: u8) {

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

    fn restar_dias(&mut self, mut dias: u8) {
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


    fn es_mayor(&self, nueva_fecha: &Fecha) -> bool {
        if self.anio < nueva_fecha.anio ||
            (self.anio == nueva_fecha.anio && self.mes < nueva_fecha.mes) ||
            (self.anio == nueva_fecha.anio && self.mes == nueva_fecha.mes && self.dia < nueva_fecha.dia) { return true }

        false
    }

    fn dias_mes_actual(&self) -> u8 {

        match self.mes  {
            4| 6 | 9| 11 => 30, // si es alguno de estos meses tiene 30 dias
            2=> if self.es_bisiesto() {29} else {28},
            _ => 31,
        }
    }

    fn es_bisiesto(&self)-> bool{
        self.anio % 4 == 0}


}

impl Biblioteca{

    fn obtener_cant_copias(self, libro: &Libro) -> u64 {

       let cant =  self.copia_libros.get(&libro.isnb);  //busco en el hashmap la cant copias.

        match cant {
            Some(cant) =>  *cant,
            None => 0 
        }
        
    }
    
    fn decrementar_cant_copias(&mut self, libro: &Libro)  {

        let mut encontrado = self.copia_libros.get_mut(&libro.isnb);

        if let Some(encontrado) = encontrado { *encontrado -= 1 }
        
    }

    fn incrementar_cant_copias(&mut self, libro: &Libro)  {

        let mut encontrado = self.copia_libros.get_mut(&libro.isnb);

        if let Some(encontrado) = encontrado { *encontrado += 1 }

    }

    fn contar_prestamos(&self, cliente: &Cliente) -> u32 {

        let mut cant_p = 0;

        for i in &self.prestamos {
            if i.cliente.nombre == cliente.nombre &&
                i.cliente.telefono == cliente.telefono &&
                i.cliente.direccion == cliente.direccion &&
                i.estado == Estado::Prestado {
                cant_p += 1;
            }
        }
        cant_p
    }

    fn realizar_prestamo(&mut self, cliente_p: Cliente, libro_p: &Libro, fecha_pv: Fecha)-> bool {
        if (self.contar_prestamos(&cliente_p) > 5) {
            return false;
        }

        let cantcopias = self.copia_libros.get(&libro_p.isnb);


        if (cantcopias.is_some()) && (*cantcopias.unwrap() == 0) {
            return false;
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

        true
    }


    fn ver_prestamos_a_vencer(&self, hoy:Fecha, dias: u8)-> Vec<&Prestamo>{

        let mut  prestamos_por_vencer = Vec::<&Prestamo>::new();
        let mut fecha_limite = hoy;
        fecha_limite.sumar_dias(dias);
        //cuando yo recorro un vec, esto me da el "prestamo" de lo que tiene adentro.
        for prestamo in &self.prestamos {
            if prestamo.estado == Estado::Prestado {
                let mut vencimiento = prestamo.fecha_vencimiento.clone();
                if(vencimiento <= fecha_limite ) {
                    prestamos_por_vencer.push(prestamo);
                }
            }
        }

        prestamos_por_vencer

    }

    fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<&Prestamo>{

        let mut prestamos_vencidos = Vec::<&Prestamo>::new();

        for prestamo in &self.prestamos {
            if (prestamo.fecha_vencimiento < fecha_actual){
                prestamos_vencidos.push(prestamo);
            }
        }
        prestamos_vencidos
    }

    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {

        self.prestamos.iter().find(|&prestamo| prestamo.cliente == *cliente && prestamo.libro.isnb == libro.isnb)
    }

    fn devolver_libro(&mut self, libro: Libro, cliente: Cliente, hoy:Fecha) {
        for prestamo in self.prestamos.iter_mut() {
            if prestamo.cliente == cliente && prestamo.libro.isnb == libro.isnb{
                prestamo.estado = Estado::Devuelto;
                prestamo.fecha_devolucion = Some(hoy.clone());
            }
        }
    }
}



fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn crear_fecha(dia: u8, mes: u8, anio: u64) -> Fecha {
        Fecha {
            dia,
            mes,
            anio,
        }
    }

    fn crear_cliente(nombre: &str, telefono: &str, direccion: &str, correo: &str) -> Cliente {
        Cliente {
            nombre: nombre.to_string(),
            telefono: telefono.to_string(),
            direccion: direccion.to_string(),
            correo: correo.to_string(),
        }
    }

    fn crear_libro(isnb: u64, titulo: &str, autor: &str, paginas: u32, genero: Genero, cant_copia: u64, ) -> Libro {
        Libro {
            isnb,
            titulo: titulo.to_string(),
            autor: autor.to_string(),
            numero_paginas: paginas,
            genero,
            cant_copia,
        }
    }

    fn crear_biblioteca(nombre: &str, direccion: &str,  libro: Option<Vec<Libro>>) -> Biblioteca {

        let mut copia_libros = HashMap::new();
        if let Some(libros) = libro {
            for libro in libros {
                copia_libros.insert(libro.isnb, libro.cant_copia );
            }
        }

        Biblioteca {
            nombre: nombre.to_string(),
            direccion: direccion.to_string(),
            copia_libros,
            prestamos: Vec::new(),
        }
    }

    #[test]
    fn test_obtener_cant_copias() {

        let libro = crear_libro(
            12345,
            "El Quijote",
            "Miguel de Cervantes",
            1000,
            Genero::Novela,
            3,
        );

        let mut biblioteca = crear_biblioteca("Biblioteca Central", "Calle Principal 123", Some(vec![libro.clone()]));

        let libro_no_existe = crear_libro(
            99999,
            "Libro Desconocido",
            "Autor Desconocido",
            200,
            Genero::Otros,
            0,
        );

        assert_eq!(biblioteca.clone().obtener_cant_copias(&libro), 3, "Debería devolver 3 copias para el libro con ISBN 12345");

        assert_eq!(biblioteca.clone().obtener_cant_copias(&libro_no_existe), 0, "Debería devolver 0 copias para un libro no registrado");

        //decrementar.

        biblioteca.decrementar_cant_copias(&libro);

        assert_eq!(biblioteca.clone().obtener_cant_copias(&libro), 2, "Deberia dar 2 copias");

        //incremento de nuevo.

        biblioteca.incrementar_cant_copias(&libro);

        assert_eq!(biblioteca.obtener_cant_copias(&libro), 3, "Deberia dar 3 copias");

    }

    #[test]

    fn test_prestamos(){

        let cliente = crear_cliente(
            "Ana Gómez",
            "987654321",
            "Avenida 456",
            "ana@example.com",
        );
        let libro1 = crear_libro(
            12345,
            "El Quijote",
            "Miguel de Cervantes",
            1000,
            Genero::Novela,
            2,
        );

        let cliente2 = crear_cliente(
            "Valentino Franco",
            "9876524321",
            "Avenida 4563",
            "valen@example.com",
        );

        let libro2 = crear_libro(
            67890,
            "Cien Años de Soledad",
            "Gabriel García Márquez",
            500,
            Genero::Novela,
            2,
        );
        let libro3 = crear_libro(
            54321,
            "Clean Code",
            "Robert C. Martin",
            400,
            Genero::Tecnico,
            2,
        );

        let mut biblioteca = crear_biblioteca(
            "Biblioteca Central",
            "Calle Principal 123",
            Some(vec![libro1.clone(), libro2.clone(), libro3.clone()]),
        );

        let fecha1 = crear_fecha(20, 5, 2025);
        let fecha2 = crear_fecha(21, 5, 2025);
        let fecha3 = crear_fecha(22, 5, 2025);

        biblioteca.realizar_prestamo(cliente.clone(), &libro1, fecha1);
        biblioteca.realizar_prestamo(cliente.clone(), &libro2, fecha2);
        biblioteca.realizar_prestamo(cliente.clone(), &libro3, fecha3);

        let hoy = crear_fecha(20, 5, 2025);

        assert_eq!(biblioteca.contar_prestamos(&cliente), 3, "Deberia tener 3 prestamos.");
        // el 22 quedaria afuera del rango a vencer entre hoy y mañana.
        let vector_prestamos_a_vencer = biblioteca.ver_prestamos_a_vencer(hoy, 1);

        assert_eq!(vector_prestamos_a_vencer.len(), 2 , "Deberia tener 2 prestamos.");

        //cambio la fecha para ver si funciona el vencimiento.

        let hoy = crear_fecha(23, 5, 2025);

        let vector_prestamos_vencidos = biblioteca.ver_prestamos_vencidos(hoy.clone());
        //como sigo usando las mismmas fechas, deberian entrar los 3 prestamos. por que hoy cambio a 23.
        assert_eq!(vector_prestamos_vencidos.len(), 3 , "Deberia tener 3 prestamos.");


        assert!(biblioteca.buscar_prestamo(&libro1, &cliente).is_some(), "deberia encontrar un préstamo para cliente y libro1");
        assert!(biblioteca.buscar_prestamo(&libro3, &cliente).is_some(), "encontrar un préstamo para cliente y libro3");
        assert!(biblioteca.buscar_prestamo(&libro1, &cliente2).is_none(), "no debería encontrar un préstamo para cliente2 y libro1");

        //devolver un libro

        let  prestamo = biblioteca.buscar_prestamo(&libro1, &cliente);
        assert_eq!(prestamo.unwrap().estado, Estado::Prestado, "El estado del préstamo debería ser Prestado");

        biblioteca.devolver_libro(libro1.clone(), cliente.clone(), hoy.clone());
        let  prestamo = biblioteca.buscar_prestamo(&libro1, &cliente);
        assert_eq!(prestamo.unwrap().estado, Estado::Devuelto, "El estado del préstamo debería ser Devuelto");
        assert_eq!(prestamo.unwrap().fecha_devolucion, Some(hoy), "La fecha de devolución debería ser hoy");

    }
    #[test]

    fn test_devolver_libro() {

        let cliente = crear_cliente(
            "Ana Gómez",
            "987654321",
            "Avenida 456",
            "ana@example.com",
        );

        let libro1 = crear_libro(
            12345,
            "El Quijote",
            "Miguel de Cervantes",
            1000,
            Genero::Novela,
            2,
        );

        let mut biblioteca = crear_biblioteca(
            "Biblioteca Central",
            "Calle Principal 123",
            Some(vec![libro1.clone()]),
        );

        let fecha1 = crear_fecha(20, 5, 2025);
        let hoy = crear_fecha(21, 5, 2025);

        biblioteca.realizar_prestamo(cliente.clone(), &libro1, fecha1); //creo el prestamo.

        let  prestamo = biblioteca.buscar_prestamo(&libro1, &cliente);
        assert_eq!(prestamo.unwrap().estado, Estado::Prestado, "El estado del préstamo debería ser Prestado");


        biblioteca.devolver_libro(libro1.clone(), cliente.clone(), hoy.clone());

        let prestamo = biblioteca.buscar_prestamo(&libro1, &cliente).expect("El préstamo debería existir");
        assert_eq!(prestamo.estado, Estado::Devuelto, "El estado del préstamo debería ser Devuelto");
        assert_eq!(prestamo.fecha_devolucion, Some(hoy), "La fecha de devolución debería ser hoy");
    }
}
//se que se repite bastante codigo pero es la forma que encontre para verificar si realmente cada modulo cumple.
//me di cuenta tarde, de que podria ser mucho mejor usar una BTreeMap en todo lo que es busqueda de copias de libros. 






