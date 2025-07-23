module _64bToPosition (clk, rst, Input, EndY, EndX, StartY, StartX);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [15:0] EndY;
  output  wire [15:0] EndX;
  output  wire [15:0] StartY;
  output  wire [15:0] StartX;

  TC_Maker16 # (.UUID(64'd1427289234798709460 ^ UUID)) Maker16_0 (.in0(wire_4), .in1(wire_2), .out(wire_12));
  TC_Maker16 # (.UUID(64'd420177246880865096 ^ UUID)) Maker16_1 (.in0(wire_6), .in1(wire_9), .out(wire_10));
  TC_Maker16 # (.UUID(64'd2657859200953048592 ^ UUID)) Maker16_2 (.in0(wire_1), .in1(wire_0), .out(wire_3));
  TC_Maker16 # (.UUID(64'd1735076062783789716 ^ UUID)) Maker16_3 (.in0(wire_7), .in1(wire_11), .out(wire_5));
  TC_Splitter64 # (.UUID(64'd183390187747982228 ^ UUID)) Splitter64_4 (.in(wire_8), .out0(wire_7), .out1(wire_11), .out2(wire_1), .out3(wire_0), .out4(wire_6), .out5(wire_9), .out6(wire_4), .out7(wire_2));
  TC_Constant # (.UUID(64'd964380976153628756 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_5 (.out());
  TC_Constant # (.UUID(64'd1355829098517522774 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_6 (.out());
  TC_Constant # (.UUID(64'd2243695799942071691 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_7 (.out());
  TC_Constant # (.UUID(64'd3664674422989834658 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_8 (.out());

  wire [7:0] wire_0;
  wire [7:0] wire_1;
  wire [7:0] wire_2;
  wire [15:0] wire_3;
  assign StartY = wire_3;
  wire [7:0] wire_4;
  wire [15:0] wire_5;
  assign StartX = wire_5;
  wire [7:0] wire_6;
  wire [7:0] wire_7;
  wire [63:0] wire_8;
  assign wire_8 = Input;
  wire [7:0] wire_9;
  wire [15:0] wire_10;
  assign EndX = wire_10;
  wire [7:0] wire_11;
  wire [15:0] wire_12;
  assign EndY = wire_12;

endmodule
