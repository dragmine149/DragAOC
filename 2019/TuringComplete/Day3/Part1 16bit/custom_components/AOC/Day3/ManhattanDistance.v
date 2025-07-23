module ManhattanDistance (clk, rst, Y, X, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] Y;
  input  wire [15:0] X;
  output  wire [31:0] Output;

  TC_Add # (.UUID(64'd1779712173377956486 ^ UUID), .BIT_WIDTH(64'd32)) Add32_0 (.in0({{16{1'b0}}, wire_4 }), .in1({{16{1'b0}}, wire_1 }), .ci(1'd0), .out(wire_0), .co());
  _16bz_ABS # (.UUID(64'd2527674122958302763 ^ UUID)) _16bz_ABS_1 (.clk(clk), .rst(rst), .X(wire_2), .Output(wire_4));
  _16bz_ABS # (.UUID(64'd3420927853212062601 ^ UUID)) _16bz_ABS_2 (.clk(clk), .rst(rst), .X(wire_3), .Output(wire_1));
  TC_Constant # (.UUID(64'd4235744232402661516 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());

  wire [31:0] wire_0;
  assign Output = wire_0;
  wire [15:0] wire_1;
  wire [15:0] wire_2;
  assign wire_2 = X;
  wire [15:0] wire_3;
  assign wire_3 = Y;
  wire [15:0] wire_4;

endmodule
