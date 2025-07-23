module ByteToDirection (clk, rst, Input, Bit_1, Bit_2);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Bit_1;
  output  wire [0:0] Bit_2;

  TC_Splitter8 # (.UUID(64'd891350813271121587 ^ UUID)) Splitter8_0 (.in(wire_10), .out0(wire_6), .out1(wire_1), .out2(wire_3), .out3(wire_5), .out4(wire_4), .out5(), .out6(wire_8), .out7());
  TC_Switch # (.UUID(64'd4298301945042490241 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_1 (.en(wire_8), .in(wire_0), .out(Bit_2));
  TC_Switch # (.UUID(64'd2653110159793525474 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_2 (.en(wire_8), .in(wire_13), .out(Bit_1));
  TC_And # (.UUID(64'd3279267446483505836 ^ UUID), .BIT_WIDTH(64'd1)) And_3 (.in0(wire_1), .in1(wire_4), .out(wire_11));
  TC_And # (.UUID(64'd1198623384024766799 ^ UUID), .BIT_WIDTH(64'd1)) And_4 (.in0(wire_3), .in1(wire_5), .out(wire_7));
  TC_Maker8 # (.UUID(64'd3470198726228039711 ^ UUID)) Maker8_5 (.in0(wire_13), .in1(wire_0), .in2(1'd0), .in3(1'd0), .in4(1'd0), .in5(1'd0), .in6(1'd0), .in7(1'd0), .out(wire_2));
  OnOrOff # (.UUID(64'd1890668696780596501 ^ UUID)) OnOrOff_6 (.clk(clk), .rst(rst), .Input(wire_11), .Output(wire_13_0));
  OnOrOff # (.UUID(64'd797250103079114079 ^ UUID)) OnOrOff_7 (.clk(clk), .rst(rst), .Input(wire_9), .Output(wire_0_1));
  OnOrOff # (.UUID(64'd2263812759793013937 ^ UUID)) OnOrOff_8 (.clk(clk), .rst(rst), .Input(wire_7), .Output(wire_0_0));
  OnOrOff # (.UUID(64'd3502149505064907893 ^ UUID)) OnOrOff_9 (.clk(clk), .rst(rst), .Input(wire_7), .Output(wire_13_1));
  TC_And3 # (.UUID(64'd2028616243251559789 ^ UUID), .BIT_WIDTH(64'd1)) And3_10 (.in0(wire_3), .in1(wire_12), .in2(wire_8), .out(wire_9));
  TC_Not # (.UUID(64'd3172528482556432240 ^ UUID), .BIT_WIDTH(64'd1)) Not_11 (.in(wire_6), .out(wire_12));

  wire [0:0] wire_0;
  wire [0:0] wire_0_0;
  wire [0:0] wire_0_1;
  assign wire_0 = wire_0_0|wire_0_1;
  wire [0:0] wire_1;
  wire [7:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [7:0] wire_10;
  assign wire_10 = Input;
  wire [0:0] wire_11;
  wire [0:0] wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_13_0;
  wire [0:0] wire_13_1;
  assign wire_13 = wire_13_0|wire_13_1;

endmodule
