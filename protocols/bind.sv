module fifo(
  input  logic clk,   // global clock
  input  logic reset, // 1-cycle active-high reset
  input  logic go,    // calyx standard control
  input  logic op,    // 0=push, 1=pop
  input logic [31:0] push_input ,  // push.input
  output logic [31:0] pop_output ,  // pop.output
  output logic done   // calyx standard control
);

  logic [31:0] dut_data_i; // In
  logic dut_v_i; // In
  logic dut_enq_not_deq_i; // In
  logic dut_reset_i; // In
  logic dut_full_o; // Out
  logic dut_empty_o; // Out
  logic [31:0] dut_data_o; // Out

  fifo_wrapper fifo_wrapper_inst (
    .clk_i (clk),
    .data_i (dut_data_i),
    .v_i (dut_v_i),
    .enq_not_deq_i (dut_enq_not_deq_i),
    .reset_i (dut_reset_i),
    .full_o (dut_full_o),
    .empty_o (dut_empty_o),
    .data_o (dut_data_o)
  );

  typedef enum logic [1:0] {
    ST_START  = 2'd0,
    ST_DONE  = 2'd1,
    ST_NODE_2  = 2'd2
  } state_t;

  state_t state, next_state;
  logic [31:0] _output_value, next__output_value;

  always_ff @(posedge clk) begin
    if (reset) begin
      state <= ST_START;
      _output_value <= 32'b00000000000000000000000000000000;
    end else begin
      state <= next_state;
      _output_value <= next__output_value;
    end
  end

  // connect output arguments to registers that capture them
  assign pop_output = _output_value;

  // done is true iff we are in the done state
  assign done = state == ST_DONE;

  always_comb begin
    // register default to their previous values
    next_state = state;
    next__output_value = _output_value;

    // default values for dut inputs
    // TODO: find all the assignments to this particular pin for a better default assignment!
    dut_data_i = 32'd0;
    // TODO: find all the assignments to this particular pin for a better default assignment!
    dut_v_i = 1'd0;
    // TODO: find all the assignments to this particular pin for a better default assignment!
    dut_enq_not_deq_i = 1'd0;
    // TODO: find all the assignments to this particular pin for a better default assignment!
    dut_reset_i = 1'd0;

    if (reset) begin
      // dut_data_i = DontCare
      dut_v_i = 1'b0;
      // dut_enq_not_deq_i = DontCare
      dut_reset_i = 1'b1;
    end else begin
      unique case (state)
      ST_START: begin
        dut_data_i = push_input;
        dut_v_i = ~(~(go));
        dut_enq_not_deq_i = ~((go) & (op));
        dut_reset_i = 1'b0;
        // assert_false((((go) & (~(op))) | ((go) & (op))) & (~(go)))
        if ((go) & (~(op)))
          next_state = ST_DONE;
        if ((go) & (op))
          next_state = ST_NODE_2;
      end
      ST_DONE: begin
        // dut_data_i = DontCare
        dut_v_i = 1'b0;
        // dut_enq_not_deq_i = DontCare
        dut_reset_i = 1'b0;
        next_state = ST_START;
      end
      ST_NODE_2: begin
        // dut_data_i = DontCare
        dut_v_i = 1'b0;
        // dut_enq_not_deq_i = DontCare
        dut_reset_i = 1'b0;
        next__output_value = dut_data_o;
        next_state = ST_DONE;
      end
      endcase
    end
  end
endmodule




module fifo_wrapper (
    input clk_i,
    input reset_i,
    input [31:0] data_i,
    input v_i,
    input enq_not_deq_i,
    output full_o,
    output empty_o,
    output [31:0] data_o
);

    // Instantiate the FIFO with explicit 32-bit width and 16 elements
    bsg_fifo_1rw_large #(
        .width_p(32),
        .els_p(16),
        .verbose_p(0)
    ) fifo_inst (
        .clk_i(clk_i),
        .reset_i(reset_i),
        .data_i(data_i),
        .v_i(v_i),
        .enq_not_deq_i(enq_not_deq_i),
        .full_o(full_o),
        .empty_o(empty_o),
        .data_o(data_o)
    );

endmodule


