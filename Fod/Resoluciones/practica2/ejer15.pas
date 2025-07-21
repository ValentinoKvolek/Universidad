program ejer15_prac2;
const 
    VALOR_ALTO = 9999;
    CANT_DET = 10;

type
     
     maestro = record
        codeProv: integer;
        nombreProv:string;
        codeLocalidad:integer;
        nombreLocalidad:string;
        viviendasSinLuz:integer;
        viviendasSinGas:integer;
        viviendasSinChapa:integer;
        viviendasSinAgua:integer;
        viviendasSinSanitario:integer;
    end;

    detalle = record
        codeProv:integer;
        codeLocalidad:integer;
        viviendasConLuz:integer;
        viviendasConGas:integer;
        viviendasConstruidas:integer;
        viviendasConAgua:integer;
        entregaSanitario:integer;
    end;

    //ordenados por codigo de prov y de localidad.


    fileMaestro = file of maestro;
    fileDetalle = file of detalle;
    
    arrayDetalles = array [1.. CANT_DET] of fileDetalle;
    arrayRegs = array[1..CANT_DET] of detalle; 

procedure leer (var arch:fileDetalle;  var dato:detalle);
begin
    if(not eof(arch)) then 
        read(arch, dato)
    else
        dato.codeProv:=VALOR_ALTO;
    
end;

procedure minimo(var vd:arrayDetalles; var vr:arrayRegs, var min:detalle);
var
    posMin,i:integer;
begin
    min.codeProv := VALOR_ALTO;
    posMin := 1;
    for i:= 1 to CANT_DET do begin
        if(vr[i].codeProv < min.codeProv) or (vr[i].codeProv = min.codeProv) and (vr[i].codeLocalidad < min.localidad) then begin
            min:=vr[i];
            posMin:=i;
        end;
    end;
    leer(vd[posMin], vr[posMin]);
end;


procedure actualizar (var mae: fileMaestro);
var
    regm: maestro;
    vd:arrayDetalles;
    vr:arrayRegs;
    min: detalle;
begin

    for i:=1 to CANT_DET do begin
        assing(vd[i], 'detalle', i);
        reset(vd[i]);
        leer(vd[i],vr[i]);
    end;

    reset(mae);
    minimo(vd,vr, min);
    read(mae, regm);
    while(min.codeProv<> VALOR_ALTO) do begin
        while(regm.codeProv <> min.codeProv) or (regm.codeLocalidad <> min.codeLocalidad) do
            read(mae, regm);

        while (regm.codeProv = min.codeProv) and (regm.codeLocalidad = min.codeLocalidad) do begin
            regm.viviendasSinLuz:= regm.viviendasSinLuz - min.viviendasConLuz;
            regm.viviendasSinAgua:= regm.viviendasSinAgua - min.viviendasConAgua;
            regm.viviendasSinGas:=regm.viviendasSinGas - min.viviendasConGas;
            regm.viviendasSinSanitario:= regm.viviendasSinSanitario - min.entregaSanitario;
            regm.viviendasSinChapa:= regm.viviendasSinChapa - min.viviendasConstruidas;
            minimo(vd,vr, min);
        end;
        seek(mae, filepos(mae)-1);
        write(mae, regm);

        if not eof (mae) then
            read(mae,regm);
    end;

    close(mae);

    for i:=1 to CANT_DET do 
        close(vd[i]);
    
end;

var
    
    mae: fileMaestro;
    regm:maestro;
    cant:integer;   

begin

    assing(mae,'maestro');

    actualizar(mae);

    reset(mae);

    cant:=0;
    while(not eof(mae)) do begin
        read(mae, regm);
        if(regm.viviendasSinChapa > 0) then
            cant:=cant+1;
    end;

    close(mae);

    writeln('la cant de localidades sin viviendas con chapa es de :', cant);

    
end.

    
