module sub1Adderx3 (clk, rst, Input_B, Input_C, Input_A, Carry_1, Output_A, Output_B, Carry_2, Output_C);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_B;
  input  wire [0:0] Input_C;
  input  wire [0:0] Input_A;
  input  wire [0:0] Carry_1;
  output  wire [0:0] Output_A;
  output  wire [0:0] Output_B;
  output  wire [0:0] Carry_2;
  output  wire [0:0] Output_C;

  sub1Adder # (.UUID(64'd564624551679378325 ^ UUID)) sub1Adder_0 (.clk(clk), .rst(rst), .Input_1(wire_1), .Input_2(wire_6), .Carry(wire_8), .Result(wire_5));
  sub1Adder # (.UUID(64'd859699103284353083 ^ UUID)) sub1Adder_1 (.clk(clk), .rst(rst), .Input_1(wire_8), .Input_2(wire_3), .Carry(wire_7), .Result(wire_4));
  sub1Adder # (.UUID(64'd2409956723943830871 ^ UUID)) sub1Adder_2 (.clk(clk), .rst(rst), .Input_1(wire_7), .Input_2(wire_9), .Carry(wire_2), .Result(wire_0));
  TC_Constant # (.UUID(64'd2914620580844619945 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Constant # (.UUID(64'd2525206167445207688 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());

  wire [0:0] wire_0;
  assign Output_C = wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Carry_1;
  wire [0:0] wire_2;
  assign Carry_2 = wire_2;
  wire [0:0] wire_3;
  assign wire_3 = Input_B;
  wire [0:0] wire_4;
  assign Output_B = wire_4;
  wire [0:0] wire_5;
  assign Output_A = wire_5;
  wire [0:0] wire_6;
  assign wire_6 = Input_A;
  wire [0:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  assign wire_9 = Input_C;

endmodule
