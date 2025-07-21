(*

    Se cuenta con un archivo con información de las diferentes distribuciones de linux
    existentes. De cada distribución se conoce: nombre, año de lanzamiento, número de
    versión del kernel, cantidad de desarrolladores y descripción. El nombre de las
    distribuciones no puede repetirse. Este archivo debe ser mantenido realizando bajas
    lógicas y utilizando la técnica de reutilización de espacio libre llamada lista invertida.
    Escriba la definición de las estructuras de datos necesarias y los siguientes
    procedimientos:

            a.​ BuscarDistribucion: módulo que recibe por parámetro el archivo, un
        nombre de distribución y devuelve la posición dentro del archivo donde se
        encuentra el registro correspondiente a la distribución dada (si existe) o
        devuelve -1 en caso de que no exista..

        b.​ AltaDistribucion: módulo que recibe como parámetro el archivo y el registro
        que contiene los datos de una nueva distribución, y se encarga de agregar
        la distribución al archivo reutilizando espacio disponible en caso de que
        exista. (El control de unicidad lo debe realizar utilizando el módulo anterior).
        En caso de que la distribución que se quiere agregar ya exista se debe
        informar “ya existe la distribución”.

        c.​ BajaDistribucion: módulo que recibe como parámetro el archivo y el
        nombre de una distribución, y se encarga de dar de baja lógicamente la
        distribución dada. Para marcar una distribución como borrada se debe
        utilizar el campo cantidad de desarrolladores para mantener actualizada
        la lista invertida. Para verificar que la distribución a borrar exista debe utilizar
        el módulo BuscarDistribucion. En caso de no existir se debe informar
        “Distribución no existente”.


*)


program ejer8_prac3;

type

    //El nombre de las distribuciones no puede repetirse.
    
    distribucion = record
        nombre: string;
        anioLanzamiento:integer;
        numKernel:real;
        cantDesarrolladores:integer;
        descripcion:string;
    end;

    fileMaestro = file of distribucion;



function BuscarDistribucion(var mae:fileMaestro; nameD: string ):integer;
var
    reg : distribucion;
    encontro:boolean;
begin
    encontro:=false;
    while(not eof (mae)) and (not encontro) do begin
        read(mae,reg);
        if(reg.nombre = nameD) then begin
            BuscarDistribucion:= FilePos(mae)-1;
            encontro:=true;
        end;
    end;

    if( not encontro) then 
        BuscarDistribucion:=-1;
end;

procedure AltaDistribucion(var mae:fileMaestro; nuevaD:distribucion);
var
    reg, aux:distribucion;

    p,sigLibre:integer;
    paux :integer;

begin
    
    reset(mae);

    p:= BuscarDistribucion(mae,nuevaD.nombre); //me fijo si no existe una distribucion igual a la nueva.

    //sino:
    if(p = -1) then begin

        seek(mae, 0);
        read(mae, reg);

        if(reg.cantDesarrolladores = 0 ) then begin //si no hay espacio libre: 

            seek(mae, filesize(mae));
            write(mae, nuevaD);

        end
        else begin

            paux:=(reg.cantDesarrolladores *-1); //me da la pos del registro libre
            seek(mae, paux); //leo para saber cual es el siguiente.
            read(mae, aux); //aux = -2 

            //actualizo el siguiente en la cabecera.
            seek(mae, 0);
            read(mae, reg); //-1
            reg.cantDesarrolladores:= aux.cantDesarrolladores; //-2
            seek(mae, 0);
            write(mae, reg);

            //pongo lo nuevo en el registro antes libre
            seek(mae, paux);
            write(mae, nuevaD);

        end;

    end
    else writeln('ya existe la distribución');

    close(mae);
    
end;

procedure BajaDistribucion(var mae: fileMaestro; nameD: string);
var
    reg,aux:distribucion;
    p:integer;

begin
    
    reset(mae); 
    p:=BuscarDistribucion(mae, nameD);

    if(p <> -1) then begin

        seek(mae, 0); //voy a la cabecera
        read(mae, reg); //le0 para ver cual era el anterior libre.
        
        seek(mae, p); //voy a el lugar a eliminar
        read(mae, aux);
        aux.cantDesarrolladores:= reg.cantDesarrolladores; //pongo en mi reg borrado al direccion al que estaba antes libre. (en caso que no exista libre se guarda el 0)
        seek(mae, p); 
        write(mae,aux);

        seek(mae, 0); //voy a la cabecera y pongo la direccion al proximo libre es nev.
        reg.cantDesarrolladores:= -p;
        write(mae, reg);
        
    end 
    else writeln('Distribución no existente');
    close(mae);
end;


var

    mae: fileMaestro;

    reg:distribucion;
    
    input, p:integer;
    nameD:string;


begin
    
    assign(mae, 'arch_distribuciones');

    
    writeln('|| (1):= Buscar Distribucion || (2):= Alta de Distribucion: || (3):= Baja de Distribucion || || (4):= Imprimir || (5):= SALIR');
    readln(input);

    while(input <> 5) do begin

        case input of

            1: 
                begin

                    writeln('Ingrese nombre de distribucion: ');
                    read(nameD);
                    p:= BuscarDistribucion(mae,nameD);

                end;

            2:
                begin

                    writeln('ingrese el nombre de la distribucion nueva:');
                    readln(reg.nombre);
                    writeln('ingrese el anio de lanzamiento');
                    readln(reg.anioLanzamiento);
                    writeln('ingrese el numero de kernel');
                    readln(reg.numKernel);
                    writeln('ingrese cant desarroladores');
                    readln(reg.cantDesarrolladores);
                    writeln('ingrese descripcion');
                    readln(reg.descripcion);

                    AltaDistribucion(mae,reg);

                end;

            3:
                begin

                    writeln('ingrese la distribucion que desea eliminar: ');
                    readln(nameD);
                    
                    BajaDistribucion(mae, nameD);
                    
                end;

            4: 
                begin

                    reset(mae);

                    while(not eof(mae)) do begin

                        read(mae, reg);
                        writeln('distribucion: ', reg.nombre, ' cant Desarolladores: ', reg.cantDesarrolladores);

                    end;

                    close(mae);

                end;

            end;

              writeln('|| (1):= Buscar Distribucion || (2):= Alta de Distribucion: || (3):= Baja de Distribucion || || (4):= Imprimir || (5):= SALIR');
            readln(input);
    end;
end.

