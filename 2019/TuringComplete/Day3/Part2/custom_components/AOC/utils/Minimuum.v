module Minimuum (clk, rst, Value, Smallest);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Value;
  output  wire [63:0] Smallest;

  TC_DelayLine # (.UUID(64'd2235521713990043004 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_2), .out(wire_3));
  TC_LessU # (.UUID(64'd3768511492904595920 ^ UUID), .BIT_WIDTH(64'd64)) LessU64_1 (.in0(wire_1), .in1(wire_3), .out(wire_6));
  TC_Mux # (.UUID(64'd4236311263091754572 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_2 (.sel(wire_5), .in0(wire_3), .in1(wire_1), .out(wire_9));
  TC_Switch # (.UUID(64'd1005022670477777888 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_3 (.en(wire_4), .in(wire_9), .out(wire_2_0));
  TC_Not # (.UUID(64'd2338706470023891677 ^ UUID), .BIT_WIDTH(64'd1)) Not_4 (.in(wire_7), .out(wire_10));
  is0z_64b # (.UUID(64'd3898569319122186368 ^ UUID)) is0z_64b_5 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_7));
  mand # (.UUID(64'd944790705625494668 ^ UUID)) mand_6 (.clk(clk), .rst(rst), .Input_1(wire_6), .Input_2(wire_10), .Output(wire_5));
  TC_Constant # (.UUID(64'd2352824382516843227 ^ UUID), .BIT_WIDTH(64'd64), .value(64'hFFFFFFFFFFFFFFFF)) Constant64_7 (.out(wire_0));
  TC_Switch # (.UUID(64'd3465678912251085097 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_8 (.en(wire_8), .in(wire_0), .out(wire_2_1));
  TC_DelayLine # (.UUID(64'd4351689730420159598 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_9 (.clk(clk), .rst(rst), .in(wire_0[0:0]), .out(wire_4));
  TC_Not # (.UUID(64'd4000343577926131322 ^ UUID), .BIT_WIDTH(64'd1)) Not_10 (.in(wire_4), .out(wire_8));

  wire [63:0] wire_0;
  wire [63:0] wire_1;
  assign wire_1 = Value;
  wire [63:0] wire_2;
  wire [63:0] wire_2_0;
  wire [63:0] wire_2_1;
  assign wire_2 = wire_2_0|wire_2_1;
  wire [63:0] wire_3;
  assign Smallest = wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [63:0] wire_9;
  wire [0:0] wire_10;

endmodule
