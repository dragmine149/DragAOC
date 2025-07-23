module ByteToNumbersz_2 (clk, rst, Input, Has_48, Number);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] Has_48;
  output  wire [7:0] Number;

  TC_Maker8 # (.UUID(64'd4109316051975259632 ^ UUID)) Maker8_0 (.in0(wire_5), .in1(wire_4), .in2(wire_6), .in3(wire_7), .in4(1'd0), .in5(1'd0), .in6(1'd0), .in7(1'd0), .out(wire_3));
  TC_IndexerBit # (.UUID(64'd1379976302442914617 ^ UUID), .INDEX(64'd0)) IndexerBit_1 (.in({{56{1'b0}}, wire_0 }), .out(wire_5));
  TC_IndexerBit # (.UUID(64'd1545243827750713238 ^ UUID), .INDEX(64'd1)) IndexerBit_2 (.in({{56{1'b0}}, wire_0 }), .out(wire_4));
  TC_IndexerBit # (.UUID(64'd890010373396261433 ^ UUID), .INDEX(64'd2)) IndexerBit_3 (.in({{56{1'b0}}, wire_0 }), .out(wire_6));
  TC_IndexerBit # (.UUID(64'd2881320377257839558 ^ UUID), .INDEX(64'd3)) IndexerBit_4 (.in({{56{1'b0}}, wire_0 }), .out(wire_7));
  TC_IndexerBit # (.UUID(64'd3490602699590728428 ^ UUID), .INDEX(64'd4)) IndexerBit_5 (.in({{56{1'b0}}, wire_0 }), .out(wire_1));
  TC_IndexerBit # (.UUID(64'd4322953864298577160 ^ UUID), .INDEX(64'd5)) IndexerBit_6 (.in({{56{1'b0}}, wire_0 }), .out(wire_11));
  TC_Mux # (.UUID(64'd2304506826537427964 ^ UUID), .BIT_WIDTH(64'd8)) Mux8_7 (.sel(wire_2), .in0(wire_0), .in1(wire_3), .out(wire_8));
  TC_Constant # (.UUID(64'd3595891673203823230 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_8 (.out());
  OnOrOff # (.UUID(64'd1026483469894239498 ^ UUID)) OnOrOff_9 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_9));
  OnOrOff # (.UUID(64'd2391064100896299597 ^ UUID)) OnOrOff_10 (.clk(clk), .rst(rst), .Input(wire_11), .Output(wire_10));
  mand # (.UUID(64'd1904663082055626372 ^ UUID)) mand_11 (.clk(clk), .rst(rst), .Input_1(wire_9), .Input_2(wire_10), .Output(wire_2));

  wire [7:0] wire_0;
  assign wire_0 = Input;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  assign Has_48 = wire_2;
  wire [7:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  wire [7:0] wire_8;
  assign Number = wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;

endmodule
