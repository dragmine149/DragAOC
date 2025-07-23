module lte (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input_1;
  input  wire [31:0] Input_2;
  output  wire [0:0] Output;

  TC_Or # (.UUID(64'd927868631695770935 ^ UUID), .BIT_WIDTH(64'd1)) Or_0 (.in0(wire_2), .in1(wire_3), .out(wire_4));
  TC_Constant # (.UUID(64'd587738403212964597 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_LessI # (.UUID(64'd1696994369820757886 ^ UUID), .BIT_WIDTH(64'd32)) LessI32_2 (.in0(wire_1), .in1(wire_0), .out(wire_2));
  TC_Equal # (.UUID(64'd837223325833467200 ^ UUID), .BIT_WIDTH(64'd32)) Equal32_3 (.in0(wire_1), .in1(wire_0), .out(wire_3));

  wire [31:0] wire_0;
  assign wire_0 = Input_2;
  wire [31:0] wire_1;
  assign wire_1 = Input_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  assign Output = wire_4;

endmodule
