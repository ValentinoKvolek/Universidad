(*

Se cuenta con un archivo que posee información de las ventas que realiza una empresa a
los diferentes clientes. 
Se necesita obtener un reporte con las ventas organizadas por cliente. 
Para ello, se deberá informar por pantalla: los datos personales del cliente, el total
mensual (mes por mes cuánto compró) y finalmente el monto total comprado en el año por el
cliente. Además, al finalizar el reporte, se debe informar el monto total de ventas obtenido
por la empresa.
El formato del archivo maestro está dado por: cliente (cod cliente, nombre y apellido), año,
mes, día y monto de la venta. El orden del archivo está dado por: cod cliente, año y mes.

Nota: tenga en cuenta que puede haber meses en los que los clientes no realizaron
compras. No es necesario que informe tales meses en el reporte.

*)
program ejer9_prac2;
const
    VALOR_ALTO = 'zzz';
type
    maestro = record
        code: integer;
        nombre: string;
        anio: integer;
        mes: integer;
        dia: integer;
        montoVenta: real;
    end;

    fileMaestro = file of maestro;

procedure leer(var arch: fileMaestro; var dato: maestro);
begin
    if (not eof(arch)) then
        read(arch, dato)
    else
        dato.nombre := VALOR_ALTO;
end;

var
    mae: fileMaestro;
    regm: maestro;
    actualCliente: string;
    actualAnio, actualMes: integer;
    totalMes, totalAnio, totalEmpresa: real;
begin
    assign(mae, 'arch_maestro');  
    reset(mae);
    totalEmpresa := 0;

    leer(mae, regm);
    while regm.nombre <> VALOR_ALTO do begin
        actualCliente := regm.nombre;
        writeln('Cliente: ', actualCliente, ' (Código: ', regm.code, ')');
        totalAnio := 0;

        while (regm.nombre <> VALOR_ALTO) and (regm.nombre = actualCliente) do begin
            actualAnio := regm.anio;
            writeln('  Año: ', actualAnio);

            while (regm.nombre <> VALOR_ALTO) and (regm.nombre = actualCliente) and (regm.anio = actualAnio) do begin
                actualMes := regm.mes;
                totalMes := 0;

                while (regm.nombre <> VALOR_ALTO) and (regm.nombre = actualCliente) and (regm.anio = actualAnio) and (regm.mes = actualMes) do begin
                    totalMes := totalMes + regm.montoVenta;
                    leer(mae, regm);
                end;

                if totalMes <> 0 then
                    writeln('    Mes ', actualMes, ': $', totalMes:0:2);

                totalAnio := totalAnio + totalMes;
            end;

            writeln('  Total del año ', actualAnio, ': $', totalAnio:0:2);
            totalEmpresa := totalEmpresa + totalAnio;
        end;
        writeln('------------------------------------------');
    end;

    writeln('MONTO TOTAL DE VENTAS DE LA EMPRESA: $', totalEmpresa:0:2);

    close(mae);
end.

     

