module minmaxz_32b (clk, rst, X, Y, Minimum, Maximum);
  parameter UUID = 0;
  parameter NAME = "";
  input wire clk;
  input wire rst;

  input  wire [63:0] X;
  input  wire [63:0] Y;
  output  wire [63:0] Minimum;
  output  wire [63:0] Maximum;

  TC_Not # (.UUID(64'd4161695322833104336 ^ UUID), .BIT_WIDTH(64'd1)) Not_0 (.in(wire_5), .out(wire_2));
  TC_Constant # (.UUID(64'd2146536127101255326 ^ UUID), .BIT_WIDTH(64'd1), .value(1'd0)) Off_1 (.out());
  TC_Mux # (.UUID(64'd807069437680946512 ^ UUID), .BIT_WIDTH(64'd32)) Mux32_2 (.sel(wire_2), .in0(wire_0[31:0]), .in1(wire_1[31:0]), .out(wire_4));
  TC_Mux # (.UUID(64'd4000245856948200043 ^ UUID), .BIT_WIDTH(64'd32)) Mux32_3 (.sel(wire_5), .in0(wire_0[31:0]), .in1(wire_1[31:0]), .out(wire_3));
  TC_LessI # (.UUID(64'd954040292442200866 ^ UUID), .BIT_WIDTH(64'd32)) LessI32_4 (.in0(wire_1[31:0]), .in1(wire_0[31:0]), .out(wire_5));

  wire [63:0] wire_0;
  assign wire_0 = Y;
  wire [63:0] wire_1;
  assign wire_1 = X;
  wire [0:0] wire_2;
  wire [31:0] wire_3;
  assign Minimum = {{32{1'b0}}, wire_3 };
  wire [31:0] wire_4;
  assign Maximum = {{32{1'b0}}, wire_4 };
  wire [0:0] wire_5;

endmodule
