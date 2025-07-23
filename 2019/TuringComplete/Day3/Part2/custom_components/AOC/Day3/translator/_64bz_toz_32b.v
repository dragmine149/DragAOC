module _64bz_toz_32b (clk, rst, Input, Output_1, Output_2);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [31:0] Output_1;
  output  wire [31:0] Output_2;

  TC_Constant # (.UUID(64'd481172826707387999 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_0 (.out());
  TC_Maker32 # (.UUID(64'd1313187820457327494 ^ UUID)) Maker32_1 (.in0(wire_7), .in1(wire_1), .in2(wire_6), .in3(wire_5), .out(wire_2));
  TC_Maker32 # (.UUID(64'd3904101815136823351 ^ UUID)) Maker32_2 (.in0(wire_4), .in1(wire_9), .in2(wire_10), .in3(wire_0), .out(wire_3));
  TC_Splitter64 # (.UUID(64'd635915171221099122 ^ UUID)) Splitter64_3 (.in(wire_8), .out0(wire_7), .out1(wire_1), .out2(wire_6), .out3(wire_5), .out4(wire_4), .out5(wire_9), .out6(wire_10), .out7(wire_0));

  wire [7:0] wire_0;
  wire [7:0] wire_1;
  wire [31:0] wire_2;
  assign Output_2 = wire_2;
  wire [31:0] wire_3;
  assign Output_1 = wire_3;
  wire [7:0] wire_4;
  wire [7:0] wire_5;
  wire [7:0] wire_6;
  wire [7:0] wire_7;
  wire [63:0] wire_8;
  assign wire_8 = Input;
  wire [7:0] wire_9;
  wire [7:0] wire_10;

endmodule
