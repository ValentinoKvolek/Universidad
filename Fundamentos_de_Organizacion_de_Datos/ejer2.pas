(*2. Realizar un algoritmo, que utilizando el archivo de números enteros no ordenados
creado en el ejercicio 1, informe por pantalla cantidad de números menores a 1500 y el
promedio de los números ingresados. El nombre del archivo a procesar debe ser
proporcionado por el usuario una única vez. Además, el algoritmo deberá listar el
contenido del archivo en pantalla.*)


program ejer2Prac1;
type
    archivo = file of integer;
var 
    arc_logico:archivo; // variable que define el nombre logico del archivo
    nro:  integer; 
    arc_fisico: string[12]; // nombre fisico del archivo.

procedure algoritmo2(var arc_logico: archivo);
var
    resultado,nro,suma,totalnro:integer;
begin
    suma:=0;
    totalnro:=0;
    while not eof( arc_logico) do begin
        totalnro:= tototalnro +1;
        read(arc_logico, nro);  // el read avanza solo a la proxima instruccion.
        suma:= suma + nro;
        if(nro<1500) then 
            write(nro);
    end;
    resultado:= suma/totalnro;\
    close(arc_logico);
    write(resultado);
    write('este es el archivo original: ');
    rewrite(arc_logico);
    while not eof( arc_logico) do begin
        read(arc_logico, nro);
        write(nro);
    end;
end;

var
    nombrearc = string;
begin

    write ('ingrese nombre del archivo');
    read(arc_fisico);
    assign(arc_logico, arc_fisico);
    rewrite(arc_logico); // se crea el archivo.
    write ('Ingrese un numero:');
    readln(nro);
    while(nro <> 3000) do begin 
        write( arc_logico, nro ); // escribo el archivo.
        write ('Ingrese un numero:');
        readln(nro);
    end;
    close(arc_logico);
    write('ingrese el nombre del archivo a verificar.')
    read(nombrearc);
    algoritmo2(nombrearc);
end.



