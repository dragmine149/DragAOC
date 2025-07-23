module TC_IndexerByte (in, out);
    parameter UUID = 0;
    parameter NAME = "";
    parameter INDEX = 0;
    input [63:0] in;
    output [7:0] out;
    
    assign out = in[INDEX*8+7:INDEX*8];
endmodule
