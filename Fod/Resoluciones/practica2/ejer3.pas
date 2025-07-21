(*3. A partir de información sobre la alfabetización en la Argentina, se necesita actualizar un
archivo que contiene los siguientes datos: nombre de provincia, cantidad de personas
alfabetizadas y total de encuestados. Se reciben dos archivos detalle provenientes de dos
agencias de censo diferentes, dichos archivos contienen: nombre de la provincia, código de
localidad, cantidad de alfabetizados y cantidad de encuestados. Se pide realizar los módulos
necesarios para actualizar el archivo maestro a partir de los dos archivos detalle.
NOTA: Los archivos están ordenados por nombre de provincia y en los archivos detalle
pueden venir 0, 1 ó más registros por cada provincia.*)

program ejer3prac2;

const valorAlto = 'ZZZ';

type

    maestro = record
        nombreProv:string;
        cantPersona:integer;
        totalEncuestas:integer;
    end;

    detalle = record
        nombreProv:string;
        codeLocalidad:integer;
        cantAlfabetizados:integer;
        cantEncuestados:integer;
    end;

    fileMaestro = file of maestro;
    fileDetalle = file of detalle;

procedure leer(var arch: fileDetalle; var dato: detalle);
begin
    if not eof(arch) then 
        read(arch, dato)
    else 
        dato.nombreProv := valorAlto;
end;


procedure minimo (var det,det2:fileDetalle; var r1,r2: detalle; var min:detalle);
begin
    if (r1.nombreProv<=r2.nombreProv) then begin
        min := r1;
        leer(det,r1)
    end
    else begin
        min := r2;
        leer(det2,r2);
    end;
end;


procedure editarArch(var mae:fileMaestro; var det: fileDetalle; var det2:fileDetalle);
var
    regd: detalle;
    regm: maestro;
    regd2: detalle;
    min:detalle;
begin
    reset(mae);
    reset(det);
    reset(det2);

    writeln('archivo maestro sin editar');
    while (not eof(mae)) do begin
        read(mae, regm);
        writeln('nombre prov: ', regm.nombreProv, ' cantidad de encuestados: ', regm.totalEncuestas,' personas alfabetas:  ', regm.cantPersona );
    end;
    
    seek(mae,0);
    
    leer(det, regd); leer(det2, regd2);
    minimo(det,det2,regd, regd2, min); //compara el minimo de llos dos detalles para comparar en orden.
    while(min.nombreProv <> valorAlto) do begin
        read(mae,regm);
        while (regm.nombreProv <> min.nombreProv) do
            read(mae,regm);

        while (regm.nombreProv = min.nombreProv) do begin
            regm.cantPersona:=regm.cantPersona + min.cantAlfabetizados;
            regm.totalEncuestas:=regm.totalEncuestas +  min.cantEncuestados;
            minimo(det, det2,regd, regd2, min);
        end;
        seek (mae, filepos(mae)-1);
        write(mae,regm);
    end;
    
    seek(mae, 0);
    writeln('archivo maestro modificado');
    while (not eof(mae)) do begin
        read(mae, regm);
        writeln('nombre prov: ', regm.nombreProv, ' cantidad de encuestados: ', regm.totalEncuestas,' personas alfabetas:  ', regm.cantPersona );
    end;


    close(mae);
    close(det);
    close(det2);
        
end;
    

var

    mae:fileMaestro;
    det:fileDetalle;
    det2:fileDetalle;

begin

    //asigno las rutas.
    assign(mae, 'archivoMaestro3');
    assign(det, 'archivoDetalle1_3');
    assign(det2, 'archivoDetalle2_3');
    //abro los archivos.
    editarArch(mae,det,det2);
    
end.
