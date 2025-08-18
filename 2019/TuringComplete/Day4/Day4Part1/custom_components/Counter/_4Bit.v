module _4Bit (clk, rst, Overrite_Value, Overrite, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Overrite_Value;
  input  wire [0:0] Overrite;
  output  wire [7:0] Output;

  TC_Not # (.UUID(64'd3342017385443961307 ^ UUID), .BIT_WIDTH(64'd1)) Not_0 (.in(wire_13), .out(wire_5));
  TC_DelayLine # (.UUID(64'd1421130288116031130 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_1 (.clk(clk), .rst(rst), .in(wire_21), .out(wire_12));
  TC_DelayLine # (.UUID(64'd885478476140123947 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_2 (.clk(clk), .rst(rst), .in(wire_2), .out(wire_14));
  TC_DelayLine # (.UUID(64'd4277950762344526856 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_3 (.clk(clk), .rst(rst), .in(wire_6), .out(wire_16));
  TC_DelayLine # (.UUID(64'd2505267336687574612 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_4 (.clk(clk), .rst(rst), .in(wire_11), .out(wire_19));
  outputswitch # (.UUID(64'd3116190216111583975 ^ UUID)) outputswitch_5 (.clk(clk), .rst(rst), .Input_A(wire_10), .Input_B(wire_5), .Path(wire_1), .Output(wire_11));
  xorandoutputswitch # (.UUID(64'd4549543780277979400 ^ UUID)) xorandoutputswitch_6 (.clk(clk), .rst(rst), .Bit(wire_4), .Carry(wire_18), .Overrite_bit(wire_8), .Overite(wire_17), .Overrite(wire_20), .And(wire_23), .Output(wire_2));
  xorandoutputswitch # (.UUID(64'd3158975692216946913 ^ UUID)) xorandoutputswitch_7 (.clk(clk), .rst(rst), .Bit(wire_15), .Carry(wire_13), .Overrite_bit(wire_9), .Overite(wire_1), .Overrite(wire_17), .And(wire_18), .Output(wire_6));
  xorandoutputswitchz_NC # (.UUID(64'd70589873830998183 ^ UUID)) xorandoutputswitchz_NC_8 (.clk(clk), .rst(rst), .Bit(wire_22), .Carry(wire_23), .Overrite_bit(wire_0), .Overite(wire_20), .Output(wire_21));
  _4zmBitz_Maker # (.UUID(64'd200691386384360365 ^ UUID)) _4zmBitz_Maker_9 (.clk(clk), .rst(rst), .\1 (wire_19), .\2 (wire_16), .\4 (wire_14), .\8 (wire_12), .Output(wire_3));
  _4zmBitz_Seperator # (.UUID(64'd2947243087433919892 ^ UUID)) _4zmBitz_Seperator_10 (.clk(clk), .rst(rst), .Input(wire_7), .Output_1(wire_10), .Output_2(wire_9), .Output_3(wire_8), .Output_4(wire_0));
  _4zmBitz_Seperator # (.UUID(64'd1311917863832729966 ^ UUID)) _4zmBitz_Seperator_11 (.clk(clk), .rst(rst), .Input(wire_3), .Output_1(wire_13), .Output_2(wire_15), .Output_3(wire_4), .Output_4(wire_22));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Overrite;
  wire [0:0] wire_2;
  wire [7:0] wire_3;
  assign Output = wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [7:0] wire_7;
  assign wire_7 = Overrite_Value;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;
  wire [0:0] wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_14;
  wire [0:0] wire_15;
  wire [0:0] wire_16;
  wire [0:0] wire_17;
  wire [0:0] wire_18;
  wire [0:0] wire_19;
  wire [0:0] wire_20;
  wire [0:0] wire_21;
  wire [0:0] wire_22;
  wire [0:0] wire_23;

endmodule
