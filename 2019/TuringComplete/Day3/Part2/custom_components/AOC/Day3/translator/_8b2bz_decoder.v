module _8b2bz_decoder (clk, rst, Input, Output_1, Output_2, Output_3, Output_4);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Output_1;
  output  wire [0:0] Output_2;
  output  wire [0:0] Output_3;
  output  wire [0:0] Output_4;

  TC_Splitter8 # (.UUID(64'd1170379082365314463 ^ UUID)) Splitter8_0 (.in(wire_6), .out0(wire_2), .out1(wire_5), .out2(), .out3(), .out4(), .out5(), .out6(), .out7());
  TC_Decoder2 # (.UUID(64'd1384626324551865882 ^ UUID)) Decoder2_1 (.sel0(wire_2), .sel1(wire_5), .out0(wire_3), .out1(wire_1), .out2(wire_0), .out3(wire_4));
  TC_Constant # (.UUID(64'd4276679834471134113 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_2 (.out());
  TC_Constant # (.UUID(64'd204466678597503611 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Constant # (.UUID(64'd4450236232813414221 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());

  wire [0:0] wire_0;
  assign Output_3 = wire_0;
  wire [0:0] wire_1;
  assign Output_2 = wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  assign Output_1 = wire_3;
  wire [0:0] wire_4;
  assign Output_4 = wire_4;
  wire [0:0] wire_5;
  wire [7:0] wire_6;
  assign wire_6 = Input;

endmodule
