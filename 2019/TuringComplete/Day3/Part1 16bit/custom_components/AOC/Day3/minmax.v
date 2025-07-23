module minmax (clk, rst, Input_1, Input_2, Minimum, Maximum);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [15:0] Input_1;
  input  wire [15:0] Input_2;
  output  wire [15:0] Minimum;
  output  wire [15:0] Maximum;

  TC_Mux # (.UUID(64'd2469713993924360560 ^ UUID), .BIT_WIDTH(64'd16)) Mux16_0 (.sel(wire_2), .in0(wire_0), .in1(wire_1), .out(wire_5));
  TC_LessI # (.UUID(64'd3464757118785046839 ^ UUID), .BIT_WIDTH(64'd16)) LessI16_1 (.in0(wire_1), .in1(wire_0), .out(wire_2));
  TC_Mux # (.UUID(64'd506182523157066850 ^ UUID), .BIT_WIDTH(64'd16)) Mux16_2 (.sel(wire_4), .in0(wire_0), .in1(wire_1), .out(wire_3));
  TC_Not # (.UUID(64'd4161695322833104336 ^ UUID), .BIT_WIDTH(64'd1)) Not_3 (.in(wire_2), .out(wire_4));
  TC_Constant # (.UUID(64'd2146536127101255326 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_4 (.out());

  wire [15:0] wire_0;
  assign wire_0 = Input_2;
  wire [15:0] wire_1;
  assign wire_1 = Input_1;
  wire [0:0] wire_2;
  wire [15:0] wire_3;
  assign Maximum = wire_3;
  wire [0:0] wire_4;
  wire [15:0] wire_5;
  assign Minimum = wire_5;

endmodule
