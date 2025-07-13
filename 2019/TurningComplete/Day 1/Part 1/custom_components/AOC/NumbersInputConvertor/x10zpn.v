module x10zpn (clk, rst, Full, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Full;
  input  wire [7:0] Input;
  output  wire [63:0] Output;

  TC_Constant # (.UUID(64'd1268553956133356806 ^ UUID), .BIT_WIDTH(64'd8), .value(8'hA)) Constant8_0 (.out(wire_5));
  TC_Mul # (.UUID(64'd1904265095000257882 ^ UUID), .BIT_WIDTH(64'd64)) Mul64_1 (.in0(wire_0), .in1({{56{1'b0}}, wire_5 }), .out0(wire_7), .out1());
  TC_Add # (.UUID(64'd4017176532664804448 ^ UUID), .BIT_WIDTH(64'd64)) Add64_2 (.in0(wire_7), .in1({{56{1'b0}}, wire_4 }), .ci(1'd0), .out(wire_2), .co());
  TC_Splitter8 # (.UUID(64'd4131343426059040226 ^ UUID)) Splitter8_3 (.in(wire_4), .out0(), .out1(), .out2(), .out3(), .out4(wire_6), .out5(), .out6(), .out7());
  TC_Not # (.UUID(64'd4040247262189621895 ^ UUID), .BIT_WIDTH(64'd1)) Not_4 (.in(wire_6), .out(wire_1));
  TC_Mux # (.UUID(64'd2133626856822857805 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_5 (.sel(wire_1), .in0(wire_0), .in1(wire_2), .out(wire_3));

  wire [63:0] wire_0;
  assign wire_0 = Full;
  wire [0:0] wire_1;
  wire [63:0] wire_2;
  wire [63:0] wire_3;
  assign Output = wire_3;
  wire [7:0] wire_4;
  assign wire_4 = Input;
  wire [7:0] wire_5;
  wire [0:0] wire_6;
  wire [63:0] wire_7;

endmodule
