use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::fs::File;
use std::io::Write;
use crate::suscripciones::suscripciones::MediosPago;
use crate::suscripciones::usuario::{ErrorCancelar as OtherErrorCancelar, ErrorCancelar, ErrorDowngrade as OtherErrorDowngrade, ErrorDowngrade, ErrorUpgrade as OtherErrorUpgrade, ErrorUpgrade, Suscripciones, Usuario};

const BASE_FOLDER: &str ="D:/Universidad/Rust/Resolucion/prac5/p5ejer5/";



struct StreamingRust {
    path: String,
    usuarios: BTreeMap<u64, Usuario>
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorMayorMedioPago{
    SinSuscripciones
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorSuscripciones{
    SinSuscripciones
}



//
//Errores
//
impl StreamingRust {

    fn reescribir_archivo_json(&self) -> bool {

        let mut file = match File::create(format!("{}biblioteca.json", self.path)) {
            Ok(res) => res,
            Err(_) => return false,
        };

        let json_data = match serde_json::to_string_pretty(&self.usuarios) {
            Ok(res) => res,
            Err(_) => return false,
        };

        file.write_all(json_data.as_bytes()).is_ok()
    }

    fn new(usuarios: Option<BTreeMap<u64, Usuario>>) -> StreamingRust {

        let plataforma = StreamingRust {
            path: BASE_FOLDER.to_string(),
            usuarios: usuarios.unwrap_or_default(),
        };

        plataforma.reescribir_archivo_json();

        plataforma
    }

    //➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
    pub fn mayor_medio_pago(&self) -> Result<MediosPago, ErrorMayorMedioPago> {
        let mut contador: HashMap<MediosPago, usize> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if usuario.suscripcion != Suscripciones::Cancelada {
                *contador.entry(usuario.datos.medio_pago.clone()).or_insert(0) += 1;
            }
        });
        contador.into_iter().max_by_key(|(_, count)| *count).map(|(medio, _)| medio).ok_or(ErrorMayorMedioPago::SinSuscripciones)
    }

    //➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
    pub fn suscripcion_mas_contratada(&self) -> Result<Suscripciones, ErrorSuscripciones> {
        let mut contador: HashMap<Suscripciones, usize> = HashMap::new();
        for (_, usuario) in self.usuarios.iter().filter(|(_, u)| u.suscripcion != Suscripciones::Cancelada) {
            *contador.entry(usuario.suscripcion).or_insert(0) += 1;
        }
        if contador.is_empty() {
            return Err(ErrorSuscripciones::SinSuscripciones);
        }
        let (mas_usada, _) = contador.into_iter().max_by_key(|(_, count)| *count).unwrap();
        Ok(mas_usada)
    }

    //➢ Saber cuál fue el medio de pago más utilizado.
    pub fn medio_pago_mas_usado_con_cancelados(&self) -> Result<MediosPago, ErrorMayorMedioPago> {
        let mut contador: HashMap<MediosPago, usize> = HashMap::new();

        for usuario in self.usuarios.values() {
            let tipo = usuario.datos.medio_pago.clone(); // 👈 CLONÁS el valor
            *contador.entry(tipo).or_insert(0) += 1;
        }

        if contador.is_empty() {
            return Err(ErrorMayorMedioPago::SinSuscripciones);
        }

        let (mas_usado, _) = contador
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap();

        Ok(mas_usado)
    }
    fn suscripcion_mas_contratada_con_cancelados(&self) -> Result<Suscripciones, ErrorSuscripciones> {
        let mut contador: HashMap<Suscripciones, usize> = HashMap::new();
        for (_, usuario) in self.usuarios.iter() {
            let tipo = usuario.suscripcion  ;
            *contador.entry(tipo).or_insert(0) += 1;
        }
        if contador.is_empty() {
            return Err(ErrorSuscripciones::SinSuscripciones);
        }
        let (mas_usado, _) = contador.into_iter().max_by_key(|&(_, count)| count).unwrap();
        Ok(mas_usado)
    }

