module Cellz_2z_Dumb (clk, rst, Increment, Overwrite, Global_Overwrite, Output, Increment_Next);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Increment;
  input  wire [7:0] Overwrite;
  input  wire [0:0] Global_Overwrite;
  output  wire [7:0] Output;
  output  wire [0:0] Increment_Next;

  TC_Not # (.UUID(64'd4435160898015840219 ^ UUID), .BIT_WIDTH(64'd1)) Not_0 (.in(wire_1), .out(wire_5));
  TC_DelayLine # (.UUID(64'd174225934206509417 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_1 (.clk(clk), .rst(rst), .in(wire_2), .out(wire_11));
  TC_Switch # (.UUID(64'd1037403255418157240 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_2 (.en(wire_2), .in(wire_11), .out(wire_7));
  TC_Mux # (.UUID(64'd3971226479063252743 ^ UUID), .BIT_WIDTH(64'd8)) Mux8_3 (.sel(wire_4), .in0(wire_6), .in1(wire_8), .out(wire_12));
  TC_Constant # (.UUID(64'd4156158477184862849 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());
  TC_Constant # (.UUID(64'd3997744249560952975 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_5 (.out());
  _4Bit # (.UUID(64'd1183297596405385417 ^ UUID)) _4Bit_6 (.clk(clk), .rst(rst), .Overrite_Value(wire_3), .Overrite(wire_0), .Output(wire_6));
  _4bgte9 # (.UUID(64'd3746322222025125859 ^ UUID)) _4bgte9_7 (.clk(clk), .rst(rst), .Input(wire_6), .Output(wire_10));
  OnOrOff # (.UUID(64'd1024934874117766360 ^ UUID)) OnOrOff_8 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_0_1));
  OnOrOff # (.UUID(64'd2936696676063898150 ^ UUID)) OnOrOff_9 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_0_3));
  OnOrOff # (.UUID(64'd908875806458571666 ^ UUID)) OnOrOff_10 (.clk(clk), .rst(rst), .Input(wire_7), .Output(wire_0_2));
  OnOrOff # (.UUID(64'd3591582489492066469 ^ UUID)) OnOrOff_11 (.clk(clk), .rst(rst), .Input(wire_4), .Output(wire_0_0));
  mand # (.UUID(64'd4438192410060361779 ^ UUID)) mand_12 (.clk(clk), .rst(rst), .Input_1(wire_10), .Input_2(wire_1), .Output(wire_2));
  TC_Switch # (.UUID(64'd3435483617657219607 ^ UUID), .BIT_WIDTH(64'd8)) Switch8_13 (.en(wire_9), .in(wire_12), .out(wire_3));
  TC_Not # (.UUID(64'd1470356014395269173 ^ UUID), .BIT_WIDTH(64'd1)) Not_14 (.in(wire_2), .out(wire_9));

  wire [0:0] wire_0;
  wire [0:0] wire_0_0;
  wire [0:0] wire_0_1;
  wire [0:0] wire_0_2;
  wire [0:0] wire_0_3;
  assign wire_0 = wire_0_0|wire_0_1|wire_0_2|wire_0_3;
  wire [0:0] wire_1;
  assign wire_1 = Increment;
  wire [0:0] wire_2;
  assign Increment_Next = wire_2;
  wire [7:0] wire_3;
  wire [0:0] wire_4;
  assign wire_4 = Global_Overwrite;
  wire [0:0] wire_5;
  wire [7:0] wire_6;
  assign Output = wire_6;
  wire [0:0] wire_7;
  wire [7:0] wire_8;
  assign wire_8 = Overwrite;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;
  wire [7:0] wire_12;

endmodule
