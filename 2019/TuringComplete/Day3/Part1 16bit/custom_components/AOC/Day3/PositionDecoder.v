module PositionDecoder (clk, rst, Position, Y, X);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Position;
  output  wire [15:0] Y;
  output  wire [15:0] X;

  TC_Splitter32 # (.UUID(64'd1522049001248278139 ^ UUID)) Splitter32_0 (.in(wire_2), .out0(wire_0), .out1(wire_6), .out2(wire_1), .out3(wire_5));
  TC_Maker16 # (.UUID(64'd251440658160595858 ^ UUID)) Maker16_1 (.in0(wire_0), .in1(wire_6), .out(wire_3));
  TC_Maker16 # (.UUID(64'd1497996566624703903 ^ UUID)) Maker16_2 (.in0(wire_1), .in1(wire_5), .out(wire_4));

  wire [7:0] wire_0;
  wire [7:0] wire_1;
  wire [31:0] wire_2;
  assign wire_2 = Position;
  wire [15:0] wire_3;
  assign X = wire_3;
  wire [15:0] wire_4;
  assign Y = wire_4;
  wire [7:0] wire_5;
  wire [7:0] wire_6;

endmodule
