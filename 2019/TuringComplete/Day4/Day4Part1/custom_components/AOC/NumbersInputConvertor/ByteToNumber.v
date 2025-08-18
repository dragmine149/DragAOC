module ByteToNumber (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [7:0] Output;

  TC_Add # (.UUID(64'd68030724543918015 ^ UUID), .BIT_WIDTH(64'd8)) Add8_0 (.in0(wire_1), .in1(wire_2), .ci(1'd0), .out(wire_0), .co());
  TC_Constant # (.UUID(64'd351491625066457929 ^ UUID), .BIT_WIDTH(64'd8), .value(8'hD0)) Constant8_1 (.out(wire_2));

  wire [7:0] wire_0;
  assign Output = wire_0;
  wire [7:0] wire_1;
  assign wire_1 = Input;
  wire [7:0] wire_2;

endmodule
