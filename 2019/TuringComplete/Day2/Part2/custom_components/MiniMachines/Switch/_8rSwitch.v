module _8rSwitch (clk, rst, Enable, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Enable;
  input  wire [7:0] Input;
  output  wire [7:0] Output;

  TC_Switch # (.UUID(64'd2986484189407684533 ^ UUID), .BIT_WIDTH(64'd8)) Output8z_0 (.en(wire_0), .in(wire_1), .out(Output));
  TC_Constant # (.UUID(64'd4531143012953195476 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Enable;
  wire [7:0] wire_1;
  assign wire_1 = Input;

endmodule
