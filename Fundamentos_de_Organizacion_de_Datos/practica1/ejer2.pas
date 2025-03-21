(*2. Realizar un algoritmo, que utilizando el archivo de números enteros no ordenados
creado en el ejercicio 1, informe por pantalla cantidad de números menores a 1500 y el
promedio de los números ingresados. El nombre del archivo a procesar debe ser
proporcionado por el usuario una única vez. Además, el algoritmo deberá listar el
contenido del archivo en pantalla.*)


program ejer2Prac1;
type
    archivo = file of integer;
    
procedure algoritmo(var intFile: archivo);
var
    cant,resultado,nro,suma:integer;
begin
    cant:=0;
    suma:=0;
    resultado:=0;
    while not eof( intFile) do begin
        read(intFile, nro);  // el read avanza solo a la proxima instruccion.
        suma:= suma + nro;
        if(nro<1500) then 
            cant:=cant+1
    end;
    
    writeln('los numero menos a 1500 fueron: ', cant);

    resultado:= suma/filesize(intFile);
    write('El resultado es: ', resultado);
    writeln('este es el archivo original: ');
    seek(intFile, 0);// coloco el puntero del archivo en 0;
    while not eof( intFile) do begin
        read(intFile, nro);
        writeln(nro);
    end;
end;

var
    nombrearc : String;
    intFile :archivo;
begin
    write('ingrese el nombre del archivo a verificar.')
    readln(nombrearc);
    assign(intFile, nombrearc); // le asigno la direccion de donde esta el archivo existente
    reset(intFile); //abre un archivo  existente.
    algoritmo(intFile);
end.



