module _4beq (clk, rst, Input_1, Input_2, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [7:0] Input_1;
  input  wire [7:0] Input_2;
  output  wire [0:0] Output;

  TC_Xor # (.UUID(64'd1310372816885407589 ^ UUID), .BIT_WIDTH(64'd1)) Xor_0 (.in0(wire_3), .in1(wire_9), .out(wire_5));
  TC_Xor # (.UUID(64'd4287642207717123448 ^ UUID), .BIT_WIDTH(64'd1)) Xor_1 (.in0(wire_6), .in1(wire_12), .out(wire_2));
  TC_Xor # (.UUID(64'd1440491588884435132 ^ UUID), .BIT_WIDTH(64'd1)) Xor_2 (.in0(wire_8), .in1(wire_0), .out(wire_14));
  TC_Xor # (.UUID(64'd3215896722570022660 ^ UUID), .BIT_WIDTH(64'd1)) Xor_3 (.in0(wire_16), .in1(wire_4), .out(wire_11));
  TC_Nor # (.UUID(64'd4529240383891744025 ^ UUID), .BIT_WIDTH(64'd1)) Nor_4 (.in0(wire_1), .in1(wire_13), .out(wire_7));
  TC_Or # (.UUID(64'd4161726975673950338 ^ UUID), .BIT_WIDTH(64'd1)) Or_5 (.in0(wire_14), .in1(wire_11), .out(wire_13));
  TC_Switch # (.UUID(64'd149389068225253891 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_6 (.en(wire_2), .in(wire_2), .out(wire_1_0));
  TC_Switch # (.UUID(64'd2107967511438961563 ^ UUID), .BIT_WIDTH(64'd1)) Switch1_7 (.en(wire_5), .in(wire_5), .out(wire_1_1));
  TC_Splitter8 # (.UUID(64'd1243825339593762698 ^ UUID)) Splitter8_8 (.in(wire_15), .out0(wire_3), .out1(wire_6), .out2(wire_8), .out3(wire_16), .out4(), .out5(), .out6(), .out7());
  TC_Splitter8 # (.UUID(64'd2965861629640661421 ^ UUID)) Splitter8_9 (.in(wire_10), .out0(wire_9), .out1(wire_12), .out2(wire_0), .out3(wire_4), .out4(), .out5(), .out6(), .out7());

  wire [0:0] wire_0;
  wire [0:0] wire_1;
  wire [0:0] wire_1_0;
  wire [0:0] wire_1_1;
  assign wire_1 = wire_1_0|wire_1_1;
  wire [0:0] wire_2;
  wire [0:0] wire_3;
  wire [0:0] wire_4;
  wire [0:0] wire_5;
  wire [0:0] wire_6;
  wire [0:0] wire_7;
  assign Output = wire_7;
  wire [0:0] wire_8;
  wire [0:0] wire_9;
  wire [7:0] wire_10;
  assign wire_10 = Input_2;
  wire [0:0] wire_11;
  wire [0:0] wire_12;
  wire [0:0] wire_13;
  wire [0:0] wire_14;
  wire [7:0] wire_15;
  assign wire_15 = Input_1;
  wire [0:0] wire_16;

endmodule