// Taken from https://github.com/bespoke-silicon-group/basejump_stl/blob/643b2bd21c7d5e70583432ba87fa578fcb9f3b45/bsg_misc/bsg_defines.v

`ifndef BSG_DEFINES_V
`define BSG_DEFINES_V

`define BSG_MAX(x,y) (((x)>(y)) ? (x) : (y))
`define BSG_MIN(x,y) (((x)<(y)) ? (x) : (y))

// maps 1 --> 1 instead of to 0
`define BSG_SAFE_CLOG2(x) ( ((x)==1) ? 1 : $clog2((x)))
`define BSG_IS_POW2(x) ( (1 << $clog2(x)) == (x))
`define BSG_WIDTH(x) ($clog2(x+1))
`define BSG_SAFE_MINUS(x, y) (((x)-(y)) < 0) ? 0 : ((x)-(y))

// calculate ceil(x/y) 
`define BSG_CDIV(x,y) (((x)+(y)-1)/(y))

`ifdef SYNTHESIS
`define BSG_UNDEFINED_IN_SIM(val) (val)
`else
`define BSG_UNDEFINED_IN_SIM(val) ('X)
`endif

`ifdef VERILATOR
`define BSG_HIDE_FROM_VERILATOR(val)
`else
`define BSG_HIDE_FROM_VERILATOR(val) val
`endif

`ifdef SYNTHESIS
`define BSG_DISCONNECTED_IN_SIM(val) (val)
`elsif VERILATOR
`define BSG_DISCONNECTED_IN_SIM(val) (val)
`else
`define BSG_DISCONNECTED_IN_SIM(val) ('z)
`endif

`define BSG_STRINGIFY(x) `"x`"

// using C-style shifts instead of a[i] allows the parameter of BSG_GET_BIT to be a parameter subrange                                                                                                                                                                               
// e.g., parameter[4:1][1], which DC 2016.12 does not allow                                                                                                                                                                                                                          

`define BSG_GET_BIT(X,NUM) (((X)>>(NUM))&1'b1)

// This version of countones works in synthesis, but only up to 64 bits                                                                                                                                                                                                              
// we do a funny thing where we propagate X's in simulation if it is more than 64 bits                                                                                                                                                                                               
// and in synthesis, go ahead and ignore the high bits                                                                                                                                                                      

`define BSG_COUNTONES_SYNTH(y) (($bits(y) < 65) ? 1'b0 : `BSG_UNDEFINED_IN_SIM(1'b0)) + (`BSG_GET_BIT(y,0) +`BSG_GET_BIT(y,1) +`BSG_GET_BIT(y,2) +`BSG_GET_BIT(y,3) +`BSG_GET_BIT(y,4) +`BSG_GET_BIT(y,5) +`BSG_GET_BIT(y,6)+`BSG_GET_BIT(y,7) +`BSG_GET_BIT(y,8)+`BSG_GET_BIT(y,9) \
                                                                                       +`BSG_GET_BIT(y,10)+`BSG_GET_BIT(y,11)+`BSG_GET_BIT(y,12)+`BSG_GET_BIT(y,13)+`BSG_GET_BIT(y,14)+`BSG_GET_BIT(y,15)+`BSG_GET_BIT(y,16)+`BSG_GET_BIT(y,17)+`BSG_GET_BIT(y,18)+`BSG_GET_BIT(y,19) \
                                                                                       +`BSG_GET_BIT(y,20)+`BSG_GET_BIT(y,21)+`BSG_GET_BIT(y,22)+`BSG_GET_BIT(y,23)+`BSG_GET_BIT(y,24)+`BSG_GET_BIT(y,25)+`BSG_GET_BIT(y,26)+`BSG_GET_BIT(y,27)+`BSG_GET_BIT(y,28)+`BSG_GET_BIT(y,29) \
                                                                                       +`BSG_GET_BIT(y,30)+`BSG_GET_BIT(y,31)+`BSG_GET_BIT(y,32)+`BSG_GET_BIT(y,33)+`BSG_GET_BIT(y,34)+`BSG_GET_BIT(y,35)+`BSG_GET_BIT(y,36)+`BSG_GET_BIT(y,37)+`BSG_GET_BIT(y,38)+`BSG_GET_BIT(y,39) \
                                                                                       +`BSG_GET_BIT(y,40)+`BSG_GET_BIT(y,41)+`BSG_GET_BIT(y,42)+`BSG_GET_BIT(y,43)+`BSG_GET_BIT(y,44)+`BSG_GET_BIT(y,45)+`BSG_GET_BIT(y,46)+`BSG_GET_BIT(y,47)+`BSG_GET_BIT(y,48)+`BSG_GET_BIT(y,49) \
                                                                                       +`BSG_GET_BIT(y,50)+`BSG_GET_BIT(y,51)+`BSG_GET_BIT(y,52)+`BSG_GET_BIT(y,53)+`BSG_GET_BIT(y,54)+`BSG_GET_BIT(y,55)+`BSG_GET_BIT(y,56)+`BSG_GET_BIT(y,57)+`BSG_GET_BIT(y,58)+`BSG_GET_BIT(y,59) \
                                                                                       +`BSG_GET_BIT(y,60)+`BSG_GET_BIT(y,61)+`BSG_GET_BIT(y,62)+`BSG_GET_BIT(y,63))

