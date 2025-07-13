module ByteToNumber (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [7:0] Output;

  TC_Add # (.UUID(64'd68030724543918015 ^ UUID), .BIT_WIDTH(64'd8)) Add8_0 (.in0(wire_0), .in1(wire_1), .ci(1'd0), .out(wire_3), .co());
  TC_Neg # (.UUID(64'd4137500188886322957 ^ UUID), .BIT_WIDTH(64'd8)) Neg8_1 (.in(wire_2), .out(wire_1));
  TC_Constant # (.UUID(64'd351491625066457929 ^ UUID), .BIT_WIDTH(64'd8), .value(8'h30)) Constant8_2 (.out(wire_2));

  wire [7:0] wire_0;
  assign wire_0 = Input;
  wire [7:0] wire_1;
  wire [7:0] wire_2;
  wire [7:0] wire_3;
  assign Output = wire_3;

endmodule
