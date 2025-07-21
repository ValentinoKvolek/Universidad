(*

Realizar un programa que genere un archivo de novelas filmadas durante el presente
año. De cada novela se registra: código, género, nombre, duración, director y precio.
El programa debe presentar un menú con las siguientes opciones:

a. Crear el archivo y cargarlo a partir de datos ingresados por teclado. Se
utiliza la técnica de lista invertida para recuperar espacio libre en el
archivo. Para ello, durante la creación del archivo, en el primer registro del
mismo se debe almacenar la cabecera de la lista. Es decir un registro
ficticio, inicializando con el valor cero (0) el campo correspondiente al
código de novela, el cual indica que no hay espacio libre dentro del
archivo.

b. Abrir el archivo existente y permitir su mantenimiento teniendo en cuenta el
inciso a), se utiliza lista invertida para recuperación de espacio. En
particular, para el campo de “enlace” de la lista (utilice el código de
novela como enlace), se debe especificar los números de registro
referenciados con signo negativo, . Una vez abierto el archivo, brindar
operaciones para:

    i. Dar de alta una novela leyendo la información desde teclado. Para
    esta operación, en caso de ser posible, deberá recuperarse el
    espacio libre. Es decir, si en el campo correspondiente al código de
    novela del registro cabecera hay un valor negativo, por ejemplo -5,
    se debe leer el registro en la posición 5, copiarlo en la posición 0
    (actualizar la lista de espacio libre) y grabar el nuevo registro en la
    posición 5. Con el valor 0 (cero) en el registro cabecera se indica
    que no hay espacio libre.

    ii.​ Modificar los datos de una novela leyendo la información desde
    teclado. El código de novela no puede ser modificado.

    iii.​ Eliminar una novela cuyo código es ingresado por teclado. Por
    ejemplo, si se da de baja un registro en la posición 8, en el campo
    código de novela del registro cabecera deberá figurar -8, y en el
    registro en la posición 8 debe copiarse el antiguo registro cabecera.

*)

program ejer3_prac3;

type

    novela = record
        code:integer;
        nombre:string;
        genero:string;
        duracion:real;
        director:string;
        precio:real;
    end;

    fileNovela = file of novela;

procedure leerNovela(var n: novela);
begin
    writeln('Ingrese codigo de la novela');
    readln(n.code);
    if(n.code <> -1) then
        begin
            writeln('Ingrese el genero de la novela');
            readln(n.genero);
            writeln('Ingrese el nombre de la novela');
            readln(n.nombre);
            writeln('Ingrese la duracion de la novela');
            readln(n.duracion);
            writeln('Ingrese el director de la novela');
            readln(n.director);
            writeln('Ingrese el precio de la novela');
            readln(n.precio);
        end;
end;

//a
procedure crearFile(var fn:fileNovela);
var
    reg:novela;
    ruta:string;
begin

    //asignacion
    writeln('elige la ruta del archivo: ');
    readln(ruta);
    assign(fn, ruta);
    //asignacion

    rewrite(fn);

    //escribo el reg ficticio.
    reg.code:= 0;
    reg.genero:='';
    reg.nombre:='';
    reg.duracion:=0;
    reg.director:='';
    reg.precio:=0.0;
    write(fn,reg);
    //escribo el reg ficticio.

    leerNovela(reg);
    while(reg.code<>-1)do begin
        write(fn,reg);
        leerNovela(reg);
    end;

    close(fn);
    writeln('archivo creado correctame.');
    
end;

//b-i
procedure alta(var fn: fileNovela);
var
    reg:novela;
    aux:novela;
    ruta:string;
begin
    //asignacion y apertura.
    writeln('ingrese la ruta del archivo para dar de alta una novela');
    readln(ruta);
    assign(fn, ruta);
    reset(fn);
    
    leerNovela(aux);
    read(fn,reg);

    if(reg.code = 0 ) then  //si no hay espacios libres, lo pongo al final.
    begin
        seek(fn,  FileSize(fn));  
        write(fn,aux); 
    end
    else begin

    //si no es 0 entonces hay un espacio libre
        seek(fn, reg.code * -1); //por enunciado se que el espacio libre es el code de novela negativo.
        read(fn, reg); //lo convierto en positivo y me pos en el lugar libre.
        seek(fn, filepos(fn)-1);
        write(fn, aux);
        seek(fn, 0); //actualizo el nuevo espacio libre.
        write(fn, reg);
    end;
    close(fn);
end;
procedure modificarNovela(var n:novela);
var
    input:integer;