    //➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.

    pub fn upgrade(&mut self, id: u64) -> Result<bool, ErrorUpgrade> {
        
        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Basic => {
                    usuario.suscripcion = Suscripciones::Classic;
                    self.reescribir_archivo_json(); 
                    return Ok(true)
                },
                Suscripciones::Classic => { 
                    usuario.suscripcion = Suscripciones::Super;
                    self.reescribir_archivo_json();
                    return Ok(true)
                },
                Suscripciones::Super => { return Err(ErrorUpgrade::SuscripcionMasAltaPosible); },
                
                _ => { return Err(ErrorUpgrade::NoCuentaConSuscripcion); }
            }
            Ok(true)
        } else {
            Err(ErrorUpgrade::NoCuentaConSuscripcion)
        }
    }
    
    pub fn downgrade(&mut self, id: u64) -> Result<bool, ErrorDowngrade> {
        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Super => {
                    usuario.suscripcion = Suscripciones::Classic;
                }
                Suscripciones::Classic => {
                    usuario.suscripcion = Suscripciones::Basic;
                }
                Suscripciones::Basic => {
                    usuario.suscripcion = Suscripciones::Cancelada;
                    self.reescribir_archivo_json();
                    return Ok(true);
                }
                Suscripciones::Cancelada => {
                    return Err(ErrorDowngrade::NoCuentaConSuscripcion);
                }
            }
            self.reescribir_archivo_json();
            Ok(true)
        } else {
            Err(ErrorDowngrade::NoCuentaConSuscripcion)
        }
    }

    pub fn cancelar_suscripcion(&mut self, id: u64) -> Result<bool, ErrorCancelar> {
        if let Some(usuario) = self.usuarios.get_mut(&id) {
            match usuario.suscripcion {
                Suscripciones::Cancelada => {
                    Err(ErrorCancelar::SuscripcionYaCancelada)
                }
                _ => {
                    usuario.suscripcion = Suscripciones::Cancelada;
                    self.reescribir_archivo_json();
                    Ok(true)
                }
            }
        } else {
            Err(ErrorCancelar::NoCuentaConSuscripcion)
        }
    }



}

