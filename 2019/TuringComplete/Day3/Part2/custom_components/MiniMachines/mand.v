module mand (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_1;
  input  wire [0:0] Input_2;
  output  wire [0:0] Output;

  TC_And # (.UUID(64'd314412038531232858 ^ UUID), .BIT_WIDTH(64'd1)) And_0 (.in0(wire_2), .in1(wire_0), .out(wire_1));
  TC_Constant # (.UUID(64'd3685043632808384014 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Input_2;
  wire [0:0] wire_1;
  assign Output = wire_1;
  wire [0:0] wire_2;
  assign wire_2 = Input_1;

endmodule
