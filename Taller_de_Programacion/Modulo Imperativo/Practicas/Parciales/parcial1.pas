program parcial1;
type

	dataMes = 1.. 12;
	
	alquiler = record
		dni:integer;
		patente:integer;
		mes:dataMes;
		anio:integer;
	end;
	
	alquileresLista = record
		dni:integer;
		mes:dataMes;
		anio:integer;
	end;
	
	
	listaAlquileres = ^nodoAlquileres;
	
	nodoAlquileres = record
		elem:alquileresLista;
		sig:listaAlquileres;
	end;
	
	arbol = ^nodo;
	
	nodo = record
		patente:integer;
		lista:listaAlquileres;
		hI:arbol;
		hD:arbol;
	end;
	
	procedure generarArbol(var a:arbol);
	
		procedure leerElemento(var elem:alquiler);
		begin
			writeln('ingrese patente');
			readln(elem.patente);
			if( elem.patente <> 0) then begin
				writeln('ingrese dni');
				readln(elem.dni);
				writeln('íngrese mes');
				readln(elem.mes);
				writeln('ingrese anio');
				readln(elem.anio);
			end;
		end;
		
		procedure cargarArbol(var a:arbol; elem:alquiler);
		
		
			procedure adaptarElem(elem:alquiler; var elem2:alquileresLista);
			begin
				elem2.dni:= elem.dni;
				elem2.mes:=elem.mes;
				elem2.anio:=elem.anio;
			end;
			
			
			procedure agregarAdelante(var l:listaAlquileres; elem2:alquileresLista);
			var
			nuevo:listaAlquileres;
			begin
				new(nuevo); nuevo^.elem:= elem2; nuevo^.sig:=nil;
				if(l= nil) then l:=nuevo
				else begin
					nuevo^.sig:=l;
					l:=nuevo;
				end;
			end;
			
		var
			elem2:alquileresLista;
		begin
			if(a = nil) then begin
				new(a); a^.patente:=elem.patente; a^.lista:=nil; a^.hI:=nil; a^.hD:=nil;
				adaptarElem(elem,elem2);
				agregarAdelante(a^.lista,elem2);
			end
			else if(elem.patente < a^.patente) then cargarArbol(a^.hI,elem)
			else if(elem.patente = a^.patente) then begin
				adaptarElem(elem,elem2);
				agregarAdelante(a^.lista,elem2);
			end
			else cargarArbol(a^.hD,elem);
		end;
	var
	elem:alquiler;
	begin
		leerElemento(elem);
		if(elem.patente <> 0) then begin
			cargarArbol(a,elem);
			generarArbol(a);
		end;
	end;
	
	
	procedure contarAlquileres(a:arbol; dnid:integer; var cant:integer);
	
		procedure recorrerLista( l:listaAlquileres; dnid:integer; var cant:integer);
		begin
			while (l<>nil) do begin
				if(l^.elem.dni = dnid) then 
					cant:=cant+1;
				l:=l^.sig;
			end;
		end;
		
	begin
		if(a<>nil) then begin
			recorrerLista(a^.lista,dnid,cant);
			contarAlquileres(a^.hI,dnid,cant);
			contarAlquileres(a^.hD,dnid,cant);
		end;
	end;
	
	procedure cantAlquileresEntreAyB(a:arbol; pa:integer; pb:integer; x:integer; var cant:integer);
	
			procedure buscoLista(l:listaAlquileres; x:integer; var cant:integer);
			begin
				while (l<>nil) do begin
					if(l^.elem.anio = x) then 
						cant:=cant +1;
					l:=l^.sig;
				end;
			end;
			
	begin
		if(a<>nil) then begin
			if(a^.patente >= pa) then begin
				if(a^.patente <= pb) then begin
					buscoLista(a^.lista,x, cant);
					cantAlquileresEntreAyB(a^.hI,pa,pb,x,cant);
					cantAlquileresEntreAyB(a^.hD,pa,pb,x,cant);
				end
				else
					cantAlquileresEntreAyB(a^.hI,pa,pb,x,cant);
			end
			else
				cantAlquileresEntreAyB(a^.hD,pa,pb,x,cant);
		end;
	end;
var
a:arbol;
dnid:integer;
cant:integer;
patenteA:integer;
patenteB:integer;
aniox:integer;
begin
	cant:=0;
	a:=nil;
	generarArbol(a);
	writeln('ingrese dni para ver los alquileres que tiene: ');
	readln(dnid);
	contarAlquileres(a,dnid,cant);
	writeln('el dni', dnid, 'la cantidad de alquileres es:', cant);
	writeln;
	writeln('ingrese un anio:');
	readln(aniox);
	writeln;
	writeln('ingrese patente A');
	readln(patenteA);
	writeln;
	writeln('ingrese patente B');
	readln(patenteB);
	cant:=0;
	cantAlquileresEntreAyB(a,patenteA,patenteB,aniox,cant);
	writeln('la cantidad de alquileres entre las patentes y el anio ees: ', cant);
end.
		
	
	
	

