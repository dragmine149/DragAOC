module is0z_64b (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Input;
  output  wire [0:0] Output;

  _64bz_toz_32b # (.UUID(64'd870880980460894123 ^ UUID)) _64bz_toz_32b_0 (.clk(clk), .rst(rst), .Input(wire_5), .Output_1(wire_2), .Output_2(wire_1));
  is0z_32b # (.UUID(64'd3729807702673713066 ^ UUID)) is0z_32b_1 (.clk(clk), .rst(rst), .Input(wire_2), .Output(wire_0));
  is0z_32b # (.UUID(64'd3489082053998795290 ^ UUID)) is0z_32b_2 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_4));
  mand # (.UUID(64'd3038063054886311266 ^ UUID)) mand_3 (.clk(clk), .rst(rst), .Input_1(wire_4), .Input_2(wire_0), .Output(wire_3));

  wire [0:0] wire_0;
  wire [31:0] wire_1;
  wire [31:0] wire_2;
  wire [0:0] wire_3;
  assign Output = wire_3;
  wire [0:0] wire_4;
  wire [63:0] wire_5;
  assign wire_5 = Input;

endmodule
