module BusSwitcher (clk, rst, Line_End, Line_Start, Option_1, Option_2, Horizontal_End, Horizontal_Start, Vertical_End, Vertical_Start);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] Line_End;
  input  wire [63:0] Line_Start;
  input  wire [0:0] Option_1;
  input  wire [0:0] Option_2;
  output  wire [63:0] Horizontal_End;
  output  wire [63:0] Horizontal_Start;
  output  wire [63:0] Vertical_End;
  output  wire [63:0] Vertical_Start;

  TC_Switch # (.UUID(64'd1037011429401707377 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_0 (.en(wire_2), .in(wire_1), .out(Horizontal_End));
  TC_Switch # (.UUID(64'd3603658728742110537 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_1 (.en(wire_2), .in(wire_3), .out(Horizontal_Start));
  TC_Switch # (.UUID(64'd2399175043195774775 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_2 (.en(wire_0), .in(wire_1), .out(Vertical_End));
  TC_Switch # (.UUID(64'd434068716900805458 ^ UUID), .BIT_WIDTH(64'd64)) Output64z_3 (.en(wire_0), .in(wire_3), .out(Vertical_Start));
  TC_Constant # (.UUID(64'd1692086865912217551 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());
  TC_Constant # (.UUID(64'd1001741648685428336 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_5 (.out());
  TC_Constant # (.UUID(64'd3034624821468319770 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_6 (.out());
  TC_Constant # (.UUID(64'd1562297806871825111 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_7 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Option_1;
  wire [63:0] wire_1;
  assign wire_1 = Line_End;
  wire [0:0] wire_2;
  assign wire_2 = Option_2;
  wire [63:0] wire_3;
  assign wire_3 = Line_Start;

endmodule
