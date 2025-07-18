module Day1Part2 (clk, rst);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;


  TC_Register # (.UUID(64'd4178573859148191576 ^ UUID), .BIT_WIDTH(64'd64)) Register64_0 (.clk(clk), .rst(rst), .load(wire_19), .save(wire_1), .in(wire_28), .out(wire_7));
  TC_Add # (.UUID(64'd359763711881990385 ^ UUID), .BIT_WIDTH(64'd64)) Add64_1 (.in0({{56{1'b0}}, wire_31 }), .in1(wire_7), .ci(1'd0), .out(wire_28), .co());
  TC_Equal # (.UUID(64'd928735983604669128 ^ UUID), .BIT_WIDTH(64'd8)) Equal8_2 (.in0(wire_31), .in1(wire_34), .out(wire_18));
  TC_Constant # (.UUID(64'd2218857169047162011 ^ UUID), .BIT_WIDTH(64'd8), .value(8'h9)) Constant8_3 (.out(wire_34));
  TC_Counter # (.UUID(64'd1097574418592050277 ^ UUID), .BIT_WIDTH(64'd64), .count(64'd1)) Counter64_4 (.clk(clk), .rst(rst), .save(wire_38), .in({{63{1'b0}}, wire_38 }), .out(wire_20));
  TC_Register # (.UUID(64'd2413979704567083807 ^ UUID), .BIT_WIDTH(64'd64)) Register64_5 (.clk(clk), .rst(rst), .load(wire_27), .save(wire_17), .in(wire_6), .out(wire_21));
  TC_Constant # (.UUID(64'd2793433701128027946 ^ UUID), .BIT_WIDTH(64'd64), .value(64'hFFFFFFFFFFFFFFFF)) Constant64_6 (.out(wire_35));
  TC_Constant # (.UUID(64'd1184275376460161862 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_7 (.out(wire_27));
  TC_Not # (.UUID(64'd4603214268349776896 ^ UUID), .BIT_WIDTH(64'd1)) Not_8 (.in(wire_0), .out(wire_17));
  TC_Mux # (.UUID(64'd3836885967411031723 ^ UUID), .BIT_WIDTH(64'd64)) Mux64_9 (.sel(wire_0), .in0(wire_35), .in1(wire_7), .out(wire_14));
  TC_DelayLine # (.UUID(64'd2178678318095205493 ^ UUID), .BIT_WIDTH(64'd64)) DelayLine64_10 (.clk(clk), .rst(rst), .in(wire_37), .out(wire_22));
  TC_Not # (.UUID(64'd1630915932547760616 ^ UUID), .BIT_WIDTH(64'd1)) Not_11 (.in(wire_13), .out(wire_1));
  TC_Constant # (.UUID(64'd2213483610403812941 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_12 (.out(wire_19));
  TC_Not # (.UUID(64'd2394884979716713933 ^ UUID), .BIT_WIDTH(64'd1)) Not_13 (.in(wire_0), .out(wire_9));
  TC_Ram # (.UUID(64'd811351278422078506 ^ UUID), .WORD_WIDTH(64'd64), .WORD_COUNT(64'd1250)) Ram_14 (.clk(clk), .rst(rst), .load(wire_8), .save(wire_29), .address(wire_24[31:0]), .in0(wire_36), .in1(64'd0), .in2(64'd0), .in3(64'd0), .out0(wire_3), .out1(), .out2(), .out3());
  TC_FileLoader # (.UUID(64'd2193553924715941358 ^ UUID), .DEFAULT_FILE_NAME("day1")) FileLoader_15 (.clk(clk), .rst(rst), .en(wire_41), .address(wire_14), .out(wire_6));
  TC_Constant # (.UUID(64'd3595939561111802633 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_16 (.out(wire_41));
  TC_Switch # (.UUID(64'd3013778101405626680 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_17 (.en(wire_11), .in(wire_1), .out(wire_29));
  TC_Switch # (.UUID(64'd3957714978086000962 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_18 (.en(wire_11), .in(wire_20), .out(wire_24_0));
  TC_Switch # (.UUID(64'd4006497041254316547 ^ UUID), .BIT_WIDTH(64'd64)) Switch64_19 (.en(wire_11), .in(wire_5), .out(wire_36));
  TC_LessU # (.UUID(64'd2412733997253528222 ^ UUID), .BIT_WIDTH(64'd64)) LessU64_20 (.in0(wire_7), .in1(wire_21), .out(wire_11));
  TC_Register # (.UUID(64'd1998148612828582261 ^ UUID), .BIT_WIDTH(64'd64)) Register64_21 (.clk(clk), .rst(rst), .load(wire_23), .save(wire_8), .in(wire_40), .out(wire_32));
  TC_Add # (.UUID(64'd3023020307212303517 ^ UUID), .BIT_WIDTH(64'd64)) Add64_22 (.in0(wire_16), .in1(wire_32), .ci(1'd0), .out(wire_40), .co());
  TC_Constant # (.UUID(64'd1388399570672712544 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_23 (.out(wire_23));
  TC_Counter # (.UUID(64'd1973994861164209717 ^ UUID), .BIT_WIDTH(64'd64), .count(64'd1)) Counter64_24 (.clk(clk), .rst(rst), .save(wire_2), .in(wire_15), .out(wire_15));
  TC_Not # (.UUID(64'd927835141595733835 ^ UUID), .BIT_WIDTH(64'd1)) Not_25 (.in(wire_11), .out(wire_26));
  TC_FileLoader # (.UUID(64'd1289831107320614410 ^ UUID), .DEFAULT_FILE_NAME("day1_test_1")) FileLoader_26 (.clk(clk), .rst(rst), .en(1'd0), .address(64'd0), .out());
  TC_Register # (.UUID(64'd2632168926008665082 ^ UUID), .BIT_WIDTH(64'd64)) Register64_27 (.clk(clk), .rst(rst), .load(wire_10), .save(wire_25), .in(wire_20), .out(wire_12));
  TC_Switch # (.UUID(64'd2600677178569538476 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_28 (.en(wire_39), .in(wire_26), .out(wire_25));
  TC_DelayLine # (.UUID(64'd3616072873250103679 ^ UUID), .BIT_WIDTH(64'd1)) DelayLine1_29 (.clk(clk), .rst(rst), .in(wire_11), .out(wire_39));
  TC_Constant # (.UUID(64'd1666034379207229566 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_30 (.out(wire_10));
  TC_Not # (.UUID(64'd101522584180394033 ^ UUID), .BIT_WIDTH(64'd1)) Not_31 (.in(wire_8), .out(wire_33));
  TC_Equal # (.UUID(64'd4073392919419213019 ^ UUID), .BIT_WIDTH(64'd64)) Equal64_32 (.in0(wire_15), .in1(wire_12), .out(wire_4));
  TC_Halt # (.UUID(64'd805079572016111755 ^ UUID), .HALT_MESSAGE("Program finished!")) Halt_33 (.clk(clk), .rst(rst), .en(wire_30));
  TC_Switch # (.UUID(64'd1270767012348834115 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_34 (.en(wire_8), .in(wire_4), .out(wire_30));
  BytesToNumbers # (.UUID(64'd4206582659042050133 ^ UUID)) BytesToNumbers_35 (.clk(clk), .rst(rst), .Carry(wire_22), .\Input_(64b) (wire_6), .Offset(wire_31), .\Output_(8b) (wire_5));
  _64zmany # (.UUID(64'd3133133748172620224 ^ UUID)) _64zmany_36 (.clk(clk), .rst(rst), .Input(wire_21), .Output(wire_0));
  flippedzm64bzmswitch # (.UUID(64'd1985891020947270661 ^ UUID)) flippedzm64bzmswitch_37 (.clk(clk), .rst(rst), .Input_1(wire_5), .Input_2(wire_18), .Output(wire_37));
  OnOrOff # (.UUID(64'd1600200685999643413 ^ UUID)) OnOrOff_38 (.clk(clk), .rst(rst), .Input(wire_18), .Output(wire_13_1));
  OnOrOff # (.UUID(64'd946715927199943045 ^ UUID)) OnOrOff_39 (.clk(clk), .rst(rst), .Input(wire_9), .Output(wire_13_0));
  flippedzm64bzmswitch # (.UUID(64'd380343949002747622 ^ UUID)) flippedzm64bzmswitch_40 (.clk(clk), .rst(rst), .Input_1(wire_15), .Input_2(wire_26), .Output(wire_24_1));
  _64zmany # (.UUID(64'd2095873869615652318 ^ UUID)) _64zmany_41 (.clk(clk), .rst(rst), .Input(wire_12), .Output(wire_8));
  OnOrOff # (.UUID(64'd4303464689983108020 ^ UUID)) OnOrOff_42 (.clk(clk), .rst(rst), .Input(wire_33), .Output(wire_2_1));
  D3rm2z_loop # (.UUID(64'd3114400491708135752 ^ UUID)) D3rm2z_loop_43 (.clk(clk), .rst(rst), .Input(wire_3), .Result(wire_16), .In_Progress(wire_2_0));

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_2;
  wire [0:0] wire_2_0;
  wire [0:0] wire_2_1;
  assign wire_2 = wire_2_0|wire_2_1;
  wire [63:0] wire_3;
  wire [0:0] wire_4;
  wire [63:0] wire_5;
  wire [63:0] wire_6;
  wire [63:0] wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [0:0] wire_10;
  wire [0:0] wire_11;
  wire [63:0] wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_13_0;
  wire [0:0] wire_13_1;
  assign wire_13 = wire_13_0|wire_13_1;
  wire [63:0] wire_14;
  wire [63:0] wire_15;
  wire [63:0] wire_16;
  wire [0:0] wire_17;
  wire [0:0] wire_18;
  wire [0:0] wire_19;
  wire [63:0] wire_20;
  wire [63:0] wire_21;
  wire [63:0] wire_22;
  wire [0:0] wire_23;
  wire [63:0] wire_24;
  wire [63:0] wire_24_0;
  wire [63:0] wire_24_1;
  assign wire_24 = wire_24_0|wire_24_1;
  wire [0:0] wire_25;
  wire [0:0] wire_26;
  wire [0:0] wire_27;
  wire [63:0] wire_28;
  wire [0:0] wire_29;
  wire [0:0] wire_30;
  wire [7:0] wire_31;
  wire [63:0] wire_32;
  wire [0:0] wire_33;
  wire [7:0] wire_34;
  wire [63:0] wire_35;
  wire [63:0] wire_36;
  wire [63:0] wire_37;
  wire [0:0] wire_38;
  assign wire_38 = 0;
  wire [0:0] wire_39;
  wire [63:0] wire_40;
  wire [0:0] wire_41;

endmodule
