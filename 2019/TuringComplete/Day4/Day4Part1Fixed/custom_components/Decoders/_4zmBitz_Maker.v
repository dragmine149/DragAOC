module _4zmBitz_Maker (clk, rst, \1 , \2 , \4 , \8 , Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] \1 ;
  input  wire [0:0] \2 ;
  input  wire [0:0] \4 ;
  input  wire [0:0] \8 ;
  output  wire [7:0] Output;

  TC_Maker8 # (.UUID(64'd1053918444515159786 ^ UUID)) Maker8_0 (.in0(wire_1), .in1(wire_0), .in2(wire_4), .in3(wire_2), .in4(1'd0), .in5(1'd0), .in6(1'd0), .in7(1'd0), .out(wire_3));
  TC_Constant # (.UUID(64'd3557036266461298560 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_Constant # (.UUID(64'd549283519447311783 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_2 (.out());
  TC_Constant # (.UUID(64'd107947252467539846 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Constant # (.UUID(64'd3986086581078435040 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());

  wire [0:0] wire_0;
  assign wire_0 = \2 ;
  wire [0:0] wire_1;
  assign wire_1 = \1 ;
  wire [0:0] wire_2;
  assign wire_2 = \8 ;
  wire [7:0] wire_3;
  assign Output = wire_3;
  wire [0:0] wire_4;
  assign wire_4 = \4 ;

endmodule
