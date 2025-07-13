module Div3z_roundz_minusz_2 (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [63:0] Output;

  TC_Mul # (.UUID(64'd3364514865364429339 ^ UUID), .BIT_WIDTH(64'd64)) DivMod64_0 (.in0(wire_2), .in1(wire_5), .out0(wire_0), .out1());
  TC_Constant # (.UUID(64'd4445812604238147585 ^ UUID), .BIT_WIDTH(64'd64), .value(64'h3)) Constant64_1 (.out(wire_5));
  TC_Add # (.UUID(64'd4555725251490577679 ^ UUID), .BIT_WIDTH(64'd64)) Add64_2 (.in0(wire_0), .in1(wire_7), .ci(1'd0), .out(wire_1), .co());
  TC_Neg # (.UUID(64'd1183156964557316822 ^ UUID), .BIT_WIDTH(64'd64)) Neg64_3 (.in(wire_4), .out(wire_7));
  TC_Constant # (.UUID(64'd4390973507027762957 ^ UUID), .BIT_WIDTH(64'd64), .value(64'h2)) Constant64_4 (.out(wire_4));
  TC_Switch # (.UUID(64'd3295356273189743515 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_5 (.en(wire_6), .in(wire_1), .out(Output));
  TC_Constant # (.UUID(64'd1681789967463892744 ^ UUID), .BIT_WIDTH(64'd64), .value(64'hFFFFFFFFFFFFFFFE)) Constant64_6 (.out(wire_3));
  TC_LessU # (.UUID(64'd250401650295717136 ^ UUID), .BIT_WIDTH(64'd64)) LessU64_7 (.in0(wire_1), .in1(wire_3), .out(wire_6));

  wire [63:0] wire_0;
  wire [63:0] wire_1;
  wire [63:0] wire_2;
  assign wire_2 = Input;
  wire [63:0] wire_3;
  wire [63:0] wire_4;
  wire [63:0] wire_5;
  wire [0:0] wire_6;
  wire [63:0] wire_7;

endmodule
