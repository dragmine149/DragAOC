module OnOrOff (clk, rst, Input, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input;
  output  wire [0:0] Output;

  TC_Switch # (.UUID(64'd4205007784991218036 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_0 (.en(wire_0), .in(wire_0), .out(Output));
  TC_Constant # (.UUID(64'd2656001879567037334 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Input;

endmodule
