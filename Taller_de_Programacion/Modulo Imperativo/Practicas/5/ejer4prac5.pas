program ejer4prac5;

type
	
	reclamo = record
		code:integer;
		dni:integer;
		anio:integer;
		tipoReclamo:string;
	end;
	
	reclamosLista = record
		tipoReclamo:string;
		code:integer;
		anio:integer;
	end;
	
	listaReclamos = ^nodoReclamos;
	
	nodoReclamos = record
		elem:reclamosLista;
		sig:listaReclamos;
	end;
	
	
	arbol = ^nodo;
	
	nodo = record
	
		dni:integer;
		cantTotal:integer;
		lista:listaReclamos;
		hI:arbol;
		hD:arbol;
		
	end;
	
	
	procedure generarArbol(var a:arbol);
	
		procedure leerReclamo(var elem:reclamo);
		begin
			writeln('ingresar codigo de reclamo: ');
			readln(elem.code);
			if(elem.code <> 0) then begin
				writeln('ingrese dni');
				readln(elem.dni);
				writeln('ingrese anio del reclamo');
				readln(elem.anio);
				writeln('ingrese tipo de reclamo');
				readln(elem.tipoReclamo);
			end;
		end;
		
		procedure cargarArbol(var a:arbol; elem:reclamo);
		
			procedure adaptarReclamo(elem:reclamo; var elem2:reclamosLista);
			begin
				elem2.code:=elem.code;
				elem2.anio:= elem.anio;
				elem2.tipoReclamo:= elem.tipoReclamo;
			end;
			
			procedure agregarAdelante(var l:listaReclamos; elem2:reclamosLista);
			var
			nuevo:listaReclamos;
			begin
				new(nuevo); nuevo^.elem:=elem2; nuevo^.sig:=nil;
				if(l =nil) then l:=nuevo
				else begin
					nuevo^.sig:=l;
					nuevo:=l;
				end;
			end;
			
		var
		elem2:reclamosLista;
		begin
			if(a = nil) then begin
				new(a); a^.dni:=elem.dni; a^.cantTotal:=1; a^.lista:=nil; a^.hI:=nil; a^.hD:=nil;
				adaptarReclamo(elem,elem2);
				agregarAdelante(a^.lista,elem2);
			end
			else
				if(elem.dni < a^.dni) then cargarArbol(a^.hI,elem) else
				if(elem.dni = a^.dni) then begin
					adaptarReclamo(elem,elem2);
					agregarAdelante(a^.lista,elem2); a^.cantTotal:=(a^.cantTotal + 1);
				end
				else cargarArbol(a^.hD,elem);
		end;
		
	var 
	
	elem:reclamo;
	
	begin
		leerReclamo(elem);
		if(elem.code<>0) then begin
			cargarArbol(a,elem);
			generarArbol(a);
		end;
	end;
	
	procedure cantReclamosEntre(a:arbol; min,max:integer; var cant:integer);
	begin
		if (a<>nil) then begin
			if(a^.dni >  min) then begin
				if(a^.dni < max) then begin
					cant:=(cant+a^.cantTotal);
					cantReclamosEntre(a^.hI,min,max,cant);
					cantReclamosEntre(a^.hD,min,max,cant);
				end
				else
				cantReclamosEntre(a^.hI,min,max,cant);
			end
			else 
				cantReclamosEntre(a^.hD,min,max,cant);
		end;
	end;
	
	procedure cantReclamos(a:arbol; dni:integer; var cant:integer);
	begin
		if (a<>nil) then begin
			if(dni < a^.dni) then cantReclamos(a^.hI,dni,cant) else
			if(dni = a^.dni) then cant:=a^.cantTotal else
			cantReclamos(a^.hD,dni,cant);
		end;
	end;
	
	procedure cantXanios(a:arbol; anior:integer);
	
		procedure recorrerLista(l:listaReclamos; anior:integer);
		begin
			while (l <>nil) do begin
				if(l^.elem.anio = anior) then
					writeln(l^.elem.code);
				l:=l^.sig;
			end;
		end;
	begin
		if (a <> nil) then begin
			recorrerLista(a^.lista,anior);
			cantXanios(a^.hI,anior);
			cantXanios(a^.hD,anior);
		end;
	end;
var

a:arbol;
dni1:integer;
dni2:integer;
max:integer;
min:integer;
cant:integer;
anior:integer;
begin
	cant:=0;
	a := nil;
	generarArbol(a);
	writeln('ingrese el primer dni');
	readln(dni1);
	writeln;
	writeln('ingrese el segundo dni');
	readln(dni2);
	if(dni1 >  dni2) then begin
		max:=dni1;
		min:=dni2;
	end
	else begin
		max:=dni2;
		min:=dni1;
	end;
	cantReclamosEntre(a,min,max,cant);
	writeln('la cantidad de reclamos que estan entre,', min,' y  ', max,' son: ',cant);
	cant:=0;
	writeln;
	writeln('ingrese otro dni para ver todos los reclamos que tiene:');
	readln(dni1);
	cantReclamos(a,dni1,cant);
	writeln('la cantidad de reclamos de', dni1,'son:' ,cant);
	writeln;
	writeln('ingrese anio para ver los codigos de los reclamos: ');
	readln(anior);
	cantXanios(a,anior); 
end.
	
	
	
	
	
	
	
	
