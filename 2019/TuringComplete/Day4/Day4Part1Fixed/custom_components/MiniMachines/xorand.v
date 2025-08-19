module xorand (clk, rst, Input_A, Input_B, And, Result);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_A;
  input  wire [0:0] Input_B;
  output  wire [0:0] And;
  output  wire [0:0] Result;

  TC_And # (.UUID(64'd4156761643241992554 ^ UUID), .BIT_WIDTH(64'd1)) And_0 (.in0(wire_2), .in1(wire_1), .out(wire_0));
  TC_Nor # (.UUID(64'd2540358885624842490 ^ UUID), .BIT_WIDTH(64'd1)) Nor_1 (.in0(wire_4), .in1(wire_0), .out(wire_3));
  TC_Nor # (.UUID(64'd574784311387816563 ^ UUID), .BIT_WIDTH(64'd1)) Nor_2 (.in0(wire_2), .in1(wire_1), .out(wire_4));

  wire [0:0] wire_0;
  assign And = wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Input_B;
  wire [0:0] wire_2;
  assign wire_2 = Input_A;
  wire [0:0] wire_3;
  assign Result = wire_3;
  wire [0:0] wire_4;

endmodule
