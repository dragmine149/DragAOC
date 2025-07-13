module newlineDetector (clk, rst, \Number_a._-45 , Index, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] \Number_a._-45 ;
  input  wire [7:0] Index;
  output  wire [7:0] Output;

  TC_Equal # (.UUID(64'd1641105168902774574 ^ UUID), .BIT_WIDTH(64'd8)) Equal8_0 (.in0(wire_0), .in1(wire_3), .out(wire_1));
  TC_Constant # (.UUID(64'd1804371855211244039 ^ UUID), .BIT_WIDTH(64'd8), .value(8'hDA)) Constant8_1 (.out(wire_3));
  TC_Switch # (.UUID(64'd3972616882035362788 ^ UUID), .BIT_WIDTH(64'd8)) Output8z_2 (.en(wire_1), .in(wire_2), .out(Output));

  wire [7:0] wire_0;
  assign wire_0 = \Number_a._-45 ;
  wire [0:0] wire_1;
  wire [7:0] wire_2;
  assign wire_2 = Index;
  wire [7:0] wire_3;

endmodule
