{3. Un supermercado requiere el procesamiento de sus productos. De cada producto se conoce código, rubro (1..10), stock y precio unitario. Se pide:
a) Generar una estructura adecuada que permita agrupar los productos por rubro.
A su vez, para cada rubro, se requiere que la búsqueda de un producto por código sea lo más eficiente posible.
El ingreso finaliza con el código de producto igual a 0.
b) Implementar un módulo que reciba la estructura generada en a), un rubro y un código de producto y retorne si dicho código existe o no para ese rubro.
c)Implementar un módulo que reciba la estructura generada en a), y retorne, para cada rubro, el código y stock del producto con mayor código
d) Implementar un módulo que reciba la estructura generada en a), dos códigos y retorne, para cada rubro,
la cantidad de productos con códigos entre los dos valores ingresados.
}
program ejer3prac5;

type

    dataRubro = 1 .. 10;

    producto = record
        codigo:integer;
        stock:integer;
        rubro:dataRubro;
        precioUnitario:real;
    end;



    arbol = ^nodo;

    nodo= record
        elem:producto;
        hI:arbol;
        hD:arbol;
    end;

    vectorArbol = array [dataRubro] of arbol;
    
    
    procedure inicializarVector(var v:vectorArbol);
    var
		i:integer;
	begin
		for i:= 1 to 10 do
			v[i]:=nil;
	end;
	
	procedure cargarVectorArbol(var v:vectorArbol);
	
		procedure leerProducto(var elem:producto);
		begin
			writeln('ingrese codigo: ');
			readln(elem.codigo);
			if(elem.codigo <>0) then begin
				writeln('ingrese stock: ');
				readln(elem.stock);
				writeln('ingrese rubro');
				readln(elem.rubro);
				writeln('precio unitario: ');
				readln(elem.precioUnitario);
			end;
		end;
		
		procedure llenarArbol(var a:arbol; elem:producto);
		begin
			if(a= nil) then begin
				new(a); a^.elem.codigo:= elem.codigo;
				a^.elem.stock:=elem.stock; a^.elem.precioUnitario:=elem.precioUnitario;
				a^.hI:=nil; a^.hD:=nil;
			end
			else 
				if(elem.codigo < a^.elem.codigo) then llenarArbol(a^.hI, elem)
				else if (elem.codigo > a^.elem.codigo) then llenarArbol(a^.hD,elem)
		end;

		
	var
	elem:producto;
	begin
		leerProducto(elem);
		if(elem.codigo <> 0 ) then begin
			llenarArbol(v[elem.rubro], elem);
			cargarVectorArbol(v);
		end;
		
	end;
	
	
	procedure buscarProduc(v:vectorArbol; rubror:dataRubro; codigor:integer);
	
		function buscar(a:arbol; codigor:integer):boolean;
		begin
			buscar:=false;
			if (a <> nil) then begin
				if(codigor < a^.elem.codigo) then buscar:=buscar(a^.hI,codigor)
				else if( codigor = a^.elem.codigo) then buscar:=true
				else buscar:=buscar(a^.hD, codigor)
			end;
		end;
		
	begin
		if(buscar(v[rubror],codigor))then
			writeln('existe')
		else
			writeln('no existe');			
	end;
    
    procedure calcularMayor(v:vectorArbol);
    
		procedure buscarMax(a:arbol; var max:integer; var stockmax:integer);
		begin
			if (a<>nil) then begin
				if(a^.elem.codigo > max) then begin
					max:=a^.elem.codigo;
					stockmax:=a^.elem.stock;
					buscarMax(a^.hI,max,stockmax);
					buscarMax(a^.hD,max,stockmax);						
				end
				else
					buscarMax(a^.HD,max,stockmax);				
			end;
		end;
		
    var
    i:integer;
    max:integer;
    stockmax:integer;
	begin
		for i:= 1 to 10 do begin
			max:=-99;
			stockmax:=0;
			buscarMax(v[i],max,stockmax);
			writeln('el codigo max fue :', max);
			writeln;
			writeln('el stock max fue :', stockmax);
		end;
	end;
    
    procedure cantidadProductos(v:vectorArbol; min:integer; max:integer);
    
		procedure buscarCant(a:arbol; min,max:integer; var cant:integer);
		begin
			if (a<>nil) then begin
			
				if(a^.elem.codigo > min) then begin
					if(a^.elem.codigo < max) then begin
						cant:=cant +1;
						buscarCant(a^.hI,min,max,cant);
						buscarCant(a^.hD,min,max,cant);
					end
					else
					buscarCant(a^.hI,min,max,cant);
				end
				else
					buscarCant(a^.hD,min,max,cant);
			end;
		end;
		
    var
    
    i:integer;
    cant:integer;
    
    begin
		for i:=1 to 10 do begin
			cant:=0;
			buscarCant(v[i],min,max,cant);
			writeln('la cantidad que hay en el rubro', i , 'son: ',cant);
		end;
    end;
    
    
var
v:vectorArbol;
rubror:dataRubro;
codigor:integer;
code1:integer;
code2:integer;
max:integer;
min:integer;
begin
	inicializarVector(v);
	cargarVectorArbol(v);
	writeln('ingrese un rubro donde buscar: ');
	readln(rubror);
	writeln;
	writeln('ingrese el codigo del producto para ver si existe: ');
	readln(codigor);
	buscarProduc(v,rubror,codigor);	
	calcularMayor(v);
	writeln;
	writeln('ingrese codigo 1: ');
	readln(code1);
	writeln;
	writeln('ingrese el codigo 2: ');
	readln(code2);
	if(code1>code2) then begin
		max:= code1;
		min:=code2;
	end
	else begin
		max:=code2;
		min:=code1;
	end;
	cantidadProductos(v,min,max);
end.
    

        