#[cfg(test)]
mod tests {
    use crate::suscripciones::fecha::Fecha;
    use crate::suscripciones::suscripciones::Suscripcion;
    use crate::suscripciones::usuario::{ErrorCancelar, ErrorDowngrade, ErrorUpgrade};
    use super::*;
    fn crear_plataforma()-> StreamingRust{
         StreamingRust{
             path : BASE_FOLDER.to_string(),
             usuarios: BTreeMap::new(),
         }
    }
    fn crear_usuario(id_usuario: u8, suscripcion: Suscripciones, datos:Suscripcion ) -> Usuario {
        Usuario {
            id_usuario,
            suscripcion,
            datos,
        }
    }
    #[test]
    fn test_upgrade_downgrade_cancelar_usuario() {
        
        let mut nuevo_user = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user.suscripcion, Suscripciones::Basic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        let mut nuevo_user6 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user6.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user6.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }
        //upgrade:
        let mut nuevo_user2 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user2.upgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user2.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        //upgrade:
        let mut nuevo_user2 = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user2.upgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user2.suscripcion, Suscripciones::Super);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }

        let mut nuevo_user7  = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user7.downgrade();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user7.suscripcion, Suscripciones::Classic);
            }
            Err(e)=>{
                panic!("no debeberia dar el error {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Classic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }
        //cancelar:
        let mut nuevo_user3 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user3.cancelar_suscription();
        match result {
            Ok(r)=>{
                assert_eq!(nuevo_user3.suscripcion, Suscripciones::Cancelada);
            }
            Err(e)=>{
                panic!("no deberia dar el error  {:?}", e);
            }
        }

        //Test Errores:

        let mut nuevo_user_error1 = Usuario::new(123, Suscripciones::Basic, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error1.downgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorDowngrade::SuscripcionMasBajaPosible)
            }
        }
        let mut nuevo_user_error2 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error2.downgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorDowngrade::NoCuentaConSuscripcion)
            }
        }
        let mut nuevo_user_error1 = Usuario::new(123, Suscripciones::Super, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error1.upgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorUpgrade::SuscripcionMasAltaPosible)
            }
        }
        let mut nuevo_user_error2 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error2.upgrade();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorUpgrade::NoCuentaConSuscripcion)
            }
        }
        let mut nuevo_user_error3 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error3.cancelar_suscription();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorCancelar::SuscripcionYaCancelada)
            }
        }
        let mut nuevo_user_error3 = Usuario::new(123, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(12, 06, 2025), Fecha::new(12, 05, 2025), MediosPago::Efectivo));
        let result = nuevo_user_error3.cancelar_suscription();
        match result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorCancelar::SuscripcionYaCancelada)
            }
        }
    }
    #[test]
    fn test_mayor_medio_pago() {
        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Classic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(5585),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.mayor_medio_pago();
        assert_eq!(resultado, Ok(MediosPago::Efectivo));
        
        let plataforma = StreamingRust::new(None);
        assert!(
            matches!(plataforma.mayor_medio_pago(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );

        let mut usuarios = BTreeMap::new();
        usuarios.insert(1, Usuario::new(1, Suscripciones::Cancelada, Suscripcion::new(10.0, Fecha::new(1, 1, 2025), Fecha::new(1, 2, 2025), MediosPago::Efectivo)));
        usuarios.insert(2, Usuario::new(2, Suscripciones::Cancelada, Suscripcion::new(10.0,Fecha::new(1, 1, 2025), Fecha::new(1, 2, 2025), MediosPago::Tarjeta(595959))));

        let plataforma = StreamingRust::new(Some(usuarios));
        assert!(
            matches!(plataforma.mayor_medio_pago(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );
        
        
    }
    #[test]
    fn test_suscripcion_mas_contratada() {
        
        let mut usuarios = BTreeMap::new();
        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Basic,
                Suscripcion::new(
                    5.0,
                    Fecha::new(1, 4, 2025),
                    Fecha::new(1, 5, 2025),
                    MediosPago::Tarjeta(23232),
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Classic,
                Suscripcion::new(
                    8.0,
                    Fecha::new(2, 4, 2025),
                    Fecha::new(2, 5, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Basic,
                Suscripcion::new(
                    5.0,
                    Fecha::new(3, 4, 2025),
                    Fecha::new(3, 5, 2025),
                    MediosPago::Tarjeta(343242),
                ),
            ),
        );
        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.suscripcion_mas_contratada();
        match resultado {
            Ok(r)=>{
                assert_eq!(r, Suscripciones::Basic);
            }
            Err(e)=>{
                panic!("no deberia dar error {:?}", e);
            }
        }
        
        
        //error
        
        let mut usuarios = BTreeMap::new();
        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    0.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(2, 1, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        let plataforma = StreamingRust::new(Some(usuarios));
        let result  = plataforma.suscripcion_mas_contratada();
        match  result {
            Ok(r)=>{
                panic!("Deberia dar error");
            }
            Err(e)=>{
                assert_eq!(e, ErrorSuscripciones::SinSuscripciones);
            }
        }
    }
    #[test]
    fn test_mayor_medio_pago_con_cancelados(){

        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Cancelada,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(5585),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.medio_pago_mas_usado_con_cancelados();
        assert_eq!(resultado, Ok(MediosPago::Efectivo));
        
        //error 

        let plataforma = StreamingRust::new(None);
        assert!(
            matches!(plataforma.medio_pago_mas_usado_con_cancelados(),
                     Err(ErrorMayorMedioPago::SinSuscripciones))
        );

    }
    #[test]
    fn test_suscripcion_mas_contratada_con_cancelados() {
        let mut usuarios = BTreeMap::new();

        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Classic,
                Suscripcion::new(
                    12.0,
                    Fecha::new(10, 3, 2025),
                    Fecha::new(10, 4, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );
        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Basic,
                Suscripcion::new(
                    8.0,
                    Fecha::new(12, 3, 2025),
                    Fecha::new(12, 4, 2025),
                    MediosPago::Tarjeta(12345),
                ),
            ),
        );
        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Classic,
                Suscripcion::new(
                    12.0,
                    Fecha::new(15, 3, 2025),
                    Fecha::new(15, 4, 2025),
                    MediosPago::Tarjeta(67890),
                ),
            ),
        );

        let plataforma = StreamingRust::new(Some(usuarios));
        let resultado = plataforma.suscripcion_mas_contratada_con_cancelados();
        assert_eq!(resultado, Ok(Suscripciones::Classic));

        let plataforma = StreamingRust::new(None);
        let result = plataforma.suscripcion_mas_contratada_con_cancelados();
        assert_eq!(result, Err(ErrorSuscripciones::SinSuscripciones))

    }
    #[test]
    fn test_error_reescribir_arch(){
        let p = StreamingRust{
            path : "///ruta/invalida/".to_string(),
            usuarios: BTreeMap::new(),
        };
       let result =  p.reescribir_archivo_json();
        assert!(!result)
    }
    #[test]
    fn test_upgrade_downgrade_desde_plataforma() {
        let mut usuarios = BTreeMap::new();
        
        usuarios.insert(
            1,
            Usuario::new(
                1,
                Suscripciones::Basic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Efectivo,
                ),
            ),
        );

        usuarios.insert(
            2,
            Usuario::new(
                2,
                Suscripciones::Super,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Tarjeta(123456),
                ),
            ),
        );

        usuarios.insert(
            3,
            Usuario::new(
                3,
                Suscripciones::Classic,
                Suscripcion::new(
                    10.0,
                    Fecha::new(1, 1, 2025),
                    Fecha::new(1, 2, 2025),
                    MediosPago::Transferencia(789456),
                ),
            ),
        );

        let mut plataforma = StreamingRust::new(Some(usuarios));

        // Upgrade usuario 1 (de Basic a Classic)
        let result = plataforma.upgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Classic);

        // Upgrade usuario 3 (de Classic a Super)
        let result = plataforma.upgrade(3);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&3).unwrap().suscripcion, Suscripciones::Super);

        // Upgrade usuario 2 (ya está en Super, debe fallar)
        let result = plataforma.upgrade(2);
        assert_eq!(result, Err(ErrorUpgrade::SuscripcionMasAltaPosible));

        // Downgrade usuario 3 (de Super a Classic)
        let result = plataforma.downgrade(3);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&3).unwrap().suscripcion, Suscripciones::Classic);

        // Downgrade usuario 1 (de Classic a Basic)
        let result = plataforma.downgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Basic);

        // Downgrade usuario 1 nuevamente (de Basic a Cancelada)
        let result = plataforma.downgrade(1);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&1).unwrap().suscripcion, Suscripciones::Cancelada);

        // Downgrade usuario 1 otra vez (ya Cancelada, debe fallar)
        let result = plataforma.downgrade(1);
        assert_eq!(result, Err(ErrorDowngrade::NoCuentaConSuscripcion));

        // Cancelar usuario 2 (Super → Cancelada)
        let result = plataforma.cancelar_suscripcion(2);
        assert!(result.is_ok());
        assert_eq!(plataforma.usuarios.get(&2).unwrap().suscripcion, Suscripciones::Cancelada);

        // Cancelar usuario 2 otra vez (ya Cancelada, debe fallar)
        let result = plataforma.cancelar_suscripcion(2);
        assert_eq!(result, Err(ErrorCancelar::SuscripcionYaCancelada));

        // Usuario inexistente
        let result = plataforma.upgrade(999);
        assert_eq!(result, Err(ErrorUpgrade::NoCuentaConSuscripcion));
    }
    

}


