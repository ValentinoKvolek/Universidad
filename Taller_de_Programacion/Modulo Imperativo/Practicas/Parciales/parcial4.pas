
{Un supermercado quiere llevar un registro de todas sus ventas, de cada venta se lee el codigo, 
        dni del cliente, sucursal en la que compró (1..5) y monto gastado, la lectura se detiene con el código de venta 0, se pide:
A-  Crear una estructura eficiente para la busqueda por dni de cliente, de cada cliente se guarda el monto gastado en cada sucursal
B-    un módulo que reciba la estructura generada en A y un número de sucursal y retorne la cantidad de clientes que no gastaron nada en la sucursal
C-    un modulo que reciba la estructura generara en A y un dni y retorne el monto total general gastado por dicho dni (es decir, la suma de los montos de todas las sucursales)}



program parcial4;
type
	dataSucursal = 1..5;
	
	venta = record
		dni:integer;
		sucursal:dataSucursal;
		monto:real;
		codeVenta:integer;
	end;
	
	vectorContador = array [1..5] of real;
	
	arbol = ^nodo;
	
	nodo = record
		dni:integer;
		vc:vectorContador;
		hI:arbol;
		hD:arbol;
	end;
	
	procedure generarArbol( var a:arbol);
	
		procedure leerElemento(var elem:venta);
		begin 
			elem.codeVenta:=Random(10);
			if(elem.codeVenta <> 0 ) then begin
				elem.sucursal:=Random(5) +1;
				elem.monto:=Random(10)+1;
				elem.dni:=Random(2) +1;
			end;
		end;
		
		procedure llenarArbol(var a:arbol; elem:venta);
		
			procedure inicializarVector(var v:vectorContador);
			var
			i:integer;
			begin
				for i:=1 to 5 do 
					v[i]:=0;
			end;
			
		begin
			if(a = nil) then begin
				new(a); a^.dni:=elem.dni; a^.hI:=nil; a^.hD:=nil;
				inicializarVector(a^.vc);
				a^.vc[elem.sucursal]:=a^.vc[elem.sucursal] + elem.monto;
			end
			else if(elem.dni = a^.dni) then
				a^.vc[elem.sucursal]:=a^.vc[elem.sucursal] + elem.monto
			else if(elem.dni < a^.dni) then llenarArbol(a^.hI,elem)
			else  llenarArbol(a^.hD,elem)
		end;
		
	var
	elem:venta;
	begin
		leerElemento(elem);
		if(elem.codeVenta <> 0 ) then begin
			writeln('se lleno el arbol');
			llenarArbol(a,elem);
			generarArbol(a);
		end;
	end;
	
	
	procedure imprimir(a:arbol);
	var i:integer;
	begin
		for i:=1 to 5 do 
			writeln(a^.vc[i]);
	end;
	
	
	procedure incisoB(a:arbol; sucursalR:integer; var cant:integer);
	begin
		if(a<>nil) then begin
			if( a^.vc[sucursalR] = 0 ) then
				cant:=cant +1;
			incisoB(a^.hI,sucursalR,cant);
			incisoB(a^.hD,sucursalR,cant);
		end;
	end;
	
	procedure incisoC(a:arbol; dniR:integer; var cantR:real);
	
		procedure contarSucursales(vc:vectorContador; var cantR:real);
		var 
		i:integer;
		begin
			for i:=1 to 5 do 
				cantR:=cantR + vc[i];
		end;
	begin
		if(dniR = a^.dni) then begin
			contarSucursales(a^.vc,cantR);
		end
		else if (dniR < a^.dni) then incisoC(a^.hI,dniR,cantR)
		else incisoC(a^.hD,dniR,cantR)
	end;
var


a:arbol;
sucursalR:integer;
cant:integer;
dniR:integer;
cantR:real;

begin
	cant:=0;
	a:=nil;
	generarArbol(a);
	imprimir(a);
	writeln('ingrese sucursal:');
	readln(sucursalR);
	incisoB(a,sucursalR,cant);
	writeln('la cantidad es',cant);
	writeln;
	writeln('dni:');
	readln(dniR);
	cantR:=0;
	incisoC(a,dniR,cantR);
	writeln('LA CANTIDAD TOTAL: ',cantR);
end.