// nullify rpgroups
`ifndef rpgroup
`define rpgroup(x)
`endif

`endif

// From https://github.com/bespoke-silicon-group/basejump_stl/blob/643b2bd21c7d5e70583432ba87fa578fcb9f3b45/bsg_misc/bsg_circular_ptr.v

// MBT
//
// 10/27/14
//
// implements a circular pointer that
// can be incremented by at most max_add_p
// and points to slots_p slots.
//

module bsg_circular_ptr #(parameter slots_p     = -1
                          , parameter max_add_p = -1
                          // local param
                          , parameter ptr_width_lp = `BSG_SAFE_CLOG2(slots_p)
			  )
   (input clk
    , input reset_i
    , input  [$clog2(max_add_p+1)-1:0] add_i
    , output [ptr_width_lp-1:0] o
    , output [ptr_width_lp-1:0] n_o
    );

   logic [ptr_width_lp-1:0] ptr_r, ptr_n, ptr_nowrap;
   logic [ptr_width_lp:0]   ptr_wrap;

   assign o = ptr_r;
   assign n_o = ptr_n;

   // increment round robin pointers
   always @(posedge clk)
     if (reset_i) ptr_r <= 0;
     else       ptr_r <= ptr_n;

   if (slots_p == 1)
     begin
	assign ptr_n = 1'b0;
	wire ignore = |add_i;
     end
   else
   
     // fixme performance optimization:
     // we should handle add-by-1 and non-power-of-two
     // in the same way as power-of-two and add-by-1
     // with a compare of ptr_r to slots_p-1 to check for
     // zeroing the ptr_r_p1 value.
     
    if (`BSG_IS_POW2(slots_p))
       begin
	  // reduce critical path on add_i signal
	  if (max_add_p == 1)
	    begin
	       wire [ptr_width_lp-1:0] ptr_r_p1 = ptr_r + 1'b1;
	       assign  ptr_n = add_i ? ptr_r_p1 : ptr_r;
	    end
	  else
	    assign  ptr_n = ptr_width_lp ' (ptr_r + add_i);
       end
     else
       begin: notpow2
          // compute wrapped and non-wrap cases
          // in parallel
          assign ptr_wrap = (ptr_width_lp+1)'({ 1'b0, ptr_r } - slots_p + add_i);
          assign ptr_nowrap = ptr_r + add_i;

          // if (ptr_r + add_i - slots_p >= 0)
          // then we have wrapped around
          assign ptr_n = ~ptr_wrap[ptr_width_lp] ? ptr_wrap[0+:ptr_width_lp] : ptr_nowrap;

	  // synopsys translate_off
          always_comb
            begin
              assert( (ptr_n < slots_p) || (|ptr_n === 'X) || reset_i || (add_i > slots_p))
                else $error("bsg_circular_ptr counter overflow (ptr_r=%b/add_i=%b/ptr_wrap=%b/ptr_n=%b)",ptr_r,add_i,ptr_wrap,ptr_n, slots_p);
            end
	  // synopsys translate_on
end
endmodule // bsg_circular_ptr


// From https://github.com/bespoke-silicon-group/basejump_stl/blob/643b2bd21c7d5e70583432ba87fa578fcb9f3b45/bsg_mem/bsg_mem_1rw_sync_synth.v

// MBT 11/9/2014
//
// Synchronous 1-port ram.
// Only one read or one write may be done per cycle.
//
// NOTE: Users of BaseJump STL should not instantiate this module directly
// they should use bsg_mem_1rw_sync.


module bsg_mem_1rw_sync_synth
  #(parameter width_p=-1
    , parameter els_p=-1
    , parameter latch_last_read_p=0
    , parameter addr_width_lp=`BSG_SAFE_CLOG2(els_p)
   )
   (input   clk_i
	 	, input v_i
		, input reset_i
    , input [`BSG_SAFE_MINUS(width_p, 1):0] data_i
    , input [addr_width_lp-1:0] addr_i
    , input w_i
    , output logic [`BSG_SAFE_MINUS(width_p, 1):0]  data_o
    );

  wire unused = reset_i;

  if (width_p == 0)
   begin: z
     wire unused0 = &{clk_i, v_i, data_i, addr_i, w_i};
     assign data_o = '0;
   end
  else
   begin: nz

  logic [addr_width_lp-1:0] addr_r;
  logic [width_p-1:0]    mem [els_p-1:0];
  logic read_en;
  logic [width_p-1:0] data_out;

  assign read_en = v_i & ~w_i;
  assign data_out = mem[addr_r];

  always_ff @ (posedge clk_i) 
    if (read_en)
      addr_r <= addr_i;
    else
      addr_r <= 'X;

  if (latch_last_read_p)
    begin: llr
      logic read_en_r; 

      bsg_dff #(
        .width_p(1)
      ) read_en_dff (
        .clk_i(clk_i)
        ,.data_i(read_en)
        ,.data_o(read_en_r)
      );

      bsg_dff_en_bypass #(
        .width_p(width_p)
      ) dff_bypass (
        .clk_i(clk_i)
        ,.en_i(read_en_r)
        ,.data_i(data_out)
        ,.data_o(data_o)
      );
    end
  else
    begin: no_llr
      assign data_o = data_out;
    end


  always_ff @(posedge clk_i)
    if (v_i & w_i) 
      mem[addr_i] <= data_i;

   end // non_zero_width
   // synopsys translate_off
   initial
     begin
        $display("## %L: instantiating width_p=%d, els_p=%d (%m)",width_p,els_p);
     end

   always_ff @(negedge clk_i)
     if (v_i)
       assert ( (v_i !== 1'b1) || (reset_i === 'X) || (reset_i === 1'b1) || (addr_i < els_p))
         else $error("Invalid address %x to %m of size %x (reset_i = %b, v_i = %b, clk_i = %b)\n", addr_i, els_p, reset_i, v_i, clk_i);
   // synopsys translate_on

endmodule



// From https://github.com/bespoke-silicon-group/basejump_stl/blob/643b2bd21c7d5e70583432ba87fa578fcb9f3b45/bsg_mem/bsg_mem_1rw_sync.v

// MBT 11/9/2014
//
// Synchronous 1-port ram.
// Only one read or one write may be done per cycle.
//


module bsg_mem_1rw_sync #(parameter width_p=-1
                          , parameter els_p=-1
                          , parameter latch_last_read_p=0
                          , parameter addr_width_lp=`BSG_SAFE_CLOG2(els_p)
                          , parameter enable_clock_gating_p=0
                          )
   (input   clk_i
    , input reset_i
    , input [`BSG_SAFE_MINUS(width_p, 1):0] data_i
    , input [addr_width_lp-1:0] addr_i
    , input v_i
    , input w_i
    , output logic [`BSG_SAFE_MINUS(width_p, 1):0]  data_o
    );

   wire clk_lo;

   if (enable_clock_gating_p)
     begin
       bsg_clkgate_optional icg
         (.clk_i( clk_i )
         ,.en_i( v_i )
         ,.bypass_i( 1'b0 )
         ,.gated_clock_o( clk_lo )
         );
     end
   else
     begin
       assign clk_lo = clk_i;
     end

   bsg_mem_1rw_sync_synth
     #(.width_p(width_p)
       ,.els_p(els_p)
       ,.latch_last_read_p(latch_last_read_p)
       ) synth
    (.clk_i( clk_lo )
    ,.reset_i
    ,.data_i
    ,.addr_i
    ,.v_i
    ,.w_i
    ,.data_o
    );

endmodule


// Taken from https://github.com/bespoke-silicon-group/basejump_stl/blob/643b2bd21c7d5e70583432ba87fa578fcb9f3b45/bsg_dataflow/bsg_fifo_1rw_large.v

// MBT
// 11/9/14
//
// a fifo with only one read or write port,
// using a 1RW *synchronous read* ram.
//
// NOTE: read results appear on next cycle
//

module bsg_fifo_1rw_large #(parameter width_p         = -1
                          , parameter els_p           = -1
			  , parameter verbose_p       = 0
                          )
   (input                  clk_i
    , input                reset_i
    , input [width_p-1:0]  data_i
    , input                v_i
    , input                enq_not_deq_i

    // full and empty are richer
    // than ready_enq and ready_deq
    // which could mean just this cycle

    , output full_o
    , output empty_o
    , output [width_p-1:0] data_o
    );

   localparam ptr_width_lp = `BSG_SAFE_CLOG2(els_p);

   logic [ptr_width_lp-1:0]       rd_ptr, wr_ptr;

   logic                          last_op_is_read_r;


   wire mem_we = enq_not_deq_i & v_i;
   wire mem_re = ~enq_not_deq_i & v_i;

   always_ff @(posedge clk_i)
     if (reset_i)
       last_op_is_read_r <= 1;
     else
       if (v_i)
         last_op_is_read_r <= mem_re;

   // empty versus full detection. very nice for
   // one port case. if ptrs match and last op was a read
   // a read, it must be empty; if last op was a write,
   // it must be full.

   wire fifo_empty = (rd_ptr == wr_ptr) & last_op_is_read_r;
   wire fifo_full  = (rd_ptr == wr_ptr) & ~last_op_is_read_r;

   assign full_o  = fifo_full;
   assign empty_o = fifo_empty;

   // synopsys translate_off

   always_ff @(posedge clk_i)
     assert (reset_i
             | ((fifo_full & mem_we) !== 1)
             ) else $error("enque on full fifo");

   always_ff @(posedge clk_i)
     assert (reset_i
             | ((fifo_empty & mem_re) !== 1)
             ) else $error("deque on empty fifo %x %x", fifo_empty, mem_re, v_i, enq_not_deq_i);

   always_ff @(posedge clk_i)
     if (verbose_p)
       if (v_i)
         begin
            if (enq_not_deq_i)
              $display("### %m enq %x onto fifo (r=%x w=%x)",data_i,rd_ptr,wr_ptr);
            else
              $display("### %m deq fifo (r=%x w=%x)",rd_ptr,wr_ptr);
         end


   wire [31:0] num_elements_debug = (fifo_empty
                                     ? 0
                                     : (fifo_full
                                        ? els_p
                                        : (wr_ptr > rd_ptr
                                           ? (wr_ptr - rd_ptr)
                                           : (els_p - (rd_ptr - wr_ptr)))));

   // synopsys translate_on

   bsg_circular_ptr #(.slots_p(els_p)
                      ,.max_add_p(1)
                      ) rd_circ_ptr
     (.clk      (clk_i)
      , .reset_i(reset_i)
      , .add_i  (mem_re)
      , .o      (rd_ptr )
      , .n_o()
      );

   bsg_circular_ptr #(.slots_p(els_p)
                      ,.max_add_p(1)
                      ) wr_circ_ptr
     (.clk      (clk_i  )
      , .reset_i(reset_i)
      , .add_i  (mem_we)
      , .o      (wr_ptr )
      , .n_o()
      );

   bsg_mem_1rw_sync #(.width_p(width_p)
                      ,.els_p(els_p)
                      )
   mem_1srw (.clk_i
             ,.reset_i
             ,.data_i (data_i                   )
             ,.addr_i (mem_we ? wr_ptr : rd_ptr )
             ,.v_i    (v_i                      )
             ,.w_i    (mem_we                   )
             ,.data_o (data_o                   )
             );



endmodule
