module _7bitz_Seperator (clk, rst, Input, \1 , \2 , \4 , \8 , \16 , \32 , \64 );
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input;
  output  wire [0:0] \1 ;
  output  wire [0:0] \2 ;
  output  wire [0:0] \4 ;
  output  wire [0:0] \8 ;
  output  wire [0:0] \16 ;
  output  wire [0:0] \32 ;
  output  wire [0:0] \64 ;

  TC_IndexerBit # (.UUID(64'd2974204974193335979 ^ UUID), .INDEX(64'd6)) IndexerBit_0 (.in({{56{1'b0}}, wire_2 }), .out(wire_4));
  TC_IndexerBit # (.UUID(64'd2865656414123185754 ^ UUID), .INDEX(64'd5)) IndexerBit_1 (.in({{56{1'b0}}, wire_2 }), .out(wire_0));
  TC_IndexerBit # (.UUID(64'd4359303656987985866 ^ UUID), .INDEX(64'd4)) IndexerBit_2 (.in({{56{1'b0}}, wire_2 }), .out(wire_1));
  TC_IndexerBit # (.UUID(64'd3593600164975947489 ^ UUID), .INDEX(64'd3)) IndexerBit_3 (.in({{56{1'b0}}, wire_2 }), .out(wire_6));
  TC_IndexerBit # (.UUID(64'd1615066421531489145 ^ UUID), .INDEX(64'd2)) IndexerBit_4 (.in({{56{1'b0}}, wire_2 }), .out(wire_5));
  TC_IndexerBit # (.UUID(64'd3381468696908495724 ^ UUID), .INDEX(64'd1)) IndexerBit_5 (.in({{56{1'b0}}, wire_2 }), .out(wire_7));
  TC_IndexerBit # (.UUID(64'd1640146763647435835 ^ UUID), .INDEX(64'd0)) IndexerBit_6 (.in({{56{1'b0}}, wire_2 }), .out(wire_3));

  wire [0:0] wire_0;
  assign \32  = wire_0;
  wire [0:0] wire_1;
  assign \16  = wire_1;
  wire [7:0] wire_2;
  assign wire_2 = Input;
  wire [0:0] wire_3;
  assign \1  = wire_3;
  wire [0:0] wire_4;
  assign \64  = wire_4;
  wire [0:0] wire_5;
  assign \4  = wire_5;
  wire [0:0] wire_6;
  assign \8  = wire_6;
  wire [0:0] wire_7;
  assign \2  = wire_7;

endmodule
