module ManhattanDistance (clk, rst, X, Y, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] X;
  input  wire [31:0] Y;
  output  wire [31:0] Output;

  TC_Constant # (.UUID(64'd4235744232402661516 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_0 (.out());
  _32bz_ABS # (.UUID(64'd2867156655478424944 ^ UUID)) _32bz_ABS_1 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_0));
  _32bz_ABS # (.UUID(64'd497840059967605316 ^ UUID)) _32bz_ABS_2 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_4));
  TC_Add # (.UUID(64'd1833361401759064682 ^ UUID), .BIT_WIDTH(64'd32)) Add32_3 (.in0(wire_0), .in1(wire_4), .ci(1'd0), .out(wire_3), .co());

  wire [31:0] wire_0;
  wire [31:0] wire_1;
  assign wire_1 = Y;
  wire [31:0] wire_2;
  assign wire_2 = X;
  wire [31:0] wire_3;
  assign Output = wire_3;
  wire [31:0] wire_4;

endmodule
