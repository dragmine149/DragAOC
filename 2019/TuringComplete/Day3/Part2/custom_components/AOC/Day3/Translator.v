module Translator (clk, rst, Input_1, Input_2, Input_3, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] Input_1;
  input  wire [0:0] Input_2;
  input  wire [0:0] Input_3;
  output  wire [31:0] Output;

  TC_Maker32 # (.UUID(64'd4591680081976350286 ^ UUID)) Maker32_0 (.in0(wire_4), .in1(wire_0), .in2(8'd0), .in3(wire_6), .out(wire_5));
  TC_Splitter16 # (.UUID(64'd4448882331349273330 ^ UUID)) Splitter16_1 (.in(wire_1), .out0(wire_4), .out1(wire_0));
  TC_Maker8 # (.UUID(64'd3415293894943623914 ^ UUID)) Maker8_2 (.in0(wire_3), .in1(wire_2), .in2(1'd0), .in3(1'd0), .in4(1'd0), .in5(1'd0), .in6(1'd0), .in7(1'd0), .out(wire_6));
  TC_Constant # (.UUID(64'd1357728289607731106 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());

  wire [7:0] wire_0;
  wire [15:0] wire_1;
  assign wire_1 = Input_1;
  wire [0:0] wire_2;
  assign wire_2 = Input_3;
  wire [0:0] wire_3;
  assign wire_3 = Input_2;
  wire [7:0] wire_4;
  wire [31:0] wire_5;
  assign Output = wire_5;
  wire [7:0] wire_6;

endmodule
