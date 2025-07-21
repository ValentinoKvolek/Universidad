(*

2. Definir un programa que genere un archivo con registros de longitud fija conteniendo
información de asistentes a un congreso a partir de la información obtenida por
teclado. Se deberá almacenar la siguiente información: nro de asistente, apellido y
nombre, email, teléfono y D.N.I. Implementar un procedimiento que, a partir del
archivo de datos generado, elimine de forma lógica todos los asistentes con nro de
asistente inferior a 1000.
Para ello se podrá utilizar algún carácter especial situándolo delante de algún campo
String a su elección. Ejemplo: ‘@Saldaño’

*)


program ejer2_prac3;
type

    congreso = record
        nroAsistente :integer;
        apellido: string[50];
        nombre:string[50];
        email:string[50];
        telefono:integer;
        dni:integer;
    end;

    fileMaestro = file of congreso;

procedure eliminar(var mae:fileMaestro);
var
    reg:congreso;
begin
    reset(mae);
    read(mae,reg);
    while(not eof(mae)) do begin
        if (reg.nroAsistente < 1000) then begin
            reg.nombre:= '$'+ reg.nombre;
            seek(mae, filepos(mae)-1);
            write(mae,reg);
        end;
        read(mae,reg);
    end;

    seek(mae,0);
    writeln('nuevo archivo teniendo en cuenta eliminados');
    while(not eof(mae)) do begin
        read(mae,reg);
        writeln('nombre: ', reg.nombre,' nro asistencia: ', reg.nroAsistente);
    end;
    close(mae);
end;

var 
    mae: fileMaestro;
    reg:congreso;
begin

    assign(mae, 'file_maestro');
    rewrite(mae);

    writeln('ingrese nombre un nomre o zzz para salir.');
    readln(reg.nombre);

    while(reg.nombre<>'zzz') do begin

        writeln(' apellido: ');
        readln(reg.apellido);
        writeln(' nro asistencia: ');
        readln(reg.nroAsistente);
        writeln(' email: ');
        readln(reg.email);
        writeln(' telefono: ');
        readln(reg.telefono);
        writeln(' dni: ');
        readln(reg.dni);
        write(mae, reg);

        writeln('ingrese nombre un nomre o zzz para salir.');
        readln(reg.nombre);
    end;
    writeln('sali');
    close(mae);
    eliminar(mae);
end.

