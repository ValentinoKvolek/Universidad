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
use std::collections::LinkedList;

#[derive(PartialEq, Clone)]
enum Genero{
    Rock, Pop, Rap, Jazz, Otros
}
#[derive(Clone)]
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero
}

struct Playlist{
    titulo_playlist:String,
    canciones:Vec<Cancion>,
}


pub enum ErrorAgregarCancion{
}
pub enum ErrorEliminarCancion{
    CancionNoExiste,
}

impl Playlist{

    fn new(nombre:String)->Playlist{
        Playlist{
            titulo_playlist:nombre,
            canciones: Vec::new(),
        }

    }
    fn agregar_cancion(&mut self, cancion: Cancion )-> Result<bool, ErrorAgregarCancion>{
            self.canciones.push(cancion);
    }

    fn eliminar_cancion(&mut self, titulo:String, artista:String)->Result<Cancion, ErrorEliminarCancion> {
        if let Some(pos) = self.canciones.iter().position(|x| x.titulo == titulo && x.artista == artista){
            Ok(self.canciones.remove(pos))
        }   else {
            Err(ErrorAgregarCancion::CancionNoExiste)
        }
    }

    fn mover_cancion(&mut self, cancion_move: Cancion, pos:u32){

        self.canciones.insert(pos as usize, cancion_move);

    }

    fn buscar_cancion_nombre(&mut self,  titulo: &str) ->  Option<&mut Cancion> {

        self.canciones.iter_mut().find(|c| c.titulo == titulo)

    }


    fn obtener_canciones_de_un_genero(&self, genero: Genero) -> Vec<Cancion> {

        self.canciones.iter().filter(|c| c.genero == genero).cloned().collect()
    }

    fn obtener_canciones_de_un_artista(&self, artista: &str) -> Vec<Cancion> {

        self.canciones.iter().filter(|c| c.artista == artista).cloned().collect()
    }

    fn modificar_titulo_playlist(&mut self, titulo:String){
        self.titulo_playlist = titulo;
    }

    fn eliminar_todo(&mut self){

        self.canciones.clear();
    }

}
fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    // Función auxiliar para crear una canción de prueba
    
    
    fn crear_cancion(titulo: &str, artista: &str, genero: Genero) -> Cancion {
        Cancion {
            titulo: titulo.to_string(),
            artista: artista.to_string(),
            genero,
        }
    }

   
    fn test_agregar_cancion() {
        let mut playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: Vec::new(),
        };
        let cancion = crear_cancion("Bohemian Rhapsody", "Queen", Genero::Rock);
        playlist.agregar_cancion(cancion.clone());
        assert_eq!(playlist.canciones.len(), 1);
        assert_eq!(playlist.canciones[0].titulo, "Bohemian Rhapsody");
    }

    #[test]
    fn test_eliminar_cancion() {
        let mut playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: vec![
                crear_cancion("cancion 1", "artista 1", Genero::Pop),
                crear_cancion("cancion 2", "artista 2", Genero::Rock),
            ],
        };
        let cancion = crear_cancion("cancion 1", "artista 1", Genero::Pop);
        playlist.eliminar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);
        assert_eq!(playlist.canciones[0].titulo, "cancion 2");
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: vec![
                crear_cancion("cancion 1", "artista 1", Genero::Pop),
                crear_cancion("cancion 2", "artista 2", Genero::Rock),
                crear_cancion("cancion 3", "artista 3", Genero::Jazz),
            ],
        };
        let cancion = crear_cancion("cancion 2", "artista 2", Genero::Rock);
        playlist.mover_cancion(cancion, 0);
        assert_eq!(playlist.canciones[0].titulo, "cancion 2");
        assert_eq!(playlist.canciones[1].titulo, "cancion 1");
    }

    #[test]
    fn test_buscar_cancion_nombre() {
        let mut playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: vec![
                crear_cancion("cancion 1", "artista 1", Genero::Pop),
                crear_cancion("cancion 2", "artista 2", Genero::Rock),
            ],
        };
        let cancion = playlist.buscar_cancion_nombre("cancion 1");
        assert!(cancion.is_some());
        assert_eq!(cancion.unwrap().titulo, "cancion 1");
        assert!(playlist.buscar_cancion_nombre("cancion 3").is_none());
    }

    #[test]
    fn test_obtener_canciones_por_genero() {
        let playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: vec![
                crear_cancion("cancion 1", "artista 1", Genero::Pop),
                crear_cancion("cancion 2", "artista 1", Genero::Pop),
                crear_cancion("cancion 3", "artista 3", Genero::Jazz),
            ],
        };
        let canciones_pop = playlist.obtener_canciones_de_un_genero(Genero::Pop);
        assert_eq!(canciones_pop.len(), 2);
        assert_eq!(canciones_pop[0].titulo, "cancion 1");
        assert_eq!(canciones_pop[1].titulo, "cancion 2");
    }

    #[test]
    fn test_obtener_canciones_por_artista() {
        let playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: vec![
                crear_cancion("cancion 1", "artista 1", Genero::Pop),
                crear_cancion("cancion 2", "artista 1", Genero::Rock),
                crear_cancion("cancion 3", "artista 3", Genero::Jazz),
            ],
        };
        let canciones_artista1 = playlist.obtener_canciones_de_un_artista("artista 1");
        assert_eq!(canciones_artista1.len(), 2);
        assert_eq!(canciones_artista1[0].titulo, "cancion 1");
        assert_eq!(canciones_artista1[1].titulo, "cancion 2");
    }

    #[test]
    fn test_modificar_titulo_playlist() {
        let mut playlist = Playlist {
            titulo_playlist: "mi playlist".to_string(),
            canciones: Vec::new(),
        };
        playlist.modificar_titulo_playlist("nueva playlist".to_string());
        assert_eq!(playlist.titulo_playlist, "nueva playlist");
    }

    #[test]
    fn test_eliminar_todo() {
        let mut playlist = Playlist {
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