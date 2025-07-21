use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;


// fecha.rs
const NOMBRE_MESES: [&str; 12]=["Enero", "Febrero", "Marzo",
    "Abril", "Mayo", "Junio","Julio","Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Fecha{
    pub dia:u8,
    pub mes:u8,
    pub anio:u16
}

impl PartialOrd for Fecha {
    // greater -> mas reciente
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Fecha {
    fn cmp(&self, other: &Self) -> Ordering {
        // comparar si self es mayor que other
        if !self.es_mayor(other) {
            Ordering::Greater
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
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

        //check mes
        if!(1..=12).contains(&self.mes) {return false;}

        if self.dia < 1 || self.dia > self.dias_mes_actual(){return false};

        //como el año es un u16 asumo que ya tiene que ser valido.

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

    pub fn es_mayor(&self, nueva_fecha: &Fecha) -> bool {
        if self.anio < nueva_fecha.anio ||
            (self.anio == nueva_fecha.anio && self.mes < nueva_fecha.mes) ||
            (self.anio == nueva_fecha.anio && self.mes == nueva_fecha.mes && self.dia < nueva_fecha.dia) { return true }

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

//
// client.rs
//

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Cliente {
    pub person: Person,
    pub suscribed_to_newsletter: bool,
    pub email: Option<String>,
}
#[derive(Debug)]
#[derive(PartialEq)]
enum ErrorNuevoCliente{
    InvalidEmail,
}
impl Cliente {
    fn new(person: Person, suscribed_to_newsletter: bool, email: Option<String>) -> Result<Cliente, ErrorNuevoCliente> {
        if let Some(email_str) = &email {
            if !email_str.contains('@') || !email_str.contains('.') {
                return Err(ErrorNuevoCliente::InvalidEmail);
            }
        }

        Ok( Cliente {
            suscribed_to_newsletter,
            person,
            email, }
        )
    }
}

//
// person.rs
//

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Person {
    name: String,
    last_name: String,
    address: String,
    dni:u32,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorNewPerson{
    InvalidName,
    InvalidLastName,
    InvalidDni,
    InvalidAdress,
}
impl Person{
    pub fn new(name: &str, last_name: &str, address: &str, dni: u32) -> Result<Person, ErrorNewPerson> {
        if name.is_empty() {
            return Err(ErrorNewPerson::InvalidName);
        }
        if dni == 0 {
            return Err(ErrorNewPerson::InvalidDni);
        }
        if address.is_empty() {
            return Err(ErrorNewPerson::InvalidAdress);
        }
        if last_name.is_empty() {
            return Err(ErrorNewPerson::InvalidLastName);
        }
        Ok( Self {
            name: name.to_string(),
            last_name: last_name.to_string(),
            address: address.to_string(),
            dni,
        })
    }

}
//
// product.rs
//
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum Category{
    Food,
    Cleaning,
    Home,
    Hygiene,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Producto {
    pub name: String,
    pub category: Category,
    pub price: f64,
}


#[derive(Debug, PartialEq)]
pub enum ErrorNewProduct{
    InvalidName,
    InvalidPrice,
}

impl Producto {
    pub fn new(name: &str, category: Category, price: f64) -> Result<Producto, ErrorNewProduct> {
        if name.is_empty() {
            return Err(ErrorNewProduct::InvalidName);
        }
        if price <= 0.0 {
            return Err(ErrorNewProduct::InvalidPrice);
        }
        Ok(Producto {
            name: name.to_string(),
            category,
            price,
        })
    }
}

//
// sell.rs
//

#[derive(Debug, Clone, PartialEq)]
pub enum MedioPago{
    TarjetaDeCredito,
    TarjetaDeDebito,
    TransferenciaBancaria,
    Efectivo,
}
pub struct Sell {
    pub fecha: Fecha,
    pub cliente: Cliente,
    pub vendedor: u32,
    pub medio_de_pago: MedioPago,
    pub productos: Vec<(Producto, u32)>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorNewSell {
    InvalidFecha,
}

impl Sell {

    //Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.
    pub fn new(cliente: Cliente, fecha: Fecha, vendedor: u32, medio_de_pago: MedioPago, productos: Vec<(Producto, u32)>) -> Result<Self, ErrorNewSell> {
        if !fecha.es_fecha_valida() {
            return Err(ErrorNewSell::InvalidFecha)
        }

        // calcular precio antes de generar la venta

        Ok(Sell {
            cliente,
            fecha,
            vendedor,
            medio_de_pago,
            productos,
        })
    }
}

//
// seller.rs
//
pub struct Seller {
    pub persona: Person,
    pub antiguedad:u16,
    pub salario:f64,
    pub legajo:u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorNewSeller {
    InvalidSalary,
}

impl Seller {
    pub fn new(persona: Person, antiguedad: u16, salario: f64, legajo:u32 ) -> Result<Seller, ErrorNewSeller> {
        if salario <= 0.0 {
            return Err(ErrorNewSeller::InvalidSalary);
        }
        Ok( Seller{ persona, antiguedad, salario, legajo } )
    }
}


//
// SHOP.RS
//

pub struct Shop{
    pub sellers: HashMap<u32, Seller>,
    pub discounts: HashMap<Category, f64>,
    pub sales: Vec<Sell>,
}

const NEWSLETTER_SUSCRIPTION_DISCOUNT: f64 = 5.0;

#[derive(Default)]
pub struct SalesReport {
    pub categories_report: HashMap<Category, u32>,
    pub sellers_report: HashMap<u32, u32> // <legajo, ventas>
}

impl Shop {

    /*
    Calcular el precio final de una venta en base a los productos que hay en ella.
    Para calcularlo tenga en cuenta que pueden haber determinados productos de alguna categoría donde debería aplicarse un descuento.
    Tanto la categoría como el porcentaje de descuento a aplicar son datos que le brinda el sistema.
    Es decir el sistema tiene una lista de las categorías con el descuento a aplicar.
    Además se debe aplicar un porcentaje de descuento general si el cliente tiene suscripción al newsletter.
 */
    pub fn final_price_calculator(&self, sell: &Sell) -> Option<f64> {
        let mut total_price = 0.0;

        for (product, amount) in sell.productos.iter() {
            let mut unit_price = product.price;
            let discount = if let Some(discount) = self.discounts.get(&product.category) { *discount }
            else { 0.0 };

            unit_price *= 1.0 - discount/100.0;

            total_price+=unit_price * (*amount as f64);
        }

        if sell.cliente.suscribed_to_newsletter {
            total_price *= 1.0 - NEWSLETTER_SUSCRIPTION_DISCOUNT/100.0;
        }

        Some(total_price)
    }

    /*
    ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que permita visualizar
        las ventas totales por categoría de producto y otro por vendedor.
    */
    pub fn report(&self) -> SalesReport {
        let mut report = SalesReport::default();
        for sell  in self.sales.iter() {
            for (product, amount) in sell.productos.iter() {
                *report.categories_report.entry(product.category).or_insert(0)+= *amount;
                *report.sellers_report.entry(sell.vendedor).or_insert(0)+= *amount;
            }
        }
        report
    }
}

pub struct CompraCliente {
    pub fecha_de_la_venta: Fecha,
    pub productos_cant: HashMap<String, u32>,
    pub monto_total: f64,
    pub medio_de_pago_utilizado: MedioPago,
}

pub struct ReportOfClient {
    pub dni: u32,
    pub compras: Vec<CompraCliente>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorReportClient{
    ElClienteNoExiste,
}

impl Shop {

    // 🧾 Implementar una funcionalidad que permita obtener un informe de compras realizadas por un cliente específico,
    //      filtrando solo aquellas en las que el monto total final de la venta sea mayor a un valor dado.
    //
    // Este informe debe incluir, ordenado cronológicamente, lo siguiente para cada compra:
    //
    // -Fecha de la venta
    // -Productos adquiridos y sus cantidades
    // -Monto total final de la venta
    // -Medio de pago utilizado
    //
    // La consulta se debe realizar a partir de un identificador único del cliente
    //      (por ejemplo, su DNI o su correo electrónico, según cómo lo hayan modelado),
    //      y un monto mínimo a partir del cual incluir la venta en el informe.
    //
    // En caso de que el cliente no tenga compras que cumplan esa condición, el sistema debe reflejar esa situación de forma adecuada.
    // 🔧 Esta funcionalidad debe implementarse como un métod.o dentro del struct principal del sistema.
    // 🧪 Además, deben incluir los tests necesarios para verificar el correcto funcionamiento de esta funcionalidad.

    pub fn get_historial_compras(&self, dni: u32, monto_minimo: f64) -> Result<ReportOfClient, ErrorReportClient> {

        let mut compras_filtradas: Vec<CompraCliente> = self.sales.iter().filter(|s| s.cliente.person.dni == dni)
            .filter_map(|s| {
                let precio_final = self.final_price_calculator(s)?;

                if precio_final < monto_minimo {
                    return None;
                }

                let mut productos_cant = HashMap::new();
                for (prod, cant) in &s.productos {
                    productos_cant.insert(prod.name.clone(), *cant);
                }

                Some(CompraCliente {
                    fecha_de_la_venta: s.fecha.clone(),
                    productos_cant,
                    monto_total: precio_final,
                    medio_de_pago_utilizado: s.medio_de_pago.clone(), })
            }).collect();

        if compras_filtradas.is_empty() {
            return Err(ErrorReportClient::ElClienteNoExiste);
        }

        compras_filtradas.sort_by(|a, b| {
            a.fecha_de_la_venta.cmp(&b.fecha_de_la_venta).reverse()
        });

        Ok(ReportOfClient {
            dni,
            compras: compras_filtradas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::{panic};

    #[test]
    fn test_create_client_with_invalid_email(){
        let persona = Person::new("valentino", "franco", "calle falsa 123", 123).unwrap();
        let cliente = Cliente::new(persona, true, Some("valenemail.com".into()));

        match cliente {
            Ok(cliente) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNuevoCliente::InvalidEmail);
            }
        }
    }

    #[test]
    fn test_create_person_errors(){
        let person = Person::new("", "franco", "calle falsa 123", 123);
        match person {
            Ok(person) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewPerson::InvalidName);
            }
        }
        let person = Person::new("valentino", "", "calle falsa 123", 123);
        match person {
            Ok(person) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewPerson::InvalidLastName);
            }
        }
        let person = Person::new("valentino", "franco", "", 123);
        match person {
            Ok(person) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewPerson::InvalidAdress);
            }
        }
        let person = Person::new("valentino", "0", "calle falsa 123", 0);
        match person {
            Ok(person) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewPerson::InvalidDni);
            }
        }
    }

    #[test]
    fn test_create_product_errors(){
        let result= Producto::new("", Category::Food, 2.0);
        match result {
            Ok(producto) => {
                panic!("deberia ser un error");

            }
            Err(e) => {
                assert_eq!(e, ErrorNewProduct::InvalidName);
            }
        }

        let result= Producto::new("manzana", Category::Food, -79.0);
        match result {
            Ok(producto) => {
                panic!("deberia ser un error");

            }
            Err(e) => {
                assert_eq!(e, ErrorNewProduct::InvalidPrice);
            }
        }
    }

    #[test]
    fn test_create_sell_with_invalid_date(){
        let product = (Producto::new("manzana", Category::Food, 2.0).unwrap(), 3u32);
        let person = Person::new("valentino", "franco", "calle falsa 123", 123).unwrap();
        let client = Cliente::new(person, false, None).unwrap();
        let seller = Seller { persona: Person::new("karina", "kvolek", "calle 12", 87654321).unwrap(), antiguedad: 5, salario: 500.0, legajo: 5873 };
        let fecha = Fecha::new(0, 4, 2025);

        let products: Vec<(Producto, u32)> = vec![product];
        let sell = Sell::new(client.clone(), fecha, seller.legajo, MedioPago::Efectivo, products.clone());
        match sell {
            Ok(sell) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewSell::InvalidFecha)
            }
        }
        let fecha = Fecha::new(4, 13, 2025);
        let sell = Sell::new(client, fecha, seller.legajo, MedioPago::Efectivo, products);
        match sell {
            Ok(sell) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewSell::InvalidFecha)
            }
        }
    }

    #[test]
    fn test_create_new_seller(){
        let person = Person::new("valentino", "franco", "calle falsa 123", 123).unwrap();
        let seller = Seller::new(person,40,300.0,5650);
        match seller {
            Ok(seller) => {
                assert_eq!(seller.legajo, 5650);
            }
            Err(e) => {
                panic!("no deberia dar el error {:?}", e);
            }
        }
        let person = Person::new("valentino", "franco", "calle falsa 123", 123).unwrap();
        let seller = Seller::new(person,40,0.0,5650);
        match seller {
            Ok(seller) => {
                panic!("deberia ser un error");
            }
            Err(e) => {
                assert_eq!(e, ErrorNewSeller::InvalidSalary)
            }
        }
    }

    #[test]
    fn test_final_price_calculator_no_discount() {
        let shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: Vec::new(),
        };

        let product = (Producto::new("manzana", Category::Food, 2.0).unwrap(), 3u32);
        let person = Person::new("valentino", "franco", "calle falsa 123", 123).unwrap();
        let client = Cliente::new(person, false, None).unwrap();
        let seller = Seller { persona: Person::new("karina", "kvolek", "calle 12", 87654321).unwrap(), antiguedad: 5, salario: 500.0, legajo: 5873 };
        let fecha = Fecha::new(6, 4, 2025);

        let products: Vec<(Producto, u32)> = vec![product];
        let sell = Sell::new(client, fecha, seller.legajo, MedioPago::Efectivo, products).unwrap();

        let result = shop.final_price_calculator(&sell).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_final_price_calculator_with_discount() {
        let mut discounts = HashMap::new();
        discounts.insert(Category::Food, 10.0);
        let shop = Shop {
            sellers: HashMap::new(),
            discounts,
            sales: Vec::new(),
        };

        let product = (Producto::new("manzana", Category::Food, 2.0).unwrap(), 3u32);
        let person = Person::new("valentino", "franco", "calle 123", 123).unwrap();
        let client = Cliente::new(person, false, None).unwrap();
        let seller = Seller { persona: Person::new("karina", "kvolek", "calle 123", 87654321).unwrap(), antiguedad: 2, salario: 100.0, legajo: 434 };
        let fecha = Fecha::new(6, 4, 2025);

        let products: Vec<(Producto, u32)> = vec![product];
        let sell = Sell::new(client, fecha, seller.legajo, MedioPago::Efectivo, products).unwrap();

        let result = shop.final_price_calculator(&sell).unwrap();
        assert_eq!(result, 5.4);
    }

    #[test]
    fn test_final_price_calculator_con_subcripcion() {
        let shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: Vec::new(),
        };

        let product = (Producto::new("manzana", Category::Food, 2.5).unwrap(), 3u32);
        let person = Person::new("valentino", "franco", "calle 123", 123).unwrap();
        let client = Cliente::new(person, true, Some("valen@gmail.com".to_string())).unwrap();
        let seller = Seller { persona: Person::new("karina", "kvolek", "calle falsa", 87654321).unwrap(), antiguedad: 2, salario: 100.0, legajo: 434 };
        let fecha = Fecha::new(6, 4, 2025);

        let products: Vec<(Producto, u32)> = vec![product];
        let sell = Sell::new(client, fecha, seller.legajo, MedioPago::Efectivo, products).unwrap();

        let result = shop.final_price_calculator(&sell).unwrap();
        assert_eq!(result, 7.125);
    }

    #[test]
    fn test_purchase_history_filtered_and_sorted() {

        let persona = Person::new("valen", "franco", "calle falsa", 42849894).unwrap();
        let cliente = Cliente::new(persona, true, Some("valen@email.com".into())).unwrap();

        let fecha1 = Fecha::new(1, 1, 2024);
        let fecha2 = Fecha::new(10, 2, 2024);

        let prod1 = Producto::new("Shampoo", Category::Hygiene, 100.0).unwrap();
        let prod2 = Producto::new("Detergente", Category::Cleaning, 50.0).unwrap();

        let venta1 = Sell::new(cliente.clone(), fecha1, 1, MedioPago::Efectivo, vec![(prod1.clone(), 2)]).unwrap();
        let venta2 = Sell::new(cliente.clone(), fecha2, 1, MedioPago::TarjetaDeCredito, vec![(prod2.clone(), 1)]).unwrap();

        let mut shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: vec![venta1, venta2],
        };

        shop.discounts.insert(Category::Cleaning, 10.0);

        let resultado = shop.get_historial_compras(42849894, 90.0).unwrap();

        assert_eq!(resultado.dni, 42849894);
        assert_eq!(resultado.compras.len(), 1);
        assert_eq!(resultado.compras[0].fecha_de_la_venta.dia, 1);
    }

    #[test]
    fn test_purchase_history_filtered_and_sorted_chronologically() {

        let persona = Person::new("valen", "franco", "calle falsa", 42849894).unwrap();
        let cliente = Cliente::new(persona, true, Some("valen@email.com".into())).unwrap();

        let fecha1 = Fecha::new(12, 2, 2024);
        let fecha2 = Fecha::new(10, 2, 2024);

        let prod1 = Producto::new("Shampoo", Category::Hygiene, 100.0).unwrap();
        let prod2 = Producto::new("Detergente", Category::Cleaning, 550.0).unwrap();

        let venta1 = Sell::new(cliente.clone(), fecha1, 1, MedioPago::Efectivo, vec![(prod1.clone(), 2)]).unwrap();
        let venta2 = Sell::new(cliente.clone(), fecha2, 1, MedioPago::TarjetaDeCredito, vec![(prod2.clone(), 1)]).unwrap();

        let mut shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: vec![venta1, venta2],
        };

        shop.discounts.insert(Category::Cleaning, 10.0);

        let resultado = shop.get_historial_compras(42849894, 90.0).unwrap();

        assert_eq!(resultado.dni, 42849894);
        assert_eq!(resultado.compras.len(), 2);
        assert_eq!(resultado.compras[0].fecha_de_la_venta.dia, 12);
    }

    #[test]
    fn test_purchase_history_filtered_and_sorted_dni_not_found() {

        let persona = Person::new("valen", "franco", "calle falsa", 42849894).unwrap();
        let cliente = Cliente::new(persona, true, Some("valen@email.com".into())).unwrap();

        let fecha1 = Fecha::new(12, 2, 2024);
        let fecha2 = Fecha::new(10, 2, 2024);

        let prod1 = Producto::new("Shampoo", Category::Hygiene, 100.0).unwrap();
        let prod2 = Producto::new("Detergente", Category::Cleaning, 550.0).unwrap();

        let venta1 = Sell::new(cliente.clone(), fecha1, 1, MedioPago::Efectivo, vec![(prod1.clone(), 2)]).unwrap();
        let venta2 = Sell::new(cliente.clone(), fecha2, 1, MedioPago::TarjetaDeCredito, vec![(prod2.clone(), 1)]).unwrap();

        let mut shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: vec![venta1, venta2],
        };

        shop.discounts.insert(Category::Cleaning, 10.0);

        let resultado = shop.get_historial_compras(1, 90.0);
        match resultado {
            Ok(a)=>{
                panic!("deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorReportClient::ElClienteNoExiste);
            }
        }
    }

    #[test]
    fn test_report_generates_correctly() {

        let prod1 = Producto::new("Jabon", Category::Cleaning, 100.0).unwrap();
        let prod2 = Producto::new("Pan", Category::Food, 50.0).unwrap();


        let person = Person::new("valentino", "franco", "Calle Falsa 123", 428489894).unwrap();
        let cliente = Cliente::new(person.clone(), false, None).unwrap();

        let fecha = Fecha::new(1, 7, 2024);
        let productos = vec![(prod1.clone(), 2), (prod2.clone(), 3)];
        let venta = Sell::new(cliente, fecha, 1001, MedioPago::Efectivo, productos.clone()).unwrap();

        let mut shop = Shop {
            sellers: HashMap::new(),
            discounts: HashMap::new(),
            sales: vec![venta],
        };

        let reporte = shop.report();

        assert_eq!(reporte.categories_report.get(&Category::Cleaning), Some(&2));
        assert_eq!(reporte.categories_report.get(&Category::Food), Some(&3));

        assert_eq!(reporte.sellers_report.get(&1001), Some(&5));
    }

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
    fn test_es_mayor() {
        let f = Fecha::new(1, 3, 2023);
        let f2 = Fecha::new(2, 4, 2023);
        let result=   f.es_mayor(&f2);
        assert!(result);

        assert!(f < f2);
    }

}

fn main() {}