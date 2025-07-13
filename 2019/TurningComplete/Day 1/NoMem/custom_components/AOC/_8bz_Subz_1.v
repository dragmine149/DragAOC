module _8bz_Subz_1 (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [7:0] Output;

  sub1Adderx3 # (.UUID(64'd1059283143975876573 ^ UUID)) sub1Adderx3_0 (.clk(clk), .rst(rst), .Input_B(wire_5), .Input_C(wire_7), .Input_A(wire_12), .Carry_1(wire_8), .Output_A(wire_0), .Output_B(wire_13), .Carry_2(wire_16), .Output_C(wire_2));
  TC_Splitter8 # (.UUID(64'd402915242123522768 ^ UUID)) Splitter8_1 (.in(wire_18), .out0(wire_8), .out1(wire_12), .out2(wire_5), .out3(wire_7), .out4(wire_4), .out5(wire_14), .out6(wire_11), .out7(wire_19));
  sub1Adderx3 # (.UUID(64'd840749279894255141 ^ UUID)) sub1Adderx3_2 (.clk(clk), .rst(rst), .Input_B(wire_14), .Input_C(wire_11), .Input_A(wire_4), .Carry_1(wire_16), .Output_A(wire_1), .Output_B(wire_17), .Carry_2(wire_15), .Output_C(wire_3));
  sub1AdderNC # (.UUID(64'd2406416215414019835 ^ UUID)) sub1AdderNC_3 (.clk(clk), .rst(rst), .Input_1(wire_15), .Input_2(wire_19), .Result(wire_9));
  TC_Maker8 # (.UUID(64'd2124862871536536717 ^ UUID)) Maker8_4 (.in0(wire_6), .in1(wire_0), .in2(wire_13), .in3(wire_2), .in4(wire_1), .in5(wire_17), .in6(wire_3), .in7(wire_9), .out(wire_10));
  TC_Not # (.UUID(64'd145676194656638781 ^ UUID), .BIT_WIDTH(64'd1)) Not_5 (.in(wire_8), .out(wire_6));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [7:0] wire_10;
  assign Output = wire_10;
  wire [0:0] wire_11;
  wire [0:0] wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_14;
  wire [0:0] wire_15;
  wire [0:0] wire_16;
  wire [0:0] wire_17;
  wire [7:0] wire_18;
  assign wire_18 = Input;
  wire [0:0] wire_19;

endmodule
