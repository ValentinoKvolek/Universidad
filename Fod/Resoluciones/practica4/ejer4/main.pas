program ejer4prac4;

procedure buscar(NRR, clave, NRR_encontrado, pos_encontrada, resultado)
var clave_encontrada: boolean;
begin 
    if (nodo = null) 
        resultado := false; {clave no encontrada}
    else
    begin
        posicionarYLeerNodo(A, nodo, NRR);
        claveEncontrada(A, nodo, clave, pos, clave_encontrada);
        if (clave_encontrada) then begin
            NRR_encontrado := NRR; { NRR actual }
            pos_encontrada := pos; { posicion dentro del array }
            resultado := true;
    end;
    else
        buscar(nodo.hijos[pos], clave, NRR_encontrado, pos_encontrada, resultado);
end;

//Asuma que el archivo se encuentra abierto y que, para la primera llamada, el parámetro NRR contiene la posición de la raíz del árbol. Responda detalladamente:
(*
a. PosicionarYLeerNodo(): Indique qué hace y la forma en que deben ser enviados los parámetros (valor o referencia). Implemente este módulo en Pascal.

     Lee un nodo del archivo en la posición especificada por NRR (Número Relativo de Registro) y lo carga en la estructura nodo para su procesamiento.

b. claveEncontrada(): Indique qué hace y la forma en que deben ser enviados los parámetros (valor o referencia). ¿Cómo lo implementaría?


Busca una clave en el arreglo de claves del nodo actual y determina si existe.
Devuelve la posición donde se encuentra (o debería insertarse) y un booleano indicando si fue encontrada.
  

c. ¿Existe algún error en este código? En caso afirmativo, modifique lo que considere necesario.

no verifica si el nodo tiene hijos antes de realizar la búsqueda

d. ¿Qué cambios son necesarios en el procedimiento de búsqueda implementado sobre un árbol B para que funcione en un árbol B+?

    para que funcione para un arbol b+ tedriamos que ignorar las coincidencia de clave si el nodo no es hoja y una vez llegada la hoja buscarla ahi

*)