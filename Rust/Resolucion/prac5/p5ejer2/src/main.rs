/*

8- Defina la estructura Cancion con campos para el título, el artista y el género.
 El género puede ser rock, pop, rap, jazz, Otros. Luego modele una playlist.
 La playlist está compuesta por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre ella:

➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones  

*/
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

const BASE_FOLDER: &str ="C:/Users/valen/RustroverProjects/Rust/";

#[derive(PartialEq, Clone, Debug)]
#[derive(Serialize)]
enum Genero{
    Rock, Pop, Rap, Jazz, Otros
}
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Serialize)]
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero
}
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Serialize)]
struct Playlist{
    path:String,
    titulo_playlist:String,
    canciones:Vec<Cancion>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorAgregarCancion{
    CancionYaExiste,
    ErrorArchivo,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorEliminarCancion{
    CancionNoExiste,
    ErrorArchivo,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorMoverCancion{
    CancionNoExiste,
    PosFueraDeLimite,
    ErrorArchivo,

}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorBuscarCancion{
    CancionNoExiste,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorBuscarGeneros{
    NoHayCancionesConEsteGenero,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorBuscarArtista{
    NoHayCancionesDeEsteArtista,
}

impl Playlist {
    fn new(nombre: String, path:String) -> Playlist {
        Playlist {
            path,
            titulo_playlist: nombre,
            canciones: Vec::new(),
        }
        
    }

    fn reescribir_archivo_json(&self, path: String)-> bool {

        let mut file = match File::create(format!("{}playlist.json", path)) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        let json_data = match serde_json::to_string_pretty(&self.canciones) {
            Ok(res) => { res }
            Err(_) => { return false }
        };

        match file.write(json_data.as_bytes()) {
            Ok(res) => { true },
            Err(_) => { false}
        }

    }
    fn agregar_cancion(&mut self, cancion: Cancion) -> Result<bool, ErrorAgregarCancion> {

        if self.canciones.contains(&cancion){

            return Err(ErrorAgregarCancion::CancionYaExiste)

        }

        self.canciones.push(cancion);

        let result = self.reescribir_archivo_json(self.path.to_string());
        if result {
            Ok(true)
        }else {
            Err(ErrorAgregarCancion::ErrorArchivo)
        }
    }
    fn eliminar_cancion(&mut self, titulo: String, artista: String) -> Result<Cancion, ErrorEliminarCancion> {

        if let Some(pos) = self.canciones.iter().position(
            |x| x.titulo == titulo && x.artista == artista
        ){

        let cancion_removida = self.canciones.remove(pos);

            if self.reescribir_archivo_json(self.path.to_string()) {
                Ok(cancion_removida)
            } else {
                Err(ErrorEliminarCancion::ErrorArchivo)
            }

        } else {
            Err(ErrorEliminarCancion::CancionNoExiste)
        }
    }
    fn mover_cancion(&mut self, titulo: String, artista: String, pos: u32) -> Result<bool, ErrorMoverCancion> {

        if pos > self.canciones.len() as u32 {
            return Err(ErrorMoverCancion::PosFueraDeLimite)
        }

        let pos_cancion = if let Some(index) = self.canciones.iter().position(|c| c.titulo == titulo && c.artista == artista) {
            index as u32
        } else {
            return Err(ErrorMoverCancion::CancionNoExiste)
        };

        let cancion = self.canciones.remove(pos_cancion as usize);
        self.canciones.insert(pos as usize, cancion);

        if self.reescribir_archivo_json(self.path.to_string()) {
            Ok(true)
        }else {
            Err(ErrorMoverCancion::ErrorArchivo)
        }
    }
    fn buscar_cancion_nombre(&mut self, titulo: &str) -> Result<Cancion, ErrorBuscarCancion> {

        match self.canciones.iter_mut().find(|c| c.titulo == titulo) {
            Some(cancion) => Ok(cancion.clone()),
            None => Err(ErrorBuscarCancion::CancionNoExiste),
        }

    }
    fn obtener_canciones_de_un_genero(&self, genero: Genero) -> Result<Vec<Cancion>, ErrorBuscarGeneros> {

        let coleccion: Vec<Cancion> = self.canciones.iter().filter(|c| c.genero == genero).cloned().collect();

        if coleccion.is_empty() {
            Err(ErrorBuscarGeneros::NoHayCancionesConEsteGenero)
        } else {
            Ok(coleccion)
        }

    }
    fn obtener_canciones_de_un_artista(&self, artista: &str) -> Result <Vec<Cancion>,ErrorBuscarArtista> {

        let coleccion: Vec<Cancion> = self.canciones.iter().filter(|c| c.artista == artista).cloned().collect();

        if coleccion.is_empty() {
            Err(ErrorBuscarArtista::NoHayCancionesDeEsteArtista)
        } else {
            Ok(coleccion)
        }

    }
    fn modificar_titulo_playlist(&mut self, titulo: String){

            self.titulo_playlist = titulo;

    }
    fn eliminar_todo(&mut self)-> Result <bool, ErrorEliminarCancion> {

        self.canciones.clear();

        if self.reescribir_archivo_json(self.path.to_string()) {
            Ok(true)
        }else {
            Err(ErrorEliminarCancion::ErrorArchivo)
        }

    }
    
}

    #[cfg(test)]
    mod tests {
        use super::*;
        fn crear_cancion(titulo: &str, artista: &str, genero: Genero) -> Cancion {
            Cancion {
                titulo: titulo.to_string(),
                artista: artista.to_string(),
                genero,
            }
        }

        #[test]
        fn test_crear_playlist(){
            let playlist = Playlist::new(String::from("Rock Pop"), BASE_FOLDER.to_string());
            assert_eq!(playlist.titulo_playlist,"Rock Pop");
            assert_eq!(playlist.canciones.len(),0);

            let result = playlist.reescribir_archivo_json("C:/Users/valen/OneDrive/autos_test.json".to_string());
            assert!(result);
            let result = playlist.reescribir_archivo_json("Z/nadaqver".to_string());
            assert!(!result);
        }
        #[test]
        fn test_agregar_cancion() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: Vec::new(),
            };

            let cancion = crear_cancion("Bohemian Rhapsody", "Queen", Genero::Rock);
            let resuelt = playlist.agregar_cancion(cancion.clone());
            match resuelt {
                Ok(r) =>{
                    assert!(r);
                }
                Err(e)=>{
                    panic!("no deberia dar error {:?}", e);

                }
            }
            let resuelt = playlist.agregar_cancion(cancion.clone());
            match resuelt {
                Ok(r) =>{
                    panic!("deberia dar error");
                }
                Err(e)=>{
                    assert_eq!(e, ErrorAgregarCancion::CancionYaExiste);
                }
            }
            let mut playlist2 = Playlist {
                path:"z/asdasd/".to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: Vec::new(),
            };
            let cancion = crear_cancion("Bohemian Rhapsody", "Queen", Genero::Rock);
            let resuelt = playlist2.agregar_cancion(cancion.clone());
            match resuelt {
                Ok(r) =>{
                    panic!("deberia dar error");
                }
                Err(e)=>{
                    assert_eq!(e, ErrorAgregarCancion::ErrorArchivo);
                }
            }

        }
        #[test]
        fn test_eliminar_cancion() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 2", Genero::Rock),
                ],
            };
            let cancion = crear_cancion("cancion 1", "artista 1", Genero::Pop);
            let result  = playlist.eliminar_cancion("cancion 1".to_string(), "artista 1".to_string());
            match result {
                Ok(r) =>{
                    assert_eq!(r,cancion);
                }
                Err(e)=>{
                    panic!("no deberia dar el error {:?}", e);
                }
            }
            let result  = playlist.eliminar_cancion("cancion 1".to_string(), "artista 1".to_string());
            match result {
                Ok(r) =>{
                    panic!("deberia dar error")
                }
                Err(e)=>{
                    assert_eq!(e, ErrorEliminarCancion::CancionNoExiste);
                }
            }

            let mut playlist2 = Playlist {
                path:"x/csad".to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 2", Genero::Rock),
                ],
            };
            let cancion = crear_cancion("cancion 1", "artista 1", Genero::Pop);
            let result  = playlist2.eliminar_cancion("cancion 1".to_string(), "artista 1".to_string());
            match result {
                Ok(r) =>{
                    panic!("deberia dar error");
                }
                Err(e)=>{
                    assert_eq!(e, ErrorEliminarCancion::ErrorArchivo);
                }
            }
            
            
            
            
        }
        #[test]
        fn test_mover_cancion() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 2", Genero::Rock),
                    crear_cancion("cancion 3", "artista 3", Genero::Jazz),
                ],
            };
            let result = playlist.mover_cancion("cancion 2".to_string(), "artista 2".to_string(), 1);
            match result {
                Ok(r) =>{
                    assert_eq!(r,true);
                },
                Err(e) => {
                    panic!("no deberia dar el error {:?}", e)
                }
            }
            let result = playlist.mover_cancion("cancion 4".to_string(), "artista 2".to_string(), 2);
            match result {
                Ok(r) =>{
                    panic!("deberia dar error");
                },
                Err(e) => {
                    assert_eq!(e, ErrorMoverCancion::CancionNoExiste);
                }
            }
            let result = playlist.mover_cancion("cancion 2".to_string(), "artista 5".to_string(), 2);
            match result {
                Ok(r) =>{
                    panic!("deberia dar error");
                },
                Err(e) => {
                    assert_eq!(e, ErrorMoverCancion::CancionNoExiste);
                }
            }
            let result = playlist.mover_cancion("cancion 2".to_string(), "artista 5".to_string(), 4);
            match result {
                Ok(r) =>{
                    panic!("deberia dar error");
                },
                Err(e) => {
                    assert_eq!(e, ErrorMoverCancion::PosFueraDeLimite);
                }
            }
        }
        #[test]
        fn test_buscar_cancion_nombre() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 2", Genero::Rock),
                ],
            };
            let result  = playlist.buscar_cancion_nombre("cancion 1");
            match result {
                Ok(c) =>{
                    assert_eq!(c.titulo, "cancion 1" );
                    assert_eq!(c.artista, "artista 1" );
                }
                Err(e)=>{
                    panic!("no deberia dar el error {:?}", e)
                }
            }
            let result  = playlist.buscar_cancion_nombre("cancion 4");
            match result {
                Ok(c) =>{
                    panic!("deberia dar error");
                }
                Err(e)=>{
                    assert_eq!(e, ErrorBuscarCancion::CancionNoExiste);
                }
            }
        }
        #[test]
        fn test_obtener_canciones_por_genero() {
            let playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 1", Genero::Pop),
                    crear_cancion("cancion 3", "artista 3", Genero::Jazz),
                ],
            };
            let result = playlist.obtener_canciones_de_un_genero(Genero::Pop);
            match result {
                Ok(r) =>{
                    assert_eq!(r.len(),2);
                }
                Err(e)=>{
                    panic!("no deberia dar error {:?}", e)
                }
            }
            let result = playlist.obtener_canciones_de_un_genero(Genero::Rap);
            match result {
                Ok(r) =>{
                    panic!("deberia dar error")
                }
                Err(e)=>{
                    assert_eq!(e, ErrorBuscarGeneros::NoHayCancionesConEsteGenero);
                }
            }

        }
        #[test]
        fn test_obtener_canciones_por_artista() {
            let playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 1", Genero::Rock),
                    crear_cancion("cancion 3", "artista 3", Genero::Jazz),
                ],
            };
            let result = playlist.obtener_canciones_de_un_artista("artista 1");
            match result {
                Ok(r) =>{
                    assert_eq!(r.len(),2);
                }
                Err(e)=>{
                    panic!("no deberia dar el error {:?}", e)
                }
            }
            let result = playlist.obtener_canciones_de_un_artista("artista 4");
            match result {
                Ok(r) =>{
                    panic!("deberia dar error");
                }
                Err(e)=>{
                    assert_eq!(e, ErrorBuscarArtista::NoHayCancionesDeEsteArtista)
                }
            }
        }
        #[test]
        fn test_modificar_titulo_playlist() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: Vec::new(),
            };
            playlist.modificar_titulo_playlist("nueva playlist".to_string());
            assert_eq!(playlist.titulo_playlist, "nueva playlist");
        }
        #[test]
        fn test_eliminar_todo() {
            let mut playlist = Playlist {
                path:BASE_FOLDER.to_string(),
                titulo_playlist: "mi playlist".to_string(),
                canciones: vec![
                    crear_cancion("cancion 1", "artista 1", Genero::Pop),
                    crear_cancion("cancion 2", "artista 2", Genero::Rock),
                ],
            };
            playlist.eliminar_todo();
            assert_eq!(playlist.canciones.len(), 0);
        }

    }
fn main() {}
