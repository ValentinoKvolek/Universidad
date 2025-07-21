program test2;

type
    producto = record
        codeP: integer;
        nombreComercial: string;
        precioVenta: real;
        stockActual: integer;
        stockMin: integer;
    end;

    venta = record
        codeP: integer;
        cantVendida: integer;
    end;

    fileProducto = file of producto;
    fileVenta = file of venta;

var
    regP: producto;
    regV: venta;
    p: fileProducto;
    v: fileVenta;
begin
    assign(p, 'maestroProductos');
    assign(v, 'detalleVentas');

    rewrite(p);
    rewrite(v);

    // Cargar productos
    writeln('--- CARGA DE PRODUCTOS ---');
    writeln('Ingrese codigo de producto (0 para terminar): ');
    readln(regP.codeP);
    while (regP.codeP <> 0) do
    begin
        writeln('Ingrese nombre de producto: ');
        readln(regP.nombreComercial);
        writeln('Ingrese precio de venta: ');
        readln(regP.precioVenta);
        writeln('Ingrese stock actual: ');
        readln(regP.stockActual);
        writeln('Ingrese stock minimo: ');
        readln(regP.stockMin);
        write(p, regP);  // Guardar registro en archivo
        writeln('Ingrese codigo de producto (0 para terminar): ');
        readln(regP.codeP);
    end;

    // Cargar ventas
    writeln('--- CARGA DE VENTAS ---');
    writeln('Ingrese codigo de producto vendido (0 para terminar): ');
    readln(regV.codeP);
    while (regV.codeP <> 0) do
    begin
        writeln('Ingrese cantidad vendida: ');
        readln(regV.cantVendida);
        write(v, regV);  // Guardar registro en archivo
        writeln('Ingrese codigo de producto vendido (0 para terminar): ');
        readln(regV.codeP);
    end;

    close(p);
    close(v);
end.
