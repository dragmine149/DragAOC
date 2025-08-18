module NotTick (clk, rst, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  output  wire [0:0] Output;

  TC_Constant # (.UUID(64'd948666581252012769 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_0 (.out(wire_0));
  TC_Not # (.UUID(64'd12262805901688295 ^ UUID), .BIT_WIDTH(64'd1)) Not_1 (.in(wire_2), .out(wire_1));
  TC_DelayLine # (.UUID(64'd669671630002368113 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_2 (.clk(clk), .rst(rst), .in(wire_0), .out(wire_2));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  assign Output = wire_1;
  wire [0:0] wire_2;

endmodule
