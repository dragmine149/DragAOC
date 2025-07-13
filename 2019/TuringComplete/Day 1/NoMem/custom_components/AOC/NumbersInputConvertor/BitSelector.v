module BitSelector (clk, rst, \8b_limit , \8b_Location , Number_1, Output, Number_2);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] \8b_limit ;
  input  wire [7:0] \8b_Location ;
  input  wire [7:0] Number_1;
  output  wire [7:0] Output;
  output  wire [7:0] Number_2;

  TC_Equal # (.UUID(64'd1928908807570332147 ^ UUID), .BIT_WIDTH(64'd8)) Equal8_0 (.in0(wire_0), .in1(wire_4), .out(wire_1));
  TC_Mux # (.UUID(64'd216556027767144756 ^ UUID), .BIT_WIDTH(64'd8)) Mux8_1 (.sel(wire_1), .in0(wire_4), .in1(wire_3), .out(wire_7));
  TC_Constant # (.UUID(64'd3830447490025826297 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_2 (.out());
  TC_Mux # (.UUID(64'd259243998919175111 ^ UUID), .BIT_WIDTH(64'd8)) Mux8_3 (.sel(wire_1), .in0(wire_5), .in1(wire_6), .out(wire_2));
  TC_Constant # (.UUID(64'd2142133555314350140 ^ UUID), .BIT_WIDTH(64'd8), .value(8'h10)) Constant8_4 (.out(wire_5));
  _8bz_Subz_1 # (.UUID(64'd4514657180930973450 ^ UUID)) _8bz_Subz_1_5 (.clk(clk), .rst(rst), .Input(wire_4), .Output(wire_3));

  wire [7:0] wire_0;
  assign wire_0 = \8b_Location ;
  wire [0:0] wire_1;
  wire [7:0] wire_2;
  assign Number_2 = wire_2;
  wire [7:0] wire_3;
  wire [7:0] wire_4;
  assign wire_4 = \8b_limit ;
  wire [7:0] wire_5;
  wire [7:0] wire_6;
  assign wire_6 = Number_1;
  wire [7:0] wire_7;
  assign Output = wire_7;

endmodule
