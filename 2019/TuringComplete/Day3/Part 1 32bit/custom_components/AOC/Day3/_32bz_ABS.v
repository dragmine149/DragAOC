module _32bz_ABS (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input;
  output  wire [31:0] Output;

  TC_IndexerBit # (.UUID(64'd720917380837010060 ^ UUID), .INDEX(64'd31)) IndexerBit_0 (.in({{32{1'b0}}, wire_0 }), .out(wire_3));
  TC_Mux # (.UUID(64'd1795728675512157787 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_1 (.sel(wire_3), .in0({{32{1'b0}}, wire_0 }), .in1({{32{1'b0}}, wire_4 }), .out(wire_1));
  TC_Mul # (.UUID(64'd2282237337778252309 ^ UUID), .BIT_WIDTH(64'd32)) Mul32_2 (.in0(wire_0), .in1(wire_2), .out0(wire_4), .out1());
  TC_Constant # (.UUID(64'd2592145288161180252 ^ UUID), .BIT_WIDTH(64'd32), .value(32'hFFFFFFFF)) Constant32_3 (.out(wire_2));

  wire [31:0] wire_0;
  assign wire_0 = Input;
  wire [63:0] wire_1;
  assign Output = wire_1[31:0];
  wire [31:0] wire_2;
  wire [0:0] wire_3;
  wire [31:0] wire_4;

endmodule
