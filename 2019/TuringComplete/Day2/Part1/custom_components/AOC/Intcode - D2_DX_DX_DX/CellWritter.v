module CellWritter (clk, rst, Clear, Write, Value_1, Value_2, Written);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Clear;
  input  wire [0:0] Write;
  input  wire [63:0] Value_1;
  output  wire [63:0] Value_2;
  output  wire [0:0] Written;

  TC_DelayLine # (.UUID(64'd3000930704486083361 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_9), .out(wire_10));
  TC_DelayLine # (.UUID(64'd991382097674406265 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_1 (.clk(clk), .rst(rst), .in(wire_2), .out(wire_0));
  TC_Switch # (.UUID(64'd1909397842002500149 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_2 (.en(wire_3), .in(wire_0), .out(wire_2_0));
  TC_Switch # (.UUID(64'd1312555040640684393 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_3 (.en(wire_3), .in(wire_10), .out(wire_9_0));
  TC_Switch # (.UUID(64'd1652953684336650460 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_4 (.en(wire_5), .in(wire_7), .out(wire_9_1));
  TC_Not # (.UUID(64'd3712875361083703769 ^ UUID), .BIT_WIDTH(64'd1)) Not_5 (.in(wire_0), .out(wire_8));
  TC_And # (.UUID(64'd1602421929700327470 ^ UUID), .BIT_WIDTH(64'd1)) And_6 (.in0(wire_0), .in1(wire_1), .out(wire_3));
  TC_And # (.UUID(64'd2191405922096561884 ^ UUID), .BIT_WIDTH(64'd1)) And_7 (.in0(wire_8), .in1(wire_6), .out(wire_5));
  TC_Constant # (.UUID(64'd1597832759848664739 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_8 (.out());
  OnOrOff # (.UUID(64'd1624498145779995160 ^ UUID)) OnOrOff_9 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_2_1));
  TC_Not # (.UUID(64'd3168164095129288740 ^ UUID), .BIT_WIDTH(64'd1)) Not_10 (.in(wire_4), .out(wire_1));

  wire [0:0] wire_0;
  assign Written = wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_2_0;
  wire [0:0] wire_2_1;
  assign wire_2 = wire_2_0|wire_2_1;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  assign wire_4 = Clear;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  assign wire_6 = Write;
  wire [63:0] wire_7;
  assign wire_7 = Value_1;
  wire [0:0] wire_8;
  wire [63:0] wire_9;
  wire [63:0] wire_9_0;
  wire [63:0] wire_9_1;
  assign wire_9 = wire_9_0|wire_9_1;
  wire [63:0] wire_10;
  assign Value_2 = wire_10;

endmodule
