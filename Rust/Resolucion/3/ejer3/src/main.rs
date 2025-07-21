/*

3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a la fecha pasada por parámetro.

*/
const NOMBRE_MESES: [&str; 12]=["Enero", "Febrero", "Marzo",
                                "Abril", "Mayo", "Junio","Julio","Agosto",
                                "Septiembre", "Octubre", "Noviembre", "Diciembre"];
struct Fecha{
    dia:u8, mes:u8, anio:u16
}

fn main() {

}

impl Fecha {
    fn new(dia: u8, mes: u8, anio: u16) -> Fecha {

        Fecha {
            dia,
            mes,
            anio
        }

    }

    fn es_fecha_valida(&self) -> bool {
        
        //check mes
        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

        //como el año es un u16 asumo que ya tiene que ser valido.
        
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


    fn es_mayor(&self, nueva_fecha: Fecha) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_fecha(){
        let nueva_fecha = Fecha::new(28, 4, 2025);
        assert_eq!(nueva_fecha.dia, 28);
        assert_eq!(nueva_fecha.mes, 4);
        assert_eq!(nueva_fecha.anio, 2025);
    }
    #[test]
    fn test_es_fecha_valida(){
        let nueva_fecha = Fecha::new(28, 4, 2025);
        assert!(nueva_fecha.es_fecha_valida(), "28/04/2025 deberia ser true");

    }
    #[test]
    fn test_no_es_fecha_valida(){
        let nueva_fecha = Fecha::new(40, 4, 2025);
        assert!(nueva_fecha.es_fecha_valida(), "40/04/2025 deberia ser false, el dia excede los limites validos.");
    }
    #[test]
    fn test_no_es_fecha_valida2(){
        let nueva_fecha = Fecha::new(28, 13, 2025);
        assert!(nueva_fecha.es_fecha_valida(),"28/13/2025 deberia ser false, mes no valido." );
    }

    #[test]
    fn test_sumar_dias() {
        let mut nueva_fecha = Fecha::new(28, 4, 2025);
        let dias = 3;

        nueva_fecha.sumar_dias(dias);
        assert_eq!(nueva_fecha.dia, 1);
        assert_eq!(nueva_fecha.mes, 5);
        assert_eq!(nueva_fecha.anio, 2025);
    }

    #[test]
    fn test_restar_dias() {
        let mut nueva_fecha = Fecha::new(2, 12, 2025);
        let dias = 3;

        nueva_fecha.restar_dias(dias);
        assert_eq!(nueva_fecha.dia, 29);
        assert_eq!(nueva_fecha.mes, 11);
        assert_eq!(nueva_fecha.anio, 2025);
    }


    #[test]
    fn es_mayor() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(28, 4, 2026);
        assert!(fecha1.es_mayor(fecha2), "deberia ser mayor");
    }

    #[test]
    fn es_mayor2() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(28, 5, 2025);
        assert!(fecha1.es_mayor(fecha2), "deberia ser mayor");
    }
    #[test]
    fn es_mayor3() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(29, 4, 2025);
        assert!(fecha1.es_mayor(fecha2),"deberia ser mayor");
    }
    #[test]
    fn no_es_mayor() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(27, 4, 2025);
        assert!(!fecha1.es_mayor(fecha2), "deberia ser menor");
    }
    #[test]
    fn no_es_mayor2() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(29, 3, 2025);
        assert!(!fecha1.es_mayor(fecha2), "deberia ser menor");
    }
    #[test]
    fn no_es_mayor3() {
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(29, 5, 2024);
        assert!(!fecha1.es_mayor(fecha2), "deberia ser menor");
    }

    #[test]
    fn iguales(){
        let fecha1 = Fecha::new(28, 4, 2025);
        let fecha2 = Fecha::new(28, 4, 2025);
        assert!(!fecha1.es_mayor(fecha2), "deberia ser menor");
    }

}


