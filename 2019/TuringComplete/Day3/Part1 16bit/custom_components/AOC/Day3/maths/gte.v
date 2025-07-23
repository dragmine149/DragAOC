module gte (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] Input_1;
  input  wire [15:0] Input_2;
  output  wire [0:0] Output;

  TC_Equal # (.UUID(64'd2937520456405132838 ^ UUID), .BIT_WIDTH(64'd16)) Equal16_0 (.in0(wire_2), .in1(wire_1), .out(wire_4));
  TC_Or # (.UUID(64'd4114019101269620761 ^ UUID), .BIT_WIDTH(64'd1)) Or_1 (.in0(wire_3), .in1(wire_4), .out(wire_5));
  TC_LessI # (.UUID(64'd3686294080427401137 ^ UUID), .BIT_WIDTH(64'd16)) LessI16_2 (.in0(wire_2), .in1(wire_1), .out(wire_0));
  TC_Constant # (.UUID(64'd3233625287895366684 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Not # (.UUID(64'd4487513249047435982 ^ UUID), .BIT_WIDTH(64'd1)) Not_4 (.in(wire_0), .out(wire_3));

  wire [0:0] wire_0;
  wire [15:0] wire_1;
  assign wire_1 = Input_1;
  wire [15:0] wire_2;
  assign wire_2 = Input_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  assign Output = wire_5;

endmodule
