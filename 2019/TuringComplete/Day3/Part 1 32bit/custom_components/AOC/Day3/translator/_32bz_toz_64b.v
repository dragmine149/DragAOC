module _32bz_toz_64b (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input_1;
  input  wire [31:0] Input_2;
  output  wire [63:0] Output;

  TC_Maker64 # (.UUID(64'd1510885484616856289 ^ UUID)) Maker64_0 (.in0(wire_10), .in1(wire_4), .in2(wire_8), .in3(wire_1), .in4(wire_6), .in5(wire_0), .in6(wire_2), .in7(wire_7), .out(wire_9));
  TC_Splitter32 # (.UUID(64'd3902734069262788430 ^ UUID)) Splitter32_1 (.in(wire_3), .out0(wire_10), .out1(wire_4), .out2(wire_8), .out3(wire_1));
  TC_Splitter32 # (.UUID(64'd3428101714179931614 ^ UUID)) Splitter32_2 (.in(wire_5), .out0(wire_6), .out1(wire_0), .out2(wire_2), .out3(wire_7));
  TC_Constant # (.UUID(64'd481172826707387999 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());

  wire [7:0] wire_0;
  wire [7:0] wire_1;
  wire [7:0] wire_2;
  wire [31:0] wire_3;
  assign wire_3 = Input_2;
  wire [7:0] wire_4;
  wire [31:0] wire_5;
  assign wire_5 = Input_1;
  wire [7:0] wire_6;
  wire [7:0] wire_7;
  wire [7:0] wire_8;
  wire [63:0] wire_9;
  assign Output = wire_9;
  wire [7:0] wire_10;

endmodule
