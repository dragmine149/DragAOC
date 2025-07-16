module _8zmany (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Output;

  TC_Splitter8 # (.UUID(64'd1243277835024812945 ^ UUID)) Splitter8_0 (.in(wire_4), .out0(wire_2), .out1(wire_8), .out2(wire_0), .out3(wire_6), .out4(wire_7), .out5(wire_5), .out6(wire_3), .out7(wire_9));
  OnOrOff # (.UUID(64'd3423688727643023586 ^ UUID)) OnOrOff_1 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_1_7));
  OnOrOff # (.UUID(64'd1937983015852000193 ^ UUID)) OnOrOff_2 (.clk(clk), .rst(rst), .Input(wire_8), .Output(wire_1_6));
  OnOrOff # (.UUID(64'd3744411019721477439 ^ UUID)) OnOrOff_3 (.clk(clk), .rst(rst), .Input(wire_0), .Output(wire_1_5));
  OnOrOff # (.UUID(64'd3428370106507161970 ^ UUID)) OnOrOff_4 (.clk(clk), .rst(rst), .Input(wire_6), .Output(wire_1_3));
  OnOrOff # (.UUID(64'd2110332955210665159 ^ UUID)) OnOrOff_5 (.clk(clk), .rst(rst), .Input(wire_7), .Output(wire_1_0));
  OnOrOff # (.UUID(64'd3628752146624637124 ^ UUID)) OnOrOff_6 (.clk(clk), .rst(rst), .Input(wire_3), .Output(wire_1_2));
  OnOrOff # (.UUID(64'd1235995497063349802 ^ UUID)) OnOrOff_7 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_1_1));
  OnOrOff # (.UUID(64'd3222899543984086281 ^ UUID)) OnOrOff_8 (.clk(clk), .rst(rst), .Input(wire_9), .Output(wire_1_4));
  TC_Switch # (.UUID(64'd1755771463398342474 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_9 (.en(wire_1), .in(wire_1), .out(Output));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_1_0;
  wire [0:0] wire_1_1;
  wire [0:0] wire_1_2;
  wire [0:0] wire_1_3;
  wire [0:0] wire_1_4;
  wire [0:0] wire_1_5;
  wire [0:0] wire_1_6;
  wire [0:0] wire_1_7;
  assign wire_1 = wire_1_0|wire_1_1|wire_1_2|wire_1_3|wire_1_4|wire_1_5|wire_1_6|wire_1_7;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [7:0] wire_4;
  assign wire_4 = Input;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;

endmodule
