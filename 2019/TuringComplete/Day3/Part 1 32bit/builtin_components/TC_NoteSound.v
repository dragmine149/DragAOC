// Dummy implementation, does nothing
module TC_NoteSound (clk, rst, command, note, pitch);
    parameter UUID = 0;
    parameter NAME = "";
    input clk;
    input rst;
    input [7:0] command;
    input [7:0] note;
    input [7:0] pitch;
endmodule
