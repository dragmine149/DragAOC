module outputswitch (clk, rst, Input_A, Input_B, Path, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_A;
  input  wire [0:0] Input_B;
  input  wire [0:0] Path;
  output  wire [0:0] Output;

  TC_Switch # (.UUID(64'd2585914349790343170 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_0 (.en(wire_3), .in(wire_2), .out(wire_0_0));
  TC_Switch # (.UUID(64'd1567588154185733440 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_1 (.en(wire_1), .in(wire_4), .out(wire_0_1));
  TC_Not # (.UUID(64'd3736288881574702847 ^ UUID), .BIT_WIDTH(64'd1)) Not_2 (.in(wire_1), .out(wire_3));

  wire [0:0] wire_0;
  wire [0:0] wire_0_0;
  wire [0:0] wire_0_1;
  assign wire_0 = wire_0_0|wire_0_1;
  assign Output = wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Path;
  wire [0:0] wire_2;
  assign wire_2 = Input_B;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  assign wire_4 = Input_A;

endmodule
