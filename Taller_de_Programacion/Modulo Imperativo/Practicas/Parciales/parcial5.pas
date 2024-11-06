program parcial5;
type

	dataFecha = record
		dia:integer;
		mes:integer;
		anio:integer;
	end;
	
	pedido = record
		dni:integer;
		nombre:string;
		fecha:dataFecha;
		monto:real;
	end;
	
	pedidosLista = record
		fecha:dataFecha;
		monto:real;
	end;
	
	listaPedidos = ^nodoPedidos;
	
	nodoPedidos = record
		elem:pedidosLista;
		sig:listaPedidos;
	end;
	
	arbol = ^nodo;
	
	nodo = record
		dni:integer;
		nombre:string;
		lista:listaPedidos;
		hI:arbol;
		hD:arbol;
	end;
	
	procedure generarArbol(var a:arbol);
	
		procedure leerElem(var elem:pedido);
		begin
			elem.monto:=random(10);
			writeln(elem.monto);
			if(elem.monto <> 0) then begin
				elem.dni:=random(10)+1;
				writeln('dni',elem.dni);
				readln(elem.nombre);
				elem.fecha.dia:=random (2) +1;
				writeln('dia: ',elem.fecha.dia);
				elem.fecha.mes:=random (2) +1;
				writeln('mes : ',elem.fecha.mes);
				elem.fecha.anio:=random(2)+1;
				writeln('anio: ',elem.fecha.anio);
			end;
		end;
		
		procedure llenarArbol(var a:arbol; elem:pedido);
		
			
			procedure agregarAdelante( var l:listaPedidos; elem:pedido);
			var
			nuevo:listaPedidos;
			begin
				writeln('agrego a la lista');
				new(nuevo);
				nuevo^.elem.fecha.dia:= elem.fecha.dia;
				nuevo^.elem.fecha.mes:= elem.fecha.mes;
				nuevo^.elem.fecha.anio:= elem.fecha.anio;
				nuevo^.elem.monto:= elem.monto;
				if (l = nil) then l:= nuevo
				else begin
					nuevo^.sig:=l;
					l:=nuevo;
				end;
				
			end;
		begin
			if(a = nil) then begin
				new(a); a^.dni:= elem.dni; a^.nombre:=elem.nombre;
				a^.lista:=nil;
				agregarAdelante(a^.lista, elem);
				a^.hI:=nil;a^.hD:=nil;
			end
			else if(elem.dni < a^.dni) then llenarArbol(a^.hI,elem)
			else llenarArbol(a^.hD,elem)
		end;
		
	var
		elem:pedido;
	begin
		leerElem(elem);
		while(elem.monto <> 0)  begin
			llenarArbol(a,elem);
			leerElem(elem);
		end;
	end;
	
	procedure cantidadClientes(a:arbol; fechaR:dataFecha; var cant:integer);
	
		function buscarLista(l:listaPedidos ;fechaR:dataFecha):boolean;
		begin
			buscarLista:=false;
			while(l<>nil) do begin
				if(fechaR.anio = l^.elem.fecha.anio) and (fechaR.mes = l^.elem.fecha.mes) and (fechaR.dia = l^.elem.fecha.dia) then 
							buscarLista:=true;
				l:=l^.sig;
			end;
		end;
		
	begin
		if(a<>nil) then begin
			if (buscarLista(a^.lista,fechaR)) then 
				cant:=cant+1;
			cantidadClientes(a^.hI,fechaR,cant);
			cantidadClientes(a^.hD,fechaR,cant);
		end;
	end;
	
//INCISO C: ingresar un dni e informar total abonado y total de pedidos realizados por ese cliente
procedure recorrerLista(l:listaPedidos; var totalAbonado:real; var cantPedidos:integer);
begin
	while (l<>nil) do
	begin
		totalAbonado:= totalAbonado + l^.elem.monto;
		cantPedidos:= cantPedidos+1;
		l:=l^.sig;
	end;
end;
procedure recorrerArbol2(a:arbol; dni:integer; var totalAbonado:real; var cantPedidos:integer);
begin
	if (a<>nil) then
	begin
		if (a^.dni = dni) then
			recorrerLista(a^.lista,totalAbonado,cantPedidos)
		else if (dni<a^.dni) then
			recorrerArbol2(a^.HI,dni,totalAbonado,cantPedidos)
			else recorrerArbol2(a^.HD,dni,totalAbonado,cantPedidos);
	end;
end;
procedure incisoC(a:arbol);
var
	totalAbonado: real;
	dni:integer;
	cantPedidos:integer;
begin
	totalAbonado:= 0; cantPedidos:= 0;
	writeln('Ingrese un dni a buscar: ');
	readln(dni);
	recorrerArbol2(a,dni,totalAbonado,cantPedidos);
	writeln('El DNI: ',dni);writeln('Tuvo un total de: ',cantPedidos,' pedidos.');writeln('Con un total abonado de: ',totalAbonado:0:2);
end;
	
	
	
var
a:arbol;
fechaR:dataFecha;
cant:integer;
begin
	randomize;
	a:=nil;
	cant:=0;
	generarArbol(a);
	writeln('ingrese dia: ');
	readln(fechaR.dia);
	writeln('ingrese mes: ');
	readln(fechaR.mes);
	writeln('ingrese anio: ');
	readln(fechaR.anio);
	cantidadClientes(a,fechaR,cant);
	writeln('la cantidad de clientes fue :', cant);
	incisoC(a);
end.
		
