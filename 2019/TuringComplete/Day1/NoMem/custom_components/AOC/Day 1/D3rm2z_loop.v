module D3rm2z_loop (clk, rst, Input, Result, In_Progress);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [63:0] Result;
  output  wire [0:0] In_Progress;

  TC_DelayLine # (.UUID(64'd1348438308015416136 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_1), .out(wire_2));
  TC_Switch # (.UUID(64'd2538000885193183181 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_1 (.en(wire_9), .in(wire_2), .out(wire_0_0));
  TC_Switch # (.UUID(64'd3109256804374953402 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_2 (.en(wire_10), .in(wire_6), .out(wire_0_1));
  TC_Not # (.UUID(64'd1087214899586684412 ^ UUID), .BIT_WIDTH(64'd1)) Not_3 (.in(wire_9), .out(wire_10));
  TC_Equal # (.UUID(64'd3833473589065923875 ^ UUID), .BIT_WIDTH(64'd64)) Equal64_4 (.in0(wire_1), .in1(64'd0), .out(wire_5));
  TC_DelayLine # (.UUID(64'd854934061391516982 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_5 (.clk(clk), .rst(rst), .in(wire_4), .out(wire_8));
  TC_Add # (.UUID(64'd4352149879627215481 ^ UUID), .BIT_WIDTH(64'd64)) Add64_6 (.in0(wire_8), .in1(wire_2), .ci(1'd0), .out(wire_3), .co());
  TC_Switch # (.UUID(64'd2686530109882044955 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_7 (.en(wire_5), .in(wire_4), .out(Result));
  TC_Not # (.UUID(64'd3574951122981285635 ^ UUID), .BIT_WIDTH(64'd1)) Not_8 (.in(wire_5), .out(wire_7));
  TC_Switch # (.UUID(64'd3682218997366916451 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_9 (.en(wire_7), .in(wire_7), .out(In_Progress));
  Div3z_roundz_minusz_2 # (.UUID(64'd659149299787266039 ^ UUID)) Div3z_roundz_minusz_2_10 (.clk(clk), .rst(rst), .Input(wire_0), .Output(wire_1));
  _64zmany # (.UUID(64'd443005857434215373 ^ UUID)) _64zmany_11 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_9));
  TC_Switch # (.UUID(64'd396893878796129036 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_12 (.en(wire_11), .in(wire_3), .out(wire_4));
  TC_DelayLine # (.UUID(64'd2606231712170727212 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_13 (.clk(clk), .rst(rst), .in(wire_7), .out(wire_11));

  wire [63:0] wire_0;
  wire [63:0] wire_0_0;
  wire [63:0] wire_0_1;
  assign wire_0 = wire_0_0|wire_0_1;
  wire [63:0] wire_1;
  wire [63:0] wire_2;
  wire [63:0] wire_3;
  wire [63:0] wire_4;
  wire [0:0] wire_5;
  wire [63:0] wire_6;
  assign wire_6 = Input;
  wire [0:0] wire_7;
  wire [63:0] wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;

endmodule
