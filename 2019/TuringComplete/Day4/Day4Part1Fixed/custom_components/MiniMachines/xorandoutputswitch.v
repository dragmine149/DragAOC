module xorandoutputswitch (clk, rst, Bit, Carry, Overrite_bit, Overite, Overrite, And, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Bit;
  input  wire [0:0] Carry;
  input  wire [0:0] Overrite_bit;
  input  wire [0:0] Overite;
  output  wire [0:0] Overrite;
  output  wire [0:0] And;
  output  wire [0:0] Output;

  TC_Constant # (.UUID(64'd2218281053047597637 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_0 (.out());
  xorand # (.UUID(64'd3645253335263248284 ^ UUID)) xorand_1 (.clk(clk), .rst(rst), .Input_A(wire_1), .Input_B(wire_0), .And(wire_3), .Result(wire_6));
  outputswitch # (.UUID(64'd4114283144548525242 ^ UUID)) outputswitch_2 (.clk(clk), .rst(rst), .Input_A(wire_2), .Input_B(wire_6), .Path(wire_4), .Output(wire_5));

  wire [0:0] wire_0;
  assign wire_0 = Bit;
  wire [0:0] wire_1;
  assign wire_1 = Carry;
  wire [0:0] wire_2;
  assign wire_2 = Overrite_bit;
  wire [0:0] wire_3;
  assign And = wire_3;
  wire [0:0] wire_4;
  assign wire_4 = Overite;
  assign Overrite = wire_4;
  wire [0:0] wire_5;
  assign Output = wire_5;
  wire [0:0] wire_6;

endmodule
