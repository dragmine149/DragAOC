module mNOR (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input_1;
  input  wire [0:0] Input_2;
  output  wire [0:0] Output;

  TC_Nor # (.UUID(64'd2282639645650429114 ^ UUID), .BIT_WIDTH(64'd1)) Nor_0 (.in0(wire_0), .in1(wire_2), .out(wire_1));
  TC_Constant # (.UUID(64'd1283998494145581654 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Input_2;
  wire [0:0] wire_1;
  assign Output = wire_1;
  wire [0:0] wire_2;
  assign wire_2 = Input_1;

endmodule
