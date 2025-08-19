module _4bgte9 (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Output;

  TC_Splitter8 # (.UUID(64'd3345404353361405943 ^ UUID)) Splitter8_0 (.in(wire_2), .out0(wire_5), .out1(wire_1), .out2(wire_4), .out3(wire_0), .out4(), .out5(), .out6(), .out7());
  mand # (.UUID(64'd2483749789121382907 ^ UUID)) mand_1 (.clk(clk), .rst(rst), .Input_1(wire_6), .Input_2(wire_0), .Output(wire_3));
  TC_Switch # (.UUID(64'd2388806708004556134 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_2 (.en(wire_3), .in(wire_3), .out(Output));
  TC_Or3 # (.UUID(64'd4058612993698156123 ^ UUID), .BIT_WIDTH(64'd1)) Or3_3 (.in0(wire_5), .in1(wire_1), .in2(wire_4), .out(wire_6));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [7:0] wire_2;
  assign wire_2 = Input;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;

endmodule
