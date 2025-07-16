module rSwitch (clk, rst, Input, Enable, Output);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [0:0] Input;
  input  wire [0:0] Enable;
  output  wire [0:0] Output;

  TC_Switch # (.UUID(64'd322785536489201904 ^ UUID), .BIT_WIDTH(64'd1)) Output1z_0 (.en(wire_0), .in(wire_1), .out(Output));
  TC_Constant # (.UUID(64'd2625550994847588994 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd1)) On_1 (.out());

  wire [0:0] wire_0;
  assign wire_0 = Enable;
  wire [0:0] wire_1;
  assign wire_1 = Input;

endmodule
