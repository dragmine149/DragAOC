module _16bz_ABS (clk, rst, X, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] X;
  output  wire [15:0] Output;

  TC_Mux # (.UUID(64'd1613219844841409961 ^ UUID), .BIT_WIDTH(64'd16)) Mux16_0 (.sel(wire_1), .in0(wire_0), .in1(wire_4), .out(wire_2));
  TC_Mul # (.UUID(64'd1980215757136314572 ^ UUID), .BIT_WIDTH(64'd16)) Mul16_1 (.in0(wire_0), .in1(wire_3), .out0(wire_4), .out1());
  TC_Constant # (.UUID(64'd3682617161905088756 ^ UUID), .BIT_WIDTH(64'd16), .value(16'hFFFF)) Constant16_2 (.out(wire_3));
  TC_IndexerBit # (.UUID(64'd720917380837010060 ^ UUID), .INDEX(64'd15)) IndexerBit_3 (.in({{48{1'b0}}, wire_0 }), .out(wire_1));

  wire [15:0] wire_0;
  assign wire_0 = X;
  wire [0:0] wire_1;
  wire [15:0] wire_2;
  assign Output = wire_2;
  wire [15:0] wire_3;
  wire [15:0] wire_4;

endmodule
