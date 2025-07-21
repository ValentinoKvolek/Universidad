program crearTxT;
type
fileTxt = text;

    rcelulares = record
        codeCel: integer;
        nombre: string;
        descripcion:string;
        marca: string;
        precio: real;
        stockMin:integer;
        stockDisponible: integer;
    end;
        
var
    nombre : string;
    txt:fileTxt;
    celu : rcelulares;
begin

    nombre:='celulares.txt';
    assign(txt, nombre);
    rewrite(txt);
    writeln('ingrese un codigo de cel');
    readln(celu.codeCel);
    while(celu.codeCel <> 0) do begin
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
        writeln(txt, celu.codeCel, ' ', celu.precio:0:2, ' ', celu.marca);
        writeln(txt, celu.stockDisponible, ' ', celu.stockMin, ' ', celu.descripcion);
        writeln(txt, celu.nombre);
        writeln('ingrese un codigo de cel o 0 para salir');
        readln(celu.codeCel);
    end;
    close(txt);
end.