begin  
    writeln('ingrese que opcion quiere modificar:');
    writeln(' (1) genero || (2) duracion ');
    writeln(' (3) nombre ||(4) director|| || (5) precio ');
    readln(input);
    case input of
        1:
            begin
                writeln('Ingrese el genero de la novela');
                readln(n.genero);
            end;
        2: 
            begin
                writeln('Ingrese el duracion de la novela');
                readln(n.duracion);
            end;
        3:
            begin
                writeln('Ingrese el nombre de la novela');
                readln(n.nombre);
            end;
        4:
            begin
                writeln('Ingrese el director de la novela');
                readln(n.director);
            end;
        5:
            begin
                writeln('Ingrese el precio de la novela');
                readln(n.precio);
            end;
        else
        writeln('Opcion invalida');
        end;

end;

//b-ii
procedure modificar(var fn: fileNovela);
var
    ruta:string;
    reg:novela;
    code:integer;
begin

    writeln('ingrese ruta del archivo que quiere modificar');
    readln(ruta);
    assign(fn,ruta);
    reset(fn);


    writeln('A continuacion ingrese el code de novela que quiere modificar');
    readln(code);

    read(fn,reg);

    while(not eof (fn))and (code <> reg.code) do 
         read(fn,reg);

    if(eof(fn)) then begin
       writeln('la novela no existe');
    end
    else begin
        seek(fn, filepos(fn)-1);
        modificarNovela(reg);
        write(fn, reg);
    end;
    close(fn);

    writeln('se modifico con exito');
end;

procedure eliminar(var fn: fileNovela);
var
    code, posEliminar,siguiente:integer;
    ruta:string;
    reg:novela;

begin

    writeln('ingrese ruta del archivo que quiere modificar');
    readln(ruta);
    assign(fn,ruta);
    reset(fn);

    writeln('A continuacion ingrese el code de novela que quiere borrar');
    readln(code);

    read(fn, reg);

    while(not eof (fn)) and (code <> reg.code) do 
         read(fn,reg);

    if(code = reg.code) then begin

        posEliminar:= (filepos(fn)-1); //me guardo la pos donde elimino

        seek(fn, 0); //registro cabecera.
        read(fn,reg);
        siguiente:=reg.code; //copio el que era el espacio libre si hay. en el caso que no se copia el 0.
        reg.code:=(posEliminar * -1); //actualizo el reg cabecera para que apunte al nuevo espacio libre (espacio que elimine negativo)
        seek(fn, 0);
        write(fn, reg);

        //ahora actualizo el nuevo espacio libre para que apunte al anterior libre.
        seek(fn, posEliminar);
        read(fn,reg);
        reg.code:=siguiente;
        seek(fn, filepos(fn)-1);
        write(fn, reg);
        writeln('novela eliminada');
    end
    else begin
            writeln('la novela no existe.');
    end;
    close(fn);

    writeln('proceso terminado.');
end;

//c
procedure generarTxT(var fn:fileNovela);
var
    txt:text;
    ruta:string;
    reg:novela;
begin
    writeln('ingrese ruta del archivo que quiere modificar');
    readln(ruta);
    assign(fn,ruta);
    reset(fn);

    seek(fn, 1);
    assign(txt, 'novelas.txt');
    rewrite(txt);
    while(not eof(fn)) do
        begin
            read(fn, reg);
            if(reg.code < 1) then
                write(txt, 'Novela eliminada: ');
            writeln(txt, 'Codigo=', reg.code, 
              ' Genero=', reg.genero, 
              ' Nombre=', reg.nombre, 
              ' Duracion=', reg.duracion:0:2, 
              ' Director=', reg.director, 
              ' Precio=', reg.precio:0:2);
        end;
    writeln('Archivo de texto creado');
    close(fn);
    close(txt);
end;

var
    input:integer;
    fn:fileNovela;
begin


    writeln('ingrese la opcion que desea: ');
    writeln('|| (1):= Crear Archivo || || (2):= Dar de Alta Archivo || ');
    writeln('|| (3):= Modificar el Archivo || || (4):= Eliminar un archivo || || (5):= Generar TxT || ');
    writeln('|| (0):= SALIR ||');
    readln(input);

    while(input<> 0) do begin
        if(input = 1) then 
            crearFile(fn)
        else if(input=2) then
            alta(fn)
        else if(input=3) then 
            modificar(fn)
        else if(input = 4) then 
            eliminar(fn)
        else if(input = 5) then
            generarTxT(fn);
        
    writeln('ingrese la opcion que desea: ');
    writeln('|| (1):= Crear Archivo || || (2):= Dar de Alta Archivo || ');
    writeln('|| (3):= Modificar el Archivo || || (4):= Eliminar un archivo || || (5):= Generar TxT || ');
    writeln('|| (0):= SALIR ||');
    readln(input);

    end;
    



end.