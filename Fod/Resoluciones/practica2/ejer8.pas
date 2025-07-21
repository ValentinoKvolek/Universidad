(*
8. Se quiere optimizar la gestión del consumo de yerba mate en distintas provincias de
Argentina. Para ello, se cuenta con un archivo maestro que contiene la siguiente
información: código de provincia, nombre de la provincia, cantidad de habitantes y cantidad
total de kilos de yerba consumidos históricamente.
Cada mes, se reciben 16 archivos de relevamiento con información sobre el consumo de
yerba en los distintos puntos del país. Cada archivo contiene: código de provincia y cantidad
de kilos de yerba consumidos en ese relevamiento. Un archivo de relevamiento puede
contener información de una o varias provincias, y una misma provincia puede aparecer
cero, una o más veces en distintos archivos de relevamiento.
Tanto el archivo maestro como los archivos de relevamiento están ordenados por código de
provincia.

Se desea realizar un programa que actualice el archivo maestro en base a la nueva
información de consumo de yerba. Además, se debe informar en pantalla aquellas
provincias (código y nombre) donde la cantidad total de yerba consumida supere los 10.000
kilos históricamente, junto con el promedio consumido de yerba por habitante. Es importante
tener en cuenta tanto las provincias actualizadas como las que no fueron actualizadas.

Nota: cada archivo debe recorrerse una única vez.

*)

program ejer8_prac2;
const
    VALOR_ALTO = 9999;
    CANT_FILE = 16;
type

    //archivos ordenados por code de prov.

    maestro = record
        codeProv:integer;
        nombreProv:string;
        cantHabitantes:integer;
        cantYerbaConsumidos:integer;
    end;

    detalle = record
        codeProv:integer;
        cantKg:real;
    end;

    fileDetalle = file of detalle;

    fileMaestro = file of maestro;

    arrayFile = array [1..CANT_FILE] OF fileDetalle;

    arrayRecord = array [1..CANT_FILE] of detalle;


procedure leer(var arch: fileDetalle; var dato: detalle);
begin
    if(not eof(arch)) then
        read(arch, dato)
    else
        dato.codeProv:= VALOR_ALTO;
end;

procedure minimo( var vD: arrayFile; var vR:arrayRecord;  var min: detalle);
var
    i,pos:integer;
begin

    for i:= 1 to CANT_FILE do begin
        if(vR[i].codeProv <  min.codeProv) then 
            min:=vR[i];
            pos:=i;
    end;
    leer(vD[pos], vR[pos]);
end;

procedure actualizar(var mae: fileMaestro);
var

    vD : arrayFile;
    vR: arrayRecord;
    regm:maestro;
    min:detalle;
    i:integer;

begin
 
    for i:=1 to CANT_FILE do begin
        assig(vD[i], 'detalle',i);
        reser(vD[i]);
        leer(vD[i], vR[i]);
    end;
    reset(mae);

    minimo(vD, vR, min);
    read(mae, regm);

    while(min.codeProv <> VALOR_ALTO) do begin
    
        while(regm.codeProv <> min.codeProv) do
            read(mae);
        while(regm.codeProv = min.codeProv) do begin
            regm.cantYerbaConsumidos:= regm.cantYerbaConsumidos + min.cantKg;
            minimo(vD, vR, min);
        end;
        seek(mae, filepos(mae)-1);
        write(mae, regm);
        if not eof(mae) then 
            read(mae);
    end;

    for i:=1 to CANT_FILE do
        close(vD[i]);

    close(mae);
    
end;

var

    mae: fileMaestro;

begin

    assign(mae, 'archivo maestro');
    actualizar(mae);

    reset(mae);
    
    while not eof (mae) do begin
        read(mae, regm);
        if regm.cantYerbaConsumidos > 10000 do begin
            prom:=regm.cantYerbaConsumidos/regm.cantHabitantes;
            writeln('nombre:', regm.nombreProv,' code: ', regm.codeProv, ' prom: ', prom:0:2);
        end;
    end;

end;

 

        


    





