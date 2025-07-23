module InverseAbs32b (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input;
  output  wire [31:0] Output;

  TC_Mux # (.UUID(64'd4405814751070039739 ^ UUID), .BIT_WIDTH(64'd32)) Mux32_0 (.sel(wire_4), .in0(wire_3), .in1(wire_2), .out(wire_1));
  TC_IndexerBit # (.UUID(64'd3145290658172974474 ^ UUID), .INDEX(64'd31)) IndexerBit_1 (.in({{32{1'b0}}, wire_3 }), .out(wire_0));
  TC_Neg # (.UUID(64'd2661211449490451668 ^ UUID), .BIT_WIDTH(64'd32)) Neg32_2 (.in(wire_3), .out(wire_2));
  TC_Not # (.UUID(64'd1182259451306726580 ^ UUID), .BIT_WIDTH(64'd1)) Not_3 (.in(wire_0), .out(wire_4));

  wire [0:0] wire_0;
  wire [31:0] wire_1;
  assign Output = wire_1;
  wire [31:0] wire_2;
  wire [31:0] wire_3;
  assign wire_3 = Input;
  wire [0:0] wire_4;

endmodule
