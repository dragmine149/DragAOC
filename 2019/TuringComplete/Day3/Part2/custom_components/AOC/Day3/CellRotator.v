module CellRotator (clk, rst, Tick, Input, Pos_2, Pos_1, Written);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Tick;
  input  wire [63:0] Input;
  output  wire [63:0] Pos_2;
  output  wire [63:0] Pos_1;
  output  wire [0:0] Written;

  TC_DelayLine # (.UUID(64'd4167107758525898191 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_10), .out(wire_1));
  TC_DelayLine # (.UUID(64'd1844771220572620078 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_1 (.clk(clk), .rst(rst), .in(wire_9), .out(wire_4));
  TC_Switch # (.UUID(64'd4603078901373628981 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_2 (.en(wire_12), .in(wire_4), .out(wire_9_1));
  TC_DelayLine # (.UUID(64'd496358025367829928 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_3 (.clk(clk), .rst(rst), .in(wire_11), .out(wire_0));
  TC_DelayLine # (.UUID(64'd2084938692225789629 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_4 (.clk(clk), .rst(rst), .in(wire_2), .out(wire_3));
  TC_Switch # (.UUID(64'd1083703999431748276 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_5 (.en(wire_6), .in(wire_3), .out(wire_2_1));
  TC_Not # (.UUID(64'd3541591910826552025 ^ UUID), .BIT_WIDTH(64'd1)) Not_6 (.in(wire_5), .out(wire_6));
  TC_Or # (.UUID(64'd1333394776193514200 ^ UUID), .BIT_WIDTH(64'd1)) Or_7 (.in0(wire_3), .in1(wire_4), .out(wire_8));
  TC_Constant # (.UUID(64'd4547427933634268459 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_8 (.out());
  TC_Constant # (.UUID(64'd2637436588695494074 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_9 (.out());
  TC_Mux # (.UUID(64'd653833941214765738 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_10 (.sel(wire_5), .in0(wire_1), .in1(wire_0), .out(wire_10));
  TC_Mux # (.UUID(64'd2979531974306718429 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_11 (.sel(wire_5), .in0(wire_0), .in1(wire_7), .out(wire_11));
  TC_Not # (.UUID(64'd837924224480288987 ^ UUID), .BIT_WIDTH(64'd1)) Not_12 (.in(wire_5), .out(wire_12));
  OnOrOff # (.UUID(64'd2252264605357279146 ^ UUID)) OnOrOff_13 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_9_0));
  OnOrOff # (.UUID(64'd516257735472237058 ^ UUID)) OnOrOff_14 (.clk(clk), .rst(rst), .Input(wire_5), .Output(wire_2_0));

  wire [63:0] wire_0;
  assign Pos_2 = wire_0;
  wire [63:0] wire_1;
  assign Pos_1 = wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_2_0;
  wire [0:0] wire_2_1;
  assign wire_2 = wire_2_0|wire_2_1;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  assign wire_5 = Tick;
  wire [0:0] wire_6;
  wire [63:0] wire_7;
  assign wire_7 = Input;
  wire [0:0] wire_8;
  assign Written = wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_9_0;
  wire [0:0] wire_9_1;
  assign wire_9 = wire_9_0|wire_9_1;
  wire [63:0] wire_10;
  wire [63:0] wire_11;
  wire [0:0] wire_12;

endmodule
