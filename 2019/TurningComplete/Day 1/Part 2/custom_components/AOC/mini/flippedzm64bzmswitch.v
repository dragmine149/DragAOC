module flippedzm64bzmswitch (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input_1;
  input  wire [0:0] Input_2;
  output  wire [63:0] Output;

  TC_Switch # (.UUID(64'd2753496366035784750 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_0 (.en(wire_0), .in(wire_1), .out(Output));
  TC_Switch # (.UUID(64'd1636605595200992945 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_1 (.en(wire_0), .in(wire_2), .out(wire_1));

  wire [0:0] wire_0;
  assign wire_0 = Input_2;
  wire [63:0] wire_1;
  wire [63:0] wire_2;
  assign wire_2 = Input_1;

endmodule
