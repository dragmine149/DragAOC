module ReverseSteps (clk, rst, LineEnd, InterceptPoint, Direction, StepCount);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] LineEnd;
  input  wire [63:0] InterceptPoint;
  input  wire [0:0] Direction;
  output  wire [63:0] StepCount;

  TC_Add # (.UUID(64'd2838636158830482426 ^ UUID), .BIT_WIDTH(64'd32)) Add32_0 (.in0(wire_2), .in1(wire_3), .ci(1'd0), .out(wire_10), .co());
  TC_Neg # (.UUID(64'd3660648889661055399 ^ UUID), .BIT_WIDTH(64'd32)) Neg32_1 (.in(wire_0), .out(wire_3));
  TC_Mux # (.UUID(64'd1996254567277932005 ^ UUID), .BIT_WIDTH(64'd32)) Mux32_2 (.sel(wire_1), .in0(wire_9), .in1(wire_4), .out(wire_0));
  TC_Mux # (.UUID(64'd2888710434002973308 ^ UUID), .BIT_WIDTH(64'd32)) Mux32_3 (.sel(wire_1), .in0(wire_8), .in1(wire_5), .out(wire_2));
  _64bz_toz_32b # (.UUID(64'd837787252922062774 ^ UUID)) _64bz_toz_32b_4 (.clk(clk), .rst(rst), .Input(wire_6), .Output_1(wire_5), .Output_2(wire_8));
  _64bz_toz_32b # (.UUID(64'd3546631969497301374 ^ UUID)) _64bz_toz_32b_5 (.clk(clk), .rst(rst), .Input(wire_7), .Output_1(wire_4), .Output_2(wire_9));

  wire [31:0] wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Direction;
  wire [31:0] wire_2;
  wire [31:0] wire_3;
  wire [31:0] wire_4;
  wire [31:0] wire_5;
  wire [63:0] wire_6;
  assign wire_6 = LineEnd;
  wire [63:0] wire_7;
  assign wire_7 = InterceptPoint;
  wire [31:0] wire_8;
  wire [31:0] wire_9;
  wire [31:0] wire_10;
  assign StepCount = {{32{1'b0}}, wire_10 };

endmodule
