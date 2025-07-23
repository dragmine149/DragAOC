module _8bgt10 (clk, rst, Input, Greater_than_10);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Greater_than_10;

  TC_Splitter8 # (.UUID(64'd1551454573336648090 ^ UUID)) Splitter8_0 (.in(wire_6), .out0(), .out1(wire_5), .out2(wire_3), .out3(wire_1), .out4(wire_8), .out5(wire_10), .out6(wire_7), .out7(wire_11));
  OnOrOff # (.UUID(64'd2186480178979593258 ^ UUID)) OnOrOff_1 (.clk(clk), .rst(rst), .Input(wire_11), .Output(wire_0_3));
  OnOrOff # (.UUID(64'd757261234213535868 ^ UUID)) OnOrOff_2 (.clk(clk), .rst(rst), .Input(wire_7), .Output(wire_0_2));
  OnOrOff # (.UUID(64'd4030702410440659180 ^ UUID)) OnOrOff_3 (.clk(clk), .rst(rst), .Input(wire_10), .Output(wire_0_0));
  OnOrOff # (.UUID(64'd550260247061350175 ^ UUID)) OnOrOff_4 (.clk(clk), .rst(rst), .Input(wire_8), .Output(wire_0_1));
  mOR # (.UUID(64'd4151498759667399739 ^ UUID)) mOR_5 (.clk(clk), .rst(rst), .Input_1(wire_5), .Input_2(wire_3), .Output(wire_2));
  mand # (.UUID(64'd259833266197962651 ^ UUID)) mand_6 (.clk(clk), .rst(rst), .Input_1(wire_2), .Input_2(wire_1), .Output(wire_9));
  mOR # (.UUID(64'd976148624690511550 ^ UUID)) mOR_7 (.clk(clk), .rst(rst), .Input_1(wire_9), .Input_2(wire_0), .Output(wire_4));

  wire [0:0] wire_0;
  wire [0:0] wire_0_0;
  wire [0:0] wire_0_1;
  wire [0:0] wire_0_2;
  wire [0:0] wire_0_3;
  assign wire_0 = wire_0_0|wire_0_1|wire_0_2|wire_0_3;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  assign Greater_than_10 = wire_4;
  wire [0:0] wire_5;
  wire [7:0] wire_6;
  assign wire_6 = Input;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;

endmodule
