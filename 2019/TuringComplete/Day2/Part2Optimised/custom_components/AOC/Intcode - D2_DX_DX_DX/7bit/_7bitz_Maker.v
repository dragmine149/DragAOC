module _7bitz_Maker (clk, rst, \1 , \2 , \4 , \8 , \16 , \32 , \64 , Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] \1 ;
  input  wire [0:0] \2 ;
  input  wire [0:0] \4 ;
  input  wire [0:0] \8 ;
  input  wire [0:0] \16 ;
  input  wire [0:0] \32 ;
  input  wire [0:0] \64 ;
  output  wire [7:0] Output;

  TC_Constant # (.UUID(64'd2020436049418464200 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_0 (.out());
  TC_Constant # (.UUID(64'd886362538849093353 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_Constant # (.UUID(64'd1488492395926015785 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_2 (.out());
  TC_Constant # (.UUID(64'd1599038980075227216 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_3 (.out());
  TC_Constant # (.UUID(64'd1585948129761570664 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());
  TC_Constant # (.UUID(64'd3012531648485352829 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_5 (.out());
  TC_Constant # (.UUID(64'd2058179512588038201 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_6 (.out());
  TC_Maker8 # (.UUID(64'd4232150586890468771 ^ UUID)) Maker8_7 (.in0(wire_1), .in1(wire_4), .in2(wire_5), .in3(wire_0), .in4(wire_7), .in5(wire_2), .in6(wire_6), .in7(1'd0), .out(wire_3));

  wire [0:0] wire_0;
  assign wire_0 = \8 ;
  wire [0:0] wire_1;
  assign wire_1 = \1 ;
  wire [0:0] wire_2;
  assign wire_2 = \32 ;
  wire [7:0] wire_3;
  assign Output = wire_3;
  wire [0:0] wire_4;
  assign wire_4 = \2 ;
  wire [0:0] wire_5;
  assign wire_5 = \4 ;
  wire [0:0] wire_6;
  assign wire_6 = \64 ;
  wire [0:0] wire_7;
  assign wire_7 = \16 ;

endmodule
