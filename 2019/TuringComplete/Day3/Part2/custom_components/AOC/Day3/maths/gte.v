module gte (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input_1;
  input  wire [31:0] Input_2;
  output  wire [0:0] Output;

  TC_Or # (.UUID(64'd4114019101269620761 ^ UUID), .BIT_WIDTH(64'd1)) Or_0 (.in0(wire_5), .in1(wire_2), .out(wire_4));
  TC_Constant # (.UUID(64'd3233625287895366684 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_Not # (.UUID(64'd4487513249047435982 ^ UUID), .BIT_WIDTH(64'd1)) Not_2 (.in(wire_3), .out(wire_5));
  TC_LessI # (.UUID(64'd2197566133528703478 ^ UUID), .BIT_WIDTH(64'd32)) LessI32_3 (.in0(wire_0), .in1(wire_1), .out(wire_3));
  TC_Equal # (.UUID(64'd3996580428223945404 ^ UUID), .BIT_WIDTH(64'd32)) Equal32_4 (.in0(wire_0), .in1(wire_1), .out(wire_2));

  wire [31:0] wire_0;
  assign wire_0 = Input_1;
  wire [31:0] wire_1;
  assign wire_1 = Input_2;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  assign Output = wire_4;
  wire [0:0] wire_5;

endmodule
