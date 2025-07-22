uses graphabc;

procedure draw(n: integer; x1, y1: real; px, py: integer);
var
  x0, y0, temp: real;
begin
  x0 := x1;
  y0 := y1;
  
  while (x1 * x1 + y1 * y1 < 4) and (n > 0) do 
  begin
    temp := x1 * x1 - y1 * y1;
    y1 := 2 * x1 * y1;
    x1 := temp;
    
    x1 := x1 + x0;
    y1 := y1 + y0;
    n := n - 1;
  end;

  if n < 2 then
    SetPixel(px, py, clBlack)
  else if n < 16 then
    SetPixel(px, py, clGreen)
  else if n < 32 then
    SetPixel(px, py, clLightGreen)
  else if n < 48 then
    SetPixel(px, py, clYellow)
  else if n < 64 then
    SetPixel(px, py, clLightYellow)
  else if n < 128 then
    SetPixel(px, py, clRed)
  else if n < 160 then
    SetPixel(px, py, clMagenta)
  else if n < 220 then
    SetPixel(px, py, clCyan)
  else if n < 253 then
    SetPixel(px, py, clBlue)
  else
    SetPixel(px, py, clWhite);
end;

var
  px, py, n: integer;
  x2, y2: real;

begin
  SetWindowSize(680, 680);
  SetWindowTitle('Mandelbrot Set');

  read(n);

  for px := 1 to 680 do
  begin
    for py := 1 to 680 do
    begin
      x2 := (px / 240) - 2;
      y2 := (py / 240) - 1.5;
      draw(n, x2, y2, px, py);
    end;
  end;
end.