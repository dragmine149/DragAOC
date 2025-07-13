module _64bz_BitToNumberss (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [63:0] Output;

  TC_Splitter64 # (.UUID(64'd3730616155640714938 ^ UUID)) Splitter64_0 (.in(wire_10), .out0(wire_5), .out1(wire_3), .out2(wire_16), .out3(wire_9), .out4(wire_12), .out5(wire_0), .out6(wire_2), .out7(wire_15));
  TC_Maker64 # (.UUID(64'd3839189991469748609 ^ UUID)) Maker64_1 (.in0(wire_1), .in1(wire_6), .in2(wire_4), .in3(wire_7), .in4(wire_14), .in5(wire_13), .in6(wire_17), .in7(wire_11), .out(wire_8));
  ByteToNumber # (.UUID(64'd2312381046858610047 ^ UUID)) ByteToNumber_2 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_1));
  ByteToNumber # (.UUID(64'd1390230231245424960 ^ UUID)) ByteToNumber_3 (.clk(clk), .rst(rst), .Input(wire_3), .Output(wire_6));
  ByteToNumber # (.UUID(64'd2776204485237388157 ^ UUID)) ByteToNumber_4 (.clk(clk), .rst(rst), .Input(wire_16), .Output(wire_4));
  ByteToNumber # (.UUID(64'd3235376117459829803 ^ UUID)) ByteToNumber_5 (.clk(clk), .rst(rst), .Input(wire_9), .Output(wire_7));
  ByteToNumber # (.UUID(64'd921121574632914700 ^ UUID)) ByteToNumber_6 (.clk(clk), .rst(rst), .Input(wire_12), .Output(wire_14));
  ByteToNumber # (.UUID(64'd545571154311746194 ^ UUID)) ByteToNumber_7 (.clk(clk), .rst(rst), .Input(wire_0), .Output(wire_13));
  ByteToNumber # (.UUID(64'd3890954476465871063 ^ UUID)) ByteToNumber_8 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_17));
  ByteToNumber # (.UUID(64'd2863624730808253877 ^ UUID)) ByteToNumber_9 (.clk(clk), .rst(rst), .Input(wire_15), .Output(wire_11));

  wire [7:0] wire_0;
  wire [7:0] wire_1;
  wire [7:0] wire_2;
  wire [7:0] wire_3;
  wire [7:0] wire_4;
  wire [7:0] wire_5;
  wire [7:0] wire_6;
  wire [7:0] wire_7;
  wire [63:0] wire_8;
  assign Output = wire_8;
  wire [7:0] wire_9;
  wire [63:0] wire_10;
  assign wire_10 = Input;
  wire [7:0] wire_11;
  wire [7:0] wire_12;
  wire [7:0] wire_13;
  wire [7:0] wire_14;
  wire [7:0] wire_15;
  wire [7:0] wire_16;
  wire [7:0] wire_17;

endmodule
