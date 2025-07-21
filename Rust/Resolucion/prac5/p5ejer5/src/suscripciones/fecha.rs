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
