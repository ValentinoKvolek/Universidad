program parcial3;

type
	
	dataDia = 1..31;
	dataMes = 1..12;

	compra = record
		codeCompra:integer;
		codeCliente:integer;
		dia:dataDia;
		mes:dataMes;
	end;
	
	vector = array [dataMes] of integer;
	
	
	lista = ^nodoLista;
	
	nodoLista = record
		elem:compra;
		sig:lista;
	end;
	
	arbol =^nodo;
	
	nodo = record
		elem:compra;
		hI:arbol;
		hD:arbol;
	end;
	
	
	
	
	
	procedure inicializarVector(var v:vector);
	var
	i:integer;
	begin
		for i:=1 to 12 do 
			v[i]:=0;
	end;
	
	procedure generalArbol(var a:arbol; var v:vector);
	
		procedure leerCompra(var elem:compra);
		begin
			elem.codeCliente:=Random(20);
			if(elem.codeCliente<> 0) then begin
				writeln(elem.codeCliente);
				elem.dia:=Random(31)+1;
				elem.mes:=Random(12)+1;
				elem.codeCompra:=Random(20000)+1;
			end;
		end;
		
		procedure llenarArbol(var a:arbol; elem:compra);
		begin
			if(a = nil) then begin
				new(a); a^.elem:=elem; a^.hI:=nil; a^.hD:=nil;
			end
			else if(elem.codeCliente < a^.elem.codeCliente) then llenarArbol(a^.hI,elem)
			else if (elem.codeCliente >= a^.elem.codeCliente) then llenarArbol(a^.hD,elem)
		end;
	
		procedure llenarVector(var v:vector ; elem:compra);
		begin
			v[elem.mes]:=v[elem.mes] + 1;
		end;
		
	var 
	elem:compra;
	begin
		leerCompra(elem);
		if(elem.codeCliente <> 0) then begin
			llenarArbol(a,elem);
			llenarVector(v,elem);
			generalArbol(a,v);
		end;
	end;

	procedure llenarLista(var l:lista; codeR:integer; a:arbol);
	
		procedure agregarAdelante(var l:lista; a:arbol);
		var
		nuevo:lista;
		begin
			new(nuevo); nuevo^.elem:= a^.elem; nuevo^.sig:=nil;
			if(l=nil) then l:=nuevo
			else begin
				nuevo^.sig:=l;
				l:=nuevo;
			end;
		end;
		
	begin	
		if(a<>nil) then begin
			if(codeR = a^.elem.codeCliente) then begin
					agregarAdelante(l,a);
					llenarLista(l,codeR,a^.hD);
				end
				else if(codeR < a^.elem.codeCliente) then llenarLista(l,codeR,a^.hI)
				else llenarLista(l,codeR,a^.hD)
		end;
	end;
	
	procedure recorrerLista(l:lista);
	begin
		while(l<>nil)do begin
			writeln('lista:');
			writeln(l^.elem.codeCompra);
			l:=l^.sig;
		end;
	end;
	
	procedure ordenarVector(var v:vector);
	var
		i,j,actual:integer;
	begin
		for i:=2 to 12 do begin
			actual:=v[i];
			j:= i-1;
			while (j > 0) and (v[j] < actual) do begin
				v[j+1]:=v[j];
				j:=j-1;
			end;
			v[j +1]:=actual;
		end;
		
	end;
	
var
a:arbol;
v:vector;
codeR:integer;
l:lista;
begin
	randomize;
	l:=nil;
	a:=nil;
	inicializarVector(v);
	generalArbol(a,v);
	writeln('ingrese un codigo: ');
	readln(codeR);
	llenarLista(l,codeR,a);
	recorrerLista(l);
	ordenarVector(v);
end.
	
	
	
	
	
	
