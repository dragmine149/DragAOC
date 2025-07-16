module x10zpnz_2 (clk, rst, Full, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Full;
  input  wire [7:0] Input;
  output  wire [63:0] Output;

  TC_Constant # (.UUID(64'd1268553956133356806 ^ UUID), .BIT_WIDTH(64'd8), .value(8'hA)) Constant8_0 (.out(wire_7));
  TC_Mul # (.UUID(64'd1904265095000257882 ^ UUID), .BIT_WIDTH(64'd64)) Mul64_1 (.in0(wire_0), .in1({{56{1'b0}}, wire_7 }), .out0(wire_4), .out1());
  TC_Mux # (.UUID(64'd2133626856822857805 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_2 (.sel(wire_1), .in0(wire_0), .in1(wire_3), .out(wire_6));
  TC_Not # (.UUID(64'd1528502635420248301 ^ UUID), .BIT_WIDTH(64'd1)) Not_3 (.in(wire_2), .out(wire_1));
  _8bgt10 # (.UUID(64'd2075555331620188666 ^ UUID)) _8bgt10_4 (.clk(clk), .rst(rst), .Input(wire_5), .Greater_than_10(wire_2));
  TC_Add # (.UUID(64'd4017176532664804448 ^ UUID), .BIT_WIDTH(64'd64)) Add64_5 (.in0(wire_4), .in1({{56{1'b0}}, wire_5 }), .ci(1'd0), .out(wire_3), .co());

  wire [63:0] wire_0;
  assign wire_0 = Full;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [63:0] wire_3;
  wire [63:0] wire_4;
  wire [7:0] wire_5;
  assign wire_5 = Input;
  wire [63:0] wire_6;
  assign Output = wire_6;
  wire [7:0] wire_7;

endmodule
