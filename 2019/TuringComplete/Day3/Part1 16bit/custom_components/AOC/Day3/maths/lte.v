module lte (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] Input_1;
  input  wire [15:0] Input_2;
  output  wire [0:0] Output;

  TC_Equal # (.UUID(64'd3282073931348948330 ^ UUID), .BIT_WIDTH(64'd16)) Equal16_0 (.in0(wire_1), .in1(wire_3), .out(wire_4));
  TC_Or # (.UUID(64'd927868631695770935 ^ UUID), .BIT_WIDTH(64'd1)) Or_1 (.in0(wire_0), .in1(wire_4), .out(wire_2));
  TC_LessI # (.UUID(64'd2013732172906029270 ^ UUID), .BIT_WIDTH(64'd16)) LessI16_2 (.in0(wire_1), .in1(wire_3), .out(wire_0));
  TC_Constant # (.UUID(64'd587738403212964597 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());

  wire [0:0] wire_0;
  wire [15:0] wire_1;
  assign wire_1 = Input_2;
  wire [0:0] wire_2;
  assign Output = wire_2;
  wire [15:0] wire_3;
  assign wire_3 = Input_1;
  wire [0:0] wire_4;

endmodule
