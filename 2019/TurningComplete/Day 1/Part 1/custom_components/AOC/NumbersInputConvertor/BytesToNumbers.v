module BytesToNumbers (clk, rst, Carry, \Input_(64b) , Offset, \Output_(8b) );
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Carry;
  input  wire [63:0] \Input_(64b) ;
  output  wire [7:0] Offset;
  output  wire [63:0] \Output_(8b) ;

  TC_Splitter64 # (.UUID(64'd2626060591295325577 ^ UUID)) Splitter64_0 (.in(wire_19), .out0(wire_13), .out1(wire_20), .out2(wire_11), .out3(wire_7), .out4(wire_8), .out5(wire_6), .out6(wire_0), .out7(wire_5));
  x10zpn # (.UUID(64'd779852039867394912 ^ UUID)) x10zpn_1 (.clk(clk), .rst(rst), .Full(wire_12), .Input(wire_20), .Output(wire_16));
  x10zpn # (.UUID(64'd2873505357101391258 ^ UUID)) x10zpn_2 (.clk(clk), .rst(rst), .Full(wire_16), .Input(wire_11), .Output(wire_10));
  x10zpn # (.UUID(64'd1946565912552497839 ^ UUID)) x10zpn_3 (.clk(clk), .rst(rst), .Full(wire_10), .Input(wire_7), .Output(wire_18));
  x10zpn # (.UUID(64'd3530602618669448078 ^ UUID)) x10zpn_4 (.clk(clk), .rst(rst), .Full(wire_18), .Input(wire_8), .Output(wire_4));
  x10zpn # (.UUID(64'd3469674152925285329 ^ UUID)) x10zpn_5 (.clk(clk), .rst(rst), .Full(wire_4), .Input(wire_6), .Output(wire_14));
  x10zpn # (.UUID(64'd3178896029310116842 ^ UUID)) x10zpn_6 (.clk(clk), .rst(rst), .Full(wire_14), .Input(wire_0), .Output(wire_17));
  x10zpn # (.UUID(64'd4391845675030578639 ^ UUID)) x10zpn_7 (.clk(clk), .rst(rst), .Full(wire_17), .Input(wire_5), .Output(wire_15));
  x10zpn # (.UUID(64'd2322638491581700898 ^ UUID)) x10zpn_8 (.clk(clk), .rst(rst), .Full(wire_1), .Input(wire_13), .Output(wire_12));
  _8bBitSelector # (.UUID(64'd2603984123404637582 ^ UUID)) _8bBitSelector_9 (.clk(clk), .rst(rst), .Number_1(wire_2), .Position(wire_3), .Number_2(wire_19));
  _64bz_BitToNumberss # (.UUID(64'd285087343790373290 ^ UUID)) _64bz_BitToNumberss_10 (.clk(clk), .rst(rst), .Input(wire_9), .Output(wire_2));
  _8bNewLine # (.UUID(64'd3071233820244704475 ^ UUID)) _8bNewLine_11 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_3));

  wire [7:0] wire_0;
  wire [63:0] wire_1;
  assign wire_1 = Carry;
  wire [63:0] wire_2;
  wire [7:0] wire_3;
  assign Offset = wire_3;
  wire [63:0] wire_4;
  wire [7:0] wire_5;
  wire [7:0] wire_6;
  wire [7:0] wire_7;
  wire [7:0] wire_8;
  wire [63:0] wire_9;
  assign wire_9 = \Input_(64b) ;
  wire [63:0] wire_10;
  wire [7:0] wire_11;
  wire [63:0] wire_12;
  wire [7:0] wire_13;
  wire [63:0] wire_14;
  wire [63:0] wire_15;
  assign \Output_(8b)  = wire_15;
  wire [63:0] wire_16;
  wire [63:0] wire_17;
  wire [63:0] wire_18;
  wire [63:0] wire_19;
  wire [7:0] wire_20;

endmodule
