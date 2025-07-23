module CellRotator (clk, rst, Input, Tick, Pos_2, Pos_1, Written);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [31:0] Input;
  input  wire [0:0] Tick;
  output  wire [31:0] Pos_2;
  output  wire [31:0] Pos_1;
  output  wire [0:0] Written;

  TC_DelayLine # (.UUID(64'd4167107758525898191 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_0 (.clk(clk), .rst(rst), .in(wire_10), .out(wire_2));
  TC_DelayLine # (.UUID(64'd1844771220572620078 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_1 (.clk(clk), .rst(rst), .in(wire_5), .out(wire_8));
  TC_Switch # (.UUID(64'd4603078901373628981 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_2 (.en(wire_6), .in(wire_8), .out(wire_5_1));
  OnOrOff # (.UUID(64'd2252264605357279146 ^ UUID)) OnOrOff_3 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_5_0));
  TC_DelayLine # (.UUID(64'd496358025367829928 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_4 (.clk(clk), .rst(rst), .in(wire_0), .out(wire_4));
  TC_DelayLine # (.UUID(64'd2084938692225789629 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_5 (.clk(clk), .rst(rst), .in(wire_11), .out(wire_3));
  TC_Switch # (.UUID(64'd1083703999431748276 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_6 (.en(wire_9), .in(wire_3), .out(wire_11_0));
  TC_Not # (.UUID(64'd3541591910826552025 ^ UUID), .BIT_WIDTH(64'd1)) Not_7 (.in(wire_1), .out(wire_9));
  OnOrOff # (.UUID(64'd516257735472237058 ^ UUID)) OnOrOff_8 (.clk(clk), .rst(rst), .Input(wire_1), .Output(wire_11_1));
  TC_Or # (.UUID(64'd1333394776193514200 ^ UUID), .BIT_WIDTH(64'd1)) Or_9 (.in0(wire_3), .in1(wire_8), .out(wire_7));
  TC_Constant # (.UUID(64'd4547427933634268459 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_10 (.out());
  TC_Constant # (.UUID(64'd2637436588695494074 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_11 (.out());
  TC_Mux # (.UUID(64'd653833941214765738 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_12 (.sel(wire_1), .in0(wire_2), .in1(wire_4), .out(wire_10));
  TC_Mux # (.UUID(64'd2979531974306718429 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_13 (.sel(wire_1), .in0(wire_4), .in1({{32{1'b0}}, wire_12 }), .out(wire_0));
  TC_Not # (.UUID(64'd837924224480288987 ^ UUID), .BIT_WIDTH(64'd1)) Not_14 (.in(wire_1), .out(wire_6));

  wire [63:0] wire_0;
  wire [0:0] wire_1;
  assign wire_1 = Tick;
  wire [63:0] wire_2;
  assign Pos_1 = wire_2[31:0];
  wire [0:0] wire_3;
  wire [63:0] wire_4;
  assign Pos_2 = wire_4[31:0];
  wire [0:0] wire_5;
  wire [0:0] wire_5_0;
  wire [0:0] wire_5_1;
  assign wire_5 = wire_5_0|wire_5_1;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  assign Written = wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [63:0] wire_10;
  wire [0:0] wire_11;
  wire [0:0] wire_11_0;
  wire [0:0] wire_11_1;
  assign wire_11 = wire_11_0|wire_11_1;
  wire [31:0] wire_12;
  assign wire_12 = Input;

endmodule
