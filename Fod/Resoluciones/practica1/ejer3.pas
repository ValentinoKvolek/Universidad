program ejer3prac1;

type
    empleado = record
        numEmpleado : integer;
        apellido : string;
        nombre : string;
        edad : integer;
        dni : integer;
    end;

    fileEmpleados = file of empleado; 

procedure incisob (var emp : fileEmpleados);
var
    aux : empleado;
    nombre, apellido : string;
begin
    reset(emp); 

    write('Ingrese nombre a buscar: ');
    readln(nombre);
    write('Ingrese apellido a buscar: ');
    readln(apellido);

    while not eof(emp) do begin
        Read(emp, aux); 
        if (aux.apellido = apellido) and (aux.nombre = nombre) then  
            writeln('Empleado encontrado: ', aux.nombre, ' ', aux.apellido, ', DNI: ', aux.dni, ', N°: ', aux.numEmpleado, ', Edad: ', aux.edad);
    end;

    writeln('Lista de empleados en el archivo:');
    seek(emp, 0);
    while not eof(emp) do begin
        read(emp, aux);
        writeln(aux.nombre, ' ', aux.apellido, ', DNI: ', aux.dni, ', N°: ', aux.numEmpleado, ', Edad: ', aux.edad);
        if aux.edad > 70 then
            writeln(aux.nombre, ' está próximo a jubilarse.');
    end;

    close(emp);
end;

procedure agregar(var emp : fileEmpleados);
var
    eAdd, e: empleado;
    existe : boolean;
begin
    reset(emp);
    existe := false;

    writeln('Ingrese el número de empleado:');
    readln(eAdd.numEmpleado);

    while not eof(emp) and not existe do begin
        read(emp, e);
        if e.numEmpleado = eAdd.numEmpleado then
            existe := true;
    end;

    if existe then
        writeln('El empleado ya existe.')
    else begin
        writeln('Ingrese el apellido del empleado:');
        readln(eAdd.apellido);
        writeln('Ingrese el nombre del empleado:');
        readln(eAdd.nombre);
        writeln('Ingrese la edad del empleado:');
        readln(eAdd.edad);
        writeln('Ingrese el DNI del empleado:');
        readln(eAdd.dni);

        seek(emp, FileSize(emp));
        write(emp, eAdd);

        writeln('Empleado agregado correctamente.');
    end;

    close(emp);
end;

procedure modificarEdad(var emp: fileEmpleados);
var
    nEmpleado : integer;
    e: empleado;
    encontrado: boolean;
begin
    reset(emp);
    encontrado := false;

    writeln('Ingrese el número de empleado al cual quiere modificar la edad:');
    readln(nEmpleado);

    while not eof(emp) and not encontrado do begin
        read(emp, e);
        if e.numEmpleado = nEmpleado then begin
            encontrado := true;
            writeln('Edad actual: ', e.edad);
            writeln('Ingrese la nueva edad: ');
            readln(e.edad);

            seek(emp, filepos(emp) - 1);
            write(emp, e);
            writeln('Edad modificada correctamente.');
        end;
    end;

    if not encontrado then
        writeln('Empleado no encontrado.');

    close(emp);
end;

var
    arc_logico: fileEmpleados;
    arc_fisico: string;
    emp: empleado;
    input: integer;
    nombrearc: string;
    txtFile, fileDni: Text;
    nameTxt, dniName: string;
begin
    writeln('Presione 0 para crear un archivo // Presione 1 para modificar un archivo ya existente');
    readln(input);

    if input = 0 then begin 
        writeln('Ingrese nombre del archivo:');
        readln(arc_fisico);
        assign(arc_logico, arc_fisico);
        rewrite(arc_logico);

        writeln('Ingrese apellido (o "fin" para terminar):');
        readln(emp.apellido);

        while emp.apellido <> 'fin' do begin
            writeln('Ingrese número de empleado:');
            readln(emp.numEmpleado);
            writeln('Ingrese nombre:');
            readln(emp.nombre);
            writeln('Ingrese edad:');
            readln(emp.edad);
            writeln('Ingrese número de DNI:');
            readln(emp.dni);

            write(arc_logico, emp);

            writeln('Ingrese un nuevo apellido (o "fin" para terminar):');
            readln(emp.apellido);
        end;

        close(arc_logico);
    end;

    writeln('Ingrese el nombre del archivo a verificar:');
    readln(nombrearc);
    assign(arc_logico, nombrearc);
    reset(arc_logico);

    input:= 999;

    while input <> 0 do begin
        writeln('Presione 1 para agregar un empleado // Presione 2 para modificar la edad de un empleado.');
        readln(input);

        if input = 1 then
            agregar(arc_logico)
        else if input = 2 then
            modificarEdad(arc_logico);

        writeln('Presione 0 para terminar o cualquier otro número para continuar.');
        readln(input);

    end;

    // Guardar los empleados en un archivo de texto
    nameTxt := 'todos_empleados.txt';
    assign(txtFile, nameTxt);
    rewrite(txtFile);
    reset(arc_logico);

    while not eof(arc_logico) do begin
        read(arc_logico, emp);
        writeln(txtFile, emp.nombre, ' ', emp.numEmpleado, ' ', emp.apellido, ' ', emp.edad, ' ', emp.dni);
    end;

    close(txtFile);
    close(arc_logico);

    // Generar archivo con DNIs en cero
    dniName := 'sindni.txt';
    assign(fileDni, dniName);
    rewrite(fileDni);
    reset(arc_logico);

    while not eof(arc_logico) do begin
        read(arc_logico, emp);
        emp.dni := 0;
        writeln(fileDni, emp.nombre, ' ', emp.numEmpleado, ' ', emp.apellido, ' ', emp.edad, ' ', emp.dni);
    end;

    writeln('Proceso finalizado.');
    
    close(fileDni);
    close(arc_logico);
end.
