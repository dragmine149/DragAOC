module Opcodez_zmz_1z_2z_99 (clk, rst, Opcode, Add, divide, halt);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Opcode;
  output  wire [0:0] Add;
  output  wire [0:0] divide;
  output  wire [0:0] halt;

  TC_Decoder2 # (.UUID(64'd4445327903845491585 ^ UUID)) Decoder2_0 (.sel0(wire_4), .sel1(wire_15), .out0(), .out1(wire_12), .out2(wire_10), .out3());
  TC_IndexerBit # (.UUID(64'd3711962272416626810 ^ UUID), .INDEX(64'd0)) IndexerBit_1 (.in({{56{1'b0}}, wire_8 }), .out(wire_4));
  TC_IndexerBit # (.UUID(64'd970141289654429279 ^ UUID), .INDEX(64'd1)) IndexerBit_2 (.in({{56{1'b0}}, wire_8 }), .out(wire_15));
  TC_Splitter8 # (.UUID(64'd13811288096728698 ^ UUID)) Splitter8_3 (.in(wire_8), .out0(wire_6), .out1(wire_7), .out2(wire_9), .out3(wire_5), .out4(wire_1), .out5(wire_17), .out6(wire_0), .out7(wire_13));
  TC_Not # (.UUID(64'd2158872654838303260 ^ UUID), .BIT_WIDTH(64'd1)) Not_4 (.in(wire_13), .out(wire_16));
  TC_Not # (.UUID(64'd1430969372784429821 ^ UUID), .BIT_WIDTH(64'd1)) Not_5 (.in(wire_1), .out(wire_11));
  TC_Not # (.UUID(64'd1347383189665234652 ^ UUID), .BIT_WIDTH(64'd1)) Not_6 (.in(wire_5), .out(wire_3));
  TC_Not # (.UUID(64'd1578157164492903579 ^ UUID), .BIT_WIDTH(64'd1)) Not_7 (.in(wire_9), .out(wire_2));
  _8inAND # (.UUID(64'd1463377719115208567 ^ UUID)) _8inAND_8 (.clk(clk), .rst(rst), .Input_1(wire_6), .Input_2(wire_7), .Input_3(wire_2), .Input_4(wire_3), .Input_5(wire_11), .Input_6(wire_17), .Input_7(wire_0), .Input_8(wire_16), .Output(wire_14));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [7:0] wire_8;
  assign wire_8 = Opcode;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  assign divide = wire_10;
  wire [0:0] wire_11;
  wire [0:0] wire_12;
  assign Add = wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_14;
  assign halt = wire_14;
  wire [0:0] wire_15;
  wire [0:0] wire_16;
  wire [0:0] wire_17;

endmodule
