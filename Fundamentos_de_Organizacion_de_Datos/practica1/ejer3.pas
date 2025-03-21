(* 3.Realizar un programa que presente un menú con opciones para:

    a. Crear un archivo de registros no ordenados de empleados y completarlo con
    datos ingresados desde teclado. De cada empleado se registra: número de
    empleado, apellido, nombre, edad y DNI. Algunos empleados se ingresan con
    DNI 00. La carga finaliza cuando se ingresa el String ‘fin’ como apellido.
    
    b. Abrir el archivo anteriormente generado y :
    i. Listar en pantalla los datos de empleados que tengan un nombre o apellido
    determinado, el cual se proporciona desde el teclado.

    ii. Listar en pantalla los empleados de a uno por línea.

    iii. Listar en pantalla los empleados mayores de 70 años, próximos a jubilarse.

*)

program ejer3prac1;
type

    empleado = record
        numEmpleado : integer;
        apellido : string;
        nombre :string;
        edad :integer;
        dni : integer;
    end;

    fileEmpleados = file of empleado; 


procedure incisob (var emp : fileEmpleados);
var
    aux : empleado;
    nombre : string;
    apellido : string;
begin
    write('ingrese nombre a buscar');
    readln(nombre);
    write('ingrese apellido a buscar');
    readln(apellido);
    seek(emp, 0);
    while not eof (emp) do begin
        Read(emp , aux); 
        if(aux.apellido = apellido) and (aux.nombre = nombre) then  
            writeln('los datos del empleado buscado son : ', aux.nombre,' ', aux.dni,' ',aux.numEmpleado,' ',aux.apellido,' ',aux.edad);
    end;
    seek(emp, 0);
    while not eof (emp) do begin
        read(emp,  aux);
        writeln('dato de todos los empleados que contiene este archivo: ');
        writeln(' ', aux.nombre,' ',aux.dni,' ',aux.numEmpleado,' ',aux.apellido);
    end;
    seek(emp, 0);
    while not eof (emp) do begin
        read(emp,  aux);
        if(aux.edad > 70) then begin
            write(aux.nombre, '  esta proximo a jubilarse');
        end;
    end;
end;

procedure agregar(var emp : fileEmpleados);
var
eAdd: empleado; 
e: empleado;
existe : boolean;
begin
    existe := false;
    seek(emp, 0); // me posiciono en el principio.
    writeln('ingrese el numero de empleado.');
    readln(eAdd.numEmpleado);
    while not eof(emp) and (not existe)do begin
        read(emp , e);
        if(e.apellido = eAdd.apellido) then
            existe := true;
    end;
    if(existe) then
        writeln('el empleado ya existe.')
    else begin
        writeln('ingrese el apellido de empleado.');
        readln(eAdd.apellido);
        writeln('ingrese el nombre de empleado.');
        readln(eAdd.nombre);
        writeln('ingrese el edad de empleado.');
        readln(eAdd.edad);
        writeln('ingrese el dni de empleado.');
        readln(eAdd.dni);
        seek(emp, FileSize(emp)); // me posiciono en el ultimo elemento de mi archivo.
        write(emp,  eAdd); //agrego el nuevo empleado.
        writeln('el archivo se modifico correctamente. aqui esta el archivo actualizado: ');
        seek(emp, 0); 
        while not eof(emp) do begin
            read(emp, e);
            writeln('los datos del archivo  son : ', e.nombre,' ', e.dni,' ',e.numEmpleado,' ',e.apellido,' ',e.edad);
        end;
    end;
end;

procedure modificarEdad(var emp: fileEmpleados);
var
    nEmpleado : integer;
    e: empleado;
    encontrado: boolean;
