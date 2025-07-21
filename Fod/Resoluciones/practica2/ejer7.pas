(*

Se dispone de un archivo maestro con información de los alumnos de la Facultad de
Informática. Cada registro del archivo maestro contiene: código de alumno, apellido, nombre,
cantidad de cursadas aprobadas y cantidad de materias con final aprobado. El archivo
maestro está ordenado por código de alumno.

Además, se tienen dos archivos detalle con información sobre el desempeño académico de
los alumnos: un archivo de cursadas y un archivo de exámenes finales. El archivo de
cursadas contiene información sobre las materias cursadas por los alumnos. Cada registro
incluye: código de alumno, código de materia, año de cursada y resultado (solo interesa si la
cursada fue aprobada o desaprobada). Por su parte, el archivo de exámenes finales
contiene información sobre los exámenes finales rendidos. Cada registro incluye: código de
alumno, código de materia, fecha del examen y nota obtenida. Ambos archivos detalle
están ordenados por código de alumno y código de materia, y pueden contener 0, 1 o
más registros por alumno en el archivo maestro. Un alumno podría cursar una materia
muchas veces, así como también podría rendir el final de una materia en múltiples
ocasiones.

Se debe desarrollar un programa que actualice el archivo maestro, ajustando la cantidad
de cursadas aprobadas y la cantidad de materias con final aprobado, utilizando la
información de los archivos detalle. Las reglas de actualización son las siguientes

● Si un alumno aprueba una cursada, se incrementa en uno la cantidad de cursadas
aprobadas.

● Si un alumno aprueba un examen final (nota >= 4), se incrementa en uno la cantidad
de materias con final aprobado.

Notas:

● Los archivos deben procesarse en un único recorrido.

● No es necesario comprobar que no haya inconsistencias en la información de los
archivos detalles. Esto es, no puede suceder que un alumno apruebe más de una
vez la cursada de una misma materia (a lo sumo la aprueba una vez), algo similar
ocurre con los exámenes finales.

*)

program ejer7prac3;

const VALOR_ALTO = 9999;

type

    //ordenado por codigo de alumno
    
    maestro = record
        codeAlumno: integer;
        apellido:string;
        nombre:string;
        cantCursadasAprobadas:integer;
        cantMateriasAprobadas:integer;
    end;

    //ordenados por code de alumno y materia.
        
    detalleC = record
         codeAlumno:integer;
         codeMateria:integer;
         anioCursada:integer;
         resultado:string;
    end;
    
    detalleF = record 
        codeAlumno:integer;
        codeMateria:integer;
        fechaExamen:string;
        nota:integer;
    end;

    fileDetalleC = file of detalleC;
    fileDetalleF = file of detalleF;
    fileMaestro = file of maestro;
    
end;

procedure leerC (var arch: fileDetalleC; var dato:detalleC);
begin
    if not eof(arch) do 
        read(arch, dato);
    else
        dato.codeAlumno:= VALOR_ALTO;
end;


procedure leerF (var arch: fileDetalleF; var dato:detalleF);
begin
    if not eof(arch) do 
        read(arch, dato);
    else
        dato.codeAlumno:= VALOR_ALTO;
end;

procedure actualizar(var mae: fileMaestro; var detC: fileDetalleC; var detF: fileDetalleF);
var
    codActual: integer;
    totalC, totalF: integer;
begin
    reset(mae);
    reset(detC);
    reset(detF);

    leerC(detC, regdc);
    leerF(detF, regdf);

    while not eof(mae) do begin
        read(mae, regm);
        codActual := regm.codeAlumno;

        totalC := 0;
        totalF := 0;

        // procesar cursadas
        while regdc.codeAlumno = codActual do begin
            if regdc.resultado = 'aprobado' then
                totalC := totalC + 1;
            leerC(detC, regdc);
        end;

        // procesar finales
        while regdf.codeAlumno = codActual do begin
            if regdf.nota >= 4 then
                totalF := totalF + 1;
            leerF(detF, regdf);
        end;

        regm.cantCursadasAprobadas := regm.cantCursadasAprobadas + totalC;
        regm.cantMateriasAprobadas := regm.cantMateriasAprobadas + totalF;

        // volver atrás y escribir
        seek(mae, filepos(mae) - 1);
        write(mae, regm);
    end;

    close(mae);
    close(detC);
    close(detF);
end;
    
end;

var
    mae: fileMaestro;
    detC: fileDetalleC;
    detF: fileDetalleF;

begin

    assign(mae, 'maestro');
    assign(detc, 'detalleCursada');
    assign(detf,'detalleFinales');

    actualizar(mae,detc,detf);
    
end.
