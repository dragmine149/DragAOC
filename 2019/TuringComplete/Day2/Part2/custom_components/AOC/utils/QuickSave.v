module QuickSave (clk, rst, Value, \Number_(stored) , \Saved?_1 , \Saved?_2 );
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Value;
  output  wire [63:0] \Number_(stored) ;
  output  wire [0:0] \Saved?_1 ;
  output  wire [0:0] \Saved?_2 ;

  TC_DelayLine # (.UUID(64'd1992170729783611433 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_3), .out(wire_2));
  TC_Switch # (.UUID(64'd2522336483601522915 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_1 (.en(wire_0), .in(wire_2), .out(wire_3_0));
  TC_Switch # (.UUID(64'd3111394576583745305 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_2 (.en(wire_4), .in(wire_1), .out(wire_3_1));
  TC_Not # (.UUID(64'd3881920702927896942 ^ UUID), .BIT_WIDTH(64'd1)) Not_3 (.in(wire_0), .out(wire_4));
  TC_Constant # (.UUID(64'd1609306207295225970 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());
  _64zmany # (.UUID(64'd3246727802728503082 ^ UUID)) _64zmany_5 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_0));

  wire [0:0] wire_0;
  assign \Saved?_1  = wire_0;
  assign \Saved?_2  = wire_0;
  wire [63:0] wire_1;
  assign wire_1 = Value;
  wire [63:0] wire_2;
  assign \Number_(stored)  = wire_2;
  wire [63:0] wire_3;
  wire [63:0] wire_3_0;
  wire [63:0] wire_3_1;
  assign wire_3 = wire_3_0|wire_3_1;
  wire [0:0] wire_4;

endmodule
