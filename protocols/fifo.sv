
module fifo  #(
    parameter WIDTH = 32,
    parameter DEPTH = 4
)(
  // implicit control
  input wire logic clk,
  input wire logic reset,
  input wire logic go,
  output logic done,
  // explicit control
  input wire logic op,
  // data
  input wire logic [WIDTH-1:0]  in,
  output logic     [WIDTH-1:0] out
);


// done after one cycle
reg done_p1;
reg done_p2;


always_ff @(posedge clk) begin
    if (reset) begin
        done_p1 <= 0;
        done_p2 <= 0;
    end else begin
        done_p1 <= go && !done_p1;
        done_p2 <= done_p1;
    end
end

assign done = done_p2;
assign out = done? 32'd123 : 32'd0;


endmodule