program ejer5;

type 
    rcelulares = record
        codeCel: integer;
        nombre: string;
        descripcion: string;
        marca: string;
        precio: real;
        stockMin: integer;
        stockDisponible: integer;
    end;

    fileCel = file of rcelulares;

//leer
procedure leer(var celu: rcelulares);
begin
    writeln('ingrese el codigo del celular');
    readln(celu.codeCel);
    writeln('ingrese el nombre del celular');
    readln(celu.nombre);
    writeln('ingrese la descripcion de el celular');
    readln(celu.marca);
    writeln('ingrese el precio');
    readln(celu.precio);
    writeln('ingrese el stock minimo');
    readln(celu.stockMin);
    writeln('ingrese el stock disponible');
    readln(celu.stockDisponible);
end;
//Leer


//B

function comprobar(celu:rcelulares):boolean;
begin
    if(celu.stockDisponible > celu.stockMin) then
        comprobar:=false
    else
        comprobar:=true;
end;

procedure incisob(var c:fileCel);
var
celu:rcelulares;
begin
    reset(c);
    while not eof(c) do begin
        read(c,celu);
        if(comprobar(celu)) then 
            writeln(celu.codeCel:4,' ' , celu.precio:0:2, celu.stockMin:4, celu.stockDisponible:4, ' ', celu.nombre, ' ', celu.descripcion, ' ', celu.marca);;
    end;
    close(c);
end;
// B

// C
procedure incisoc(var c:fileCel);
var
    celu:rcelulares;
begin
    reset(c);
    while not eof(c) do begin
        read(c,celu);
        if(celu.descripcion <> '') then
            writeln(celu.codeCel:4,' ' , celu.precio:0:2, celu.stockMin:4, celu.stockDisponible:4, ' ', celu.nombre, ' ', celu.descripcion, ' ', celu.marca);;
    end;
    close(c);
end;
//C

//6a
procedure addFile(var c: fileCel);
var

  celu: rcelulares;
begin

  leer(celu);  
  reset(c);   
  seek(c, fileSize(c));  

  write(c, celu);  

  seek(c, 0); 

  writeln('El archivo con el celular añadido es este:');
  while not eof(c) do begin
    read(c, celu);  
    with celu do
      writeln(codeCel:4, ' ', precio:0:2, stockMin:4, stockDisponible:4, ' ', nombre, ' ', descripcion, ' ', marca);
  end;
  close(c);
end;
//6a

//6b
procedure editStock(var c:fileCel);
var
    celu:rcelulares;
    newStock: integer;
    nombre: string;
begin
    writeln('ingrese el nombre del celular al cual quiere cambiarle el stock');
    readln(nombre);
    reset(c);
    seek(c,0);
    read(c,celu);
    while not eof(c) and (celu.nombre <> nombre) do begin
        read(c,celu);
    end;
    if not eof(c) then begin
        writeln('ingrese el nuevo stock');
        readln(newStock);
        seek(c, filePos(c) - 1);
        celu.stockDisponible := newStock;
        write(c, celu);
    end
    else
        writeln('el celular buscado no existe.');

    seek(c, 0); 

    writeln('El archivo con el stock actulizado es:');
    while not eof(c) do begin
        read(c, celu);  
        with celu do
        writeln(codeCel:4, ' ', precio:0:2, stockMin:4, stockDisponible:4, ' ', nombre, ' ', descripcion, ' ', marca);
    end;

    close(c);
end;
//6b

var
    Fcel: fileCel;
    filetxt: text;
    filetxt2: text;
    filetxt3: text;
    nombre1, nombre2, nombre3: string;
    celu: rcelulares;
    input: integer;

begin
    
    nombre1 := 'celulares';
    assign(Fcel, nombre1);
    rewrite(Fcel);

    
    nombre2 := 'celulares.txt';
    assign(filetxt, nombre2);
    reset(filetxt);

    
    while not eof(filetxt) do
    begin
        with celu do
        begin
            // Leer tres líneas del archivo de texto para cada celular.
            readln(filetxt, codeCel, precio, marca);  
            readln(filetxt, stockDisponible, stockMin, descripcion); 
            readln(filetxt, nombre);             
        end;

        write(Fcel, celu);
    end;


    seek(Fcel, 0);

    
    while not eof(Fcel) do
    begin
        read(Fcel, celu);
        with celu do
            writeln(codeCel:4,' ' , precio:0:2, stockMin:4, stockDisponible:4, ' ', nombre, ' ', descripcion, ' ', marca);
    end;


    close(Fcel); 
    close(filetxt);
    writeln('los celulares con stock disponible menores al stock minimo son:');
    incisob(Fcel);
    writeln('los celulares que tienen descripcion son:');
    incisoc(Fcel);
    
    //D

    reset(Fcel);
    assign(filetxt2,nombre2);
    rewrite(filetxt2);

    while not eof(Fcel) do begin
        read(Fcel, celu);
        with celu do begin
            writeln(filetxt2, celu.codeCel, ' ', celu.precio:0:2, ' ', celu.marca);
            writeln(filetxt2, celu.stockDisponible, ' ', celu.stockMin, ' ', celu.descripcion);
            writeln(filetxt2, celu.nombre);
        end;
    end;


    writeln('archivo creado correctamente');
    close(Fcel);
    close(filetxt2);
    //D

    writeln('deseas cerrar el programa(press 0) o te gustaria add un celular el archivo(press 1), si necesitas editar un stock(press 2)');
    readln(input);
    while (input <>0) do begin
        if(input = 1) then
            addFile(Fcel)
        else if(input = 2) then
            editStock(Fcel);
        writeln('deseas cerrar el programa(press 0) o te gustaria add un celular el archivo(press 1), si necesitas editar un stock(press 2)');
        readln(input);
    end;

    //6D

    nombre3:= 'SinStock.txt';
    assign(filetxt3, nombre3);
    rewrite(filetxt3);
    reset(Fcel);
    
    while not eof(Fcel) do begin
        read(Fcel, celu);
        if(celu.stockDisponible = 0) then begin
            with celu do begin
                writeln(filetxt3, celu.codeCel, ' ', celu.precio:0:2, ' ', celu.marca);
                writeln(filetxt3, celu.stockDisponible, ' ', celu.stockMin, ' ', celu.descripcion);
                writeln(filetxt3, celu.nombre);
            end;
        end;
    end;
    close(Fcel); close(filetxt3);

    //6D
end.
