(*
Se desea modelar la información necesaria para un sistema de recuentos de casos de covid
para el ministerio de salud de la provincia de buenos aires

Diariamente se reciben archivos provenientes de los distintos municipios, la información
contenida en los mismos es la siguiente: código de localidad, código cepa, cantidad de
casos activos, cantidad de casos nuevos, cantidad de casos recuperados, cantidad de casos
fallecidos

El ministerio cuenta con un archivo maestro con la siguiente información: código localidad,
nombre localidad, código cepa, nombre cepa, cantidad de casos activos, cantidad de casos
nuevos, cantidad de recuperados y cantidad de fallecidos.
Se debe realizar el procedimiento que permita actualizar el maestro con los detalles
recibidos, se reciben 10 detalles. 


Todos los archivos están ordenados por código de
localidad y código de cepa.

Para la actualización se debe proceder de la siguiente manera:
1. Al número de fallecidos se le suman el valor de fallecidos recibido del detalle.
2. Idem anterior para los recuperados.
3. Los casos activos se actualizan con el valor recibido en el detalle.
4. Idem anterior para los casos nuevos hallados.

Realice las declaraciones necesarias, el programa principal y los procedimientos que
requiera para la actualización solicitada e informe cantidad de localidades con más de 50
casos activos (las localidades pueden o no haber sido actualizadas)
*)

program ejer6_prac2;

uses
    SysUtils;

const VALOR_ALTO = 9999;


type

    detalle = record
        codeLocalidad : integer;
        codeCepa: integer;
        cantActivos: integer;
        cantNuevos:integer;
        cantRecuperados:integer;
        cantFallecidos:integer;
    end;

    maestro = record
        codeLocalidad:integer;
        nombreLocalidad:string;
        codeCepa:integer;
        nombreCepa:string;
        cantActivos:integer;
        cantNuevos:integer;
        cantRecuperados:integer;
        cantFallecidos:integer;
    end;

    fileDetalle = file of detalle;
    fileMaestro = file of maestro;

    detalles = array [1.. 10] of fileDetalle;
    registrosDetalle = array [1.. 10] of detalle; 

procedure leer(var arch:fileDetalle; var dato:detalle);
begin
    if(not eof(arch)) then
        read(arch, dato)
    else    
        dato.codeLocalidad := VALOR_ALTO;
end;

procedure minimo(var vectorD:detalles; var vectorRD:detalle, var min:detalle);
var

i, pos:integer;

begin

    min.codeLocalidad:= VALOR_ALTO;
    pos:=1;

    for i:=1 to 10 do begin
        if(vectorRD[i].codeLocalidad < min.codeLocalidad) or (vectorRD[i].codeLocalidad = min.codeLocalidad) and (vectorRD[i].codeCepa < min.codeCepa) then begin
            min:= vectorRD[i];
            pos:= i;
        end;
    end;

    leer(vectorD[pos], vectorRD[pos]);

end;


procedure actualizar(var mae: fileMaestro);
var

    min: detalle;

    regm: maestro;

    vectorD:detalles; //vector de 10 files detalles.
    vectorRD: registrosDetalle; //vector de 10 registros de tipo detalle.

    i:integer;


begin   

    for i:=1 to 10 do begin
        assign(vectorD[i], 'detalle'+ IntToStr(i));
        reset(vectorD[i]);
        leer(vectorD[i], vectorRD[i]);
    end;
    reset(mae);

    minimo(vectorD,vectorRD,min);

    while(min.codeLocalidad <> VALOR_ALTO) do begin

        read(mae, regm);


        //mientras no coincida en ninguno de los dos no actualizo, 
        while (regm.codeLocalidad <> min.codeLocalidad) or (regm.codeCepa <> min.codeCepa) do 
            read(mae,regm);

        while(regm.codeLocalidad = min.codeLocalidad) and (regm.codeCepa = min.codeCepa ) begin
            regm.cantFallecidos := regm.cantFallecidos + min.cantFallecidos;
            regm.cantRecuperados := regm.cantRecuperados + min.cantRecuperados;
            regm.cantActivos := min.cantActivos;
            regm.cantNuevos := min.cantNuevos;
            minimo(vectorD, vectorRD, min);
        end;

        seek(mae, filepos(mae) - 1);
        write(mae, regm);

        if not eof(mae) then
            read(mae, regm)

    end;

    close(mae)

    for i:=1 to 10 do
        close(vectorD[i]);
    
end;

var
    
    mae: fileMaestro;

    cant:integer;

begin
    cant:=0;
    assign(mae, 'maestro');

    
    actualizar(mae);


    reset(mae);
    
    while (not eof(mae)) do begin
        read(mae,regm);
        if(regm.cantActivos > 50) then
            cant:=cant+1;
    end;

    writeln('la cantidad de localidades con ams de 50 casos activos son: ', cant);

    close(mae);
    
end;
