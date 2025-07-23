module x10zpnz_ZL16bZR (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input_1;
  input  wire [15:0] Input_2;
  output  wire [15:0] Output;

  TC_Constant # (.UUID(64'd567636894558252470 ^ UUID), .BIT_WIDTH(64'd8), .value(8'hA)) Constant8_0 (.out(wire_7));
  TC_Not # (.UUID(64'd1567346462929172721 ^ UUID), .BIT_WIDTH(64'd1)) Not_1 (.in(wire_5), .out(wire_0));
  TC_Mul # (.UUID(64'd4323488090029637784 ^ UUID), .BIT_WIDTH(64'd16)) Mul16_2 (.in0(wire_1), .in1({{8{1'b0}}, wire_7 }), .out0(wire_3), .out1());
  TC_Add # (.UUID(64'd2874539403569390277 ^ UUID), .BIT_WIDTH(64'd16)) Add16_3 (.in0(wire_3), .in1({{8{1'b0}}, wire_2 }), .ci(1'd0), .out(wire_6), .co());
  _8bgt10 # (.UUID(64'd2852418764236240148 ^ UUID)) _8bgt10_4 (.clk(clk), .rst(rst), .Input(wire_2), .Greater_than_10(wire_5));
  TC_Mux # (.UUID(64'd2440734806097346726 ^ UUID), .BIT_WIDTH(64'd16)) Mux16_5 (.sel(wire_0), .in0(wire_1), .in1(wire_6), .out(wire_4));

  wire [0:0] wire_0;
  wire [15:0] wire_1;
  assign wire_1 = Input_2;
  wire [7:0] wire_2;
  assign wire_2 = Input_1;
  wire [15:0] wire_3;
  wire [15:0] wire_4;
  assign Output = wire_4;
  wire [0:0] wire_5;
  wire [15:0] wire_6;
  wire [7:0] wire_7;

endmodule