begin
    encontrado := false;
    writeln('ingrese el numero de empleado al cual quiere modificar la edad');
    readln(nEmpleado);
    seek (emp , 0);
    while not eof(emp) and not encontrado do begin
        read(emp, e);
        if(e.numEmpleado = nEmpleado) then begin
            encontrado:= true;
            writeln('la edad actual : ', e.edad);
            writeln('ingrese nueva edad: ');
            readln(e.edad);
        // retroceder una posición para sobrescribir el registro
            seek(emp, filepos(emp) - 1); // le resto uno porque esta aputnando al siguiente dato
            write(emp, e); //sobre escribo el dato y listo.
            writeln('Edad modificada correctamente.');
        end;
    end;
   if not encontrado then
        writeln('Empleado no encontrado.');
end;


var
    arc_logico:fileEmpleados; // variable que define el nombre logico del archivo
    arc_fisico: string; // nombre fisico del archivo.
    emp : empleado;
    input: integer;
    nombrearc : string;
    txtFile: fileEmpleados;
    fileDni: fileEmpleados;
    nameTxt:string;
    dniName: string;
    
begin
    writeln('presione 0 para crear un archivo // presione 1 para modificar un archivo ya existente');
    readln(input);
    if(input = 0) then begin 
        writeln('Ingrese nombre del archivo:');
        readln(arc_fisico);  

        assign(arc_logico, arc_fisico);
        rewrite(arc_logico); // Se crea el archivo.

        writeln('Ingrese apellido:');
        readln(emp.apellido);  

        while emp.apellido <> 'fin' do begin
            writeln('Ingrese numero de empleado:');
            readln(emp.numEmpleado);
            writeln('Ingrese nombre:');
            readln(emp.nombre);
            writeln('Ingrese edad:');
            readln(emp.edad);
            writeln('Ingrese numero de dni:');
            readln(emp.dni);

            write(arc_logico, emp); // Escribo el archivo, y apunta al siguiente automáticamente.

            writeln('Ingrese un nuevo apellido (o "fin" para terminar):');
            readln(emp.apellido);
        end;
        
        incisob(arc_logico);

        writeln('desea terminar el programa? (presiona 0 para terminar o cualquier otro numero para continuar)');
        readln(input);
        close(arc_logico);
    end;

    write('ingrese el nombre del archivo a verificar.');
    readln(nombrearc);

    assign(arc_logico, nombrearc); // le asigno la direccion de donde esta el archivo existente

    reset(arc_logico); //abre un archivo  existente.

    while(input <> 0) do begin
        writeln('si desea agregar un empleado al archivo presione 1 o si desea editar una edad presione 2.');
        read(input);
        if(input =  1) then
            agregar(arc_logico)
        else 
            modificarEdad(arc_logico);
        writeln('desea terminar el programa? (presiona 0 para terminar o cualquier otro numero para continuar)');
        readln(input);
    end;

    //guardo el archivo en otro que sea .txt
    nameTxt:= 'todos_empleados.txt';
    assign(txtFile, nameTxt);
    rewrite(txtFile);
    seek(arc_logico, 0);
    while not eof(arc_logico) do begin
        read(arc_logico, emp);
        write(txtFile, emp);
    end;
    seek(txtFile, 0);
    writeln('estos son los datos del archivo pasados un txt file : ');
    while not eof(txtFile) do begin
        read(txtFile, emp);
        writeln(' ', emp.nombre,' ', emp.dni,' ',emp.numEmpleado,' ',emp.apellido,' ',emp.edad);
    end;

    //actualizo todo los dnis
    dniName:= 'sindni.txt';
    assign(fileDni, dniName);
    rewrite(fileDni);

    seek(arc_logico, 0);
    while not eof(arc_logico) do begin
        Read( arc_logico, emp);
            emp.dni:=00;
        Write( fileDni, emp );
    end;
    seek(fileDni, 0);
    writeln('estos son los datos del archivo con dni modificado : ');
    while not eof(fileDni) do begin
        read(fileDni, emp);
        writeln(' ', emp.nombre,' ', emp.dni,' ',emp.numEmpleado,' ',emp.apellido,' ',emp.edad);
    end;

    close(arc_logico);
    close(txtFile);
    close(fileDni);
end.