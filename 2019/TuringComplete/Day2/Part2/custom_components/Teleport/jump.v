module jump (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input;
  output  wire [0:0] Output;

  TC_Constant # (.UUID(64'd1486493815960754372 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_0 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Input;
  assign Output = wire_0;

endmodule
