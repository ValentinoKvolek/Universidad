program ejer7;

type

    novelas = record
        code: integer;
        nombre: string;
        genero: string;
        precio:real;
    end;

    fileNovelas =  file of novelas;

procedure leer(var n:novelas);
begin
    writeln('ingrese el codigo');
    readln(n.code);
    writeln('ingrese el nombre');
    readln(n.nombre);
    writeln('ingrese el genero');
    readln(n.genero);
    writeln('ingrese el precio');
    readln(n.precio);
end;

procedure agregar(var fn : fileNovelas);
var 
    novela:novelas;
begin
    reset(fn);
    leer(novela);
    seek(fn, fileSize(fn));
    write(fn,novela);
    
    seek(fn, 0); 

    writeln('El archivo con la novela nueva es este:');
    while not eof(fn) do begin
        read(fn, novela);  
        with novela do
        writeln(code:4, ' ', precio:0:2,' ', genero, ' ', nombre);
    end;
    close(fn);
end;

procedure editar (var fn : fileNovelas);
var
    codigo:integer;
    novela:novelas;
    newNombre: string;
    newGen:string;
    newPrecio:real;
    input:integer;
begin
    reset(fn);
    writeln('ingrese el codigo de la novela que quiere modificar');
    readln(codigo);
    seek(fn, 0);
    read(fn,novela);
    while not eof(fn) and (novela.code <> codigo) do begin
        read(fn,novela);
    end;
    if not eof(fn) then begin
        writeln('ingrese 1 si quire modifica el precio, 2 si es el genero o 3 para el nombre');
        readln(input);
        if(input = 1) then begin
            writeln('ingrese el nuevo precio');
            readln(newPrecio);
            seek(fn,filePos(fn)-1);
            novela.precio:= newPrecio;
            write(fn,novela);
        end
        else if(input = 2) then begin
            writeln('ingrese el genero nuevo');
            readln(newGen);
            seek(fn,filePos(fn)-1);
            novela.genero:= newGen;
            write(fn,novela);
        end
        else begin
            writeln('ingrese el nuevo nombre');
            readln(newNombre);
            seek(fn,filePos(fn)-1);
            novela.nombre:= newNombre;
            write(fn,novela);
        end;
    end
    else  writeln('la novela no existe');
    close(fn);
end;

var

    filetxt : text;
    nombre1, nombre2: string;
    fn:fileNovelas;
    novela:novelas;
    input: integer;

begin

    writeln('ingrese un nombre para el archivo binario');
    readln(nombre1);
    assign(fn, nombre1);
    rewrite(fn);

    nombre2 := 'novelas.txt';
    assign(filetxt, nombre2);
    reset(filetxt);

        
    while not eof(filetxt) do
    begin
        with novela do
        begin
            // Leer dos líneas del archivo de texto para cada novela.
            readln(filetxt, code, precio, genero);  
            readln(filetxt, nombre);           
        end;

        write(fn, novela);
    end;

    close(filetxt);
    close(fn);

    //b

    writeln('que desea hacer? (0 para salir.) // press 1 para agregar una novela// press 2 para modificar una existente');
    readln(input);
    while(input <> 0) do begin
        if(input = 1) then
            agregar(fn)
        else
            editar(fn);

        writeln('que desea hacer? (0 para salir.) // press 1 para agregar una novela// press 2 para modificar una existente');
        readln(input);
    end;

    //b

end.