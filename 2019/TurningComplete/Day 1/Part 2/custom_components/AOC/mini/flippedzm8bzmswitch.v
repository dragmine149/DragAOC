module flippedzm8bzmswitch (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input_1;
  input  wire [0:0] Input_2;
  output  wire [7:0] Output;

  TC_Switch # (.UUID(64'd1299838339173229892 ^ UUID), .BIT_WIDTH(64'd8)) Output8z_0 (.en(wire_0), .in(wire_1), .out(Output));
  TC_Constant # (.UUID(64'd1488490622442079861 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Input_2;
  wire [7:0] wire_1;
  assign wire_1 = Input_1;

endmodule
