module sub1Adder (clk, rst, Input_1, Input_2, Carry, Result);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_1;
  input  wire [0:0] Input_2;
  output  wire [0:0] Carry;
  output  wire [0:0] Result;

  TC_Nor # (.UUID(64'd3061192093571773255 ^ UUID), .BIT_WIDTH(64'd1)) Nor_0 (.in0(wire_0), .in1(wire_1), .out(wire_4));
  TC_And # (.UUID(64'd1457820170052812187 ^ UUID), .BIT_WIDTH(64'd1)) And_1 (.in0(wire_0), .in1(wire_1), .out(wire_2));
  TC_Or # (.UUID(64'd2726445722458280910 ^ UUID), .BIT_WIDTH(64'd1)) Or_2 (.in0(wire_0), .in1(wire_1), .out(wire_5));
  TC_Or # (.UUID(64'd3571827508339527164 ^ UUID), .BIT_WIDTH(64'd1)) Or_3 (.in0(wire_4), .in1(wire_2), .out(wire_3));

  wire [0:0] wire_0;
  assign wire_0 = Input_2;
  wire [0:0] wire_1;
  assign wire_1 = Input_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  assign Result = wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  assign Carry = wire_5;

endmodule
