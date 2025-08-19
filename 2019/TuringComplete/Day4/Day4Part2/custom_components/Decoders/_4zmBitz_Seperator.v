module _4zmBitz_Seperator (clk, rst, Input, Output_1, Output_2, Output_3, Output_4);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Output_1;
  output  wire [0:0] Output_2;
  output  wire [0:0] Output_3;
  output  wire [0:0] Output_4;

  TC_Splitter8 # (.UUID(64'd2698854735380175248 ^ UUID)) Splitter8_0 (.in(wire_4), .out0(wire_2), .out1(wire_1), .out2(wire_3), .out3(wire_0), .out4(), .out5(), .out6(), .out7());
  TC_Constant # (.UUID(64'd3219825596555428862 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_Constant # (.UUID(64'd235412243955052466 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_2 (.out());
  TC_Constant # (.UUID(64'd586764407081932253 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Constant # (.UUID(64'd3309561538501496115 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());

  wire [0:0] wire_0;
  assign Output_4 = wire_0;
  wire [0:0] wire_1;
  assign Output_2 = wire_1;
  wire [0:0] wire_2;
  assign Output_1 = wire_2;
  wire [0:0] wire_3;
  assign Output_3 = wire_3;
  wire [7:0] wire_4;
  assign wire_4 = Input;

endmodule
