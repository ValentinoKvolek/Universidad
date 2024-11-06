program parcial2;
type

	recital = record
		nombreBanda:string;
		cantCanciones:integer;
		montoRecaudado:integer;
	end;
	
	
	arbol = ^nodo;
	
	nodo = record
		elem:recital;
		hI:arbol;
		hD:arbol;
	end;
	
	
	lista = ^nodoLista;
	
	nodoLista= record
		elem:recital;
		sig:lista;
	end;
	
	
	procedure generarArbol(var a:arbol);
	
		procedure leerElemento(var elem:recital);
		begin
			writeln('ingresar nombre de la banda');
			readln(elem.nombreBanda);
			if(elem.nombreBanda <> 'ZZZ') then begin
				writeln('ingresar cantidad de canciones');
				readln(elem.cantCanciones);
				writeln('monto recaudado');
				readln(elem.montoRecaudado);
			end;
		end;
		
		procedure llenarArbol(var a:arbol; elem:recital);
		begin
			if(a = nil) then begin
				new(a); a^.elem:=elem; a^.hI:=nil; a^.hD:=nil;
			end
			else begin if(elem.montoRecaudado < a^.elem.montoRecaudado) then llenarArbol(a^.hI, elem)
				else llenarArbol(a^.hD,elem);
			end;
		end;
		
	var
	elem:recital;
	begin

		leerElemento(elem);
		if(elem.nombreBanda <> 'ZZZ') then begin
			llenarArbol(a,elem);
			generarArbol(a);
		end;
	end;
	
	procedure generarLista(var l:lista; v1:integer; v2:integer; a:arbol);
	
		procedure insertarOrdenado(var l:lista; a:arbol);
		var
		act,ant,nuevo:lista;
		begin
			writeln('insertar ordenado 1');
			new(nuevo); nuevo^.elem:= a^.elem; nuevo^.sig:=nil;
			writeln('insertar ordenado 2');
			if(l = nil) then l:=nuevo
			else begin
				act:=l; ant:=l;
				while (act <> nil) and ( nuevo^.elem.montoRecaudado > act^.elem.montoRecaudado ) do begin
					ant:=act;
					act:=act^.sig;
					writeln('insertar ordenado 3');
				end;  
				writeln('insertar ordenado 4');
			end;
			if(act = l) then begin
					nuevo^.sig:=l;
					l:=nuevo;
					writeln('insertar ordenado 5');
				end
				else begin
					ant^.sig:=nuevo;
					nuevo^.sig:=act;
					writeln('insertar ordenado 6');
				end;
			end;
	begin
		writeln('genere lista');
		if(a<>nil) then begin
		
			if(a^.elem.montoRecaudado >= v1{min}) then begin
			
				if(a^.elem.montoRecaudado <= v2{max}) then begin
					insertarOrdenado(l,a);
					generarLista(l,v1,v2,a^.hI);
					generarLista(l,v1,v2,a^.hD);
				end
				
				else 
					generarLista(l,v1,v2,a^.hI);
			end
			else
				generarLista(l,v1,v2,a^.hD);
		end;
	end;
	
	
	procedure cantRecitales(l:lista; var cantr:integer);
	begin
		if(l<>nil) then  begin
			if(l^.elem.cantCanciones >= 12 ) then 
				cantr:=cantr +1;
			cantRecitales(l^.sig,cantr);
		end;
	end;
	
var
a:arbol;
l:lista;
valor1:integer;
valor2:integer; 
cantr:integer;
begin
	cantr:=0;
	a:=nil;
	l:=nil;
	generarArbol(a);
	writeln('ingresar primer valor:');
	readln(valor1);
	writeln;
	writeln('íngrese segundo valor:');
	readln(valor2);
	generarLista(l,valor1,valor2,a);
	cantRecitales(l,cantr);
	writeln('la cantidad de recitales que tocaron mas de 12 canciones fueron: ', cantr);
end.
	
