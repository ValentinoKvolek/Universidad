/*


use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub struct Balance(pub f64);

impl Balance {

    pub fn nuevo(balance: f64) -> Self {
        Balance(balance)
    }


    pub fn sumar_f64(&mut self, val: f64) {
        self.0 += val;
    }


    pub fn restar_f64(&mut self, val: f64) {
        self.0 -= val;
    }


    pub fn f64(&self) -> f64 {
        self.0
    }
}


impl Hash for Balance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}


impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}


impl From<f64> for Balance {
    fn from(valor: f64) -> Self {
        Balance(valor)
    }
}

pub trait ComoBalance {
    fn como_balance(&self) -> Balance;
}

// Implementación para `f64`
impl ComoBalance for f64 {
    fn como_balance(&self) -> Balance {
        Balance(*self)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub dni: u32,
    pub verificado: bool,
    pub balance: Balance,
    pub balance_cripto: HashMap<String, Balance>,
}

 */


