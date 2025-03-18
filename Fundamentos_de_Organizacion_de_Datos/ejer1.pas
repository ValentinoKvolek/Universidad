(*

    1. Realizar un algoritmo que cree un archivo de números enteros no ordenados y permita
    incorporar datos al archivo. Los números son ingresados desde teclado. La carga finaliza
    cuando se ingresa el número 30000, que no debe incorporarse al archivo. El nombre del
    archivo debe ser proporcionado por el usuario desde teclado. 

*)

program ejer1Prac1;
type
    archivo = file of integer;
var 
    arc_logico:archivo; // variable que define el nombre logico del archivo
    nro:  integer; 
    arc_fisico: string[12]; // nombre fisico del archivo.

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
end.