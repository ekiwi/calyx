use baa::{BitVecOps, BitVecValue};

use crate::{
    flatten::{
        flat_ir::prelude::{AssignedValue, GlobalPortIdx, PortValue},
        primitives::{
            all_defined, comb_primitive, declare_ports, ports,
            prim_trait::UpdateStatus, Primitive,
        },
        structures::environment::PortMap,
    },
    primitives::stateful::floored_division,
};

use super::prim_trait::UpdateResult;

pub struct StdConst {
    value: BitVecValue,
    out: GlobalPortIdx,
}

impl StdConst {
    pub fn new(value: BitVecValue, out: GlobalPortIdx) -> Self {
        Self { value, out }
    }
}

impl Primitive for StdConst {
    fn exec_comb(&self, port_map: &mut PortMap) -> UpdateResult {
        Ok(if port_map[self.out].is_undef() {
            port_map[self.out] = PortValue::new_cell(self.value.clone());
            UpdateStatus::Changed
        } else {
            UpdateStatus::Unchanged
        })
    }

    fn exec_cycle(&mut self, _port_map: &mut PortMap) -> UpdateResult {
        Ok(UpdateStatus::Unchanged)
    }

    fn has_comb(&self) -> bool {
        false
    }

    fn has_stateful(&self) -> bool {
        false
    }
}

pub struct StdMux {
    base: GlobalPortIdx,
}

impl StdMux {
    declare_ports![ COND: 0, TRU: 1, FAL:2, OUT: 3];
    pub fn new(base: GlobalPortIdx) -> Self {
        Self { base }
    }
}

impl Primitive for StdMux {
    fn exec_comb(&self, port_map: &mut PortMap) -> UpdateResult {
        ports![&self.base; cond: Self::COND, tru: Self::TRU, fal: Self::FAL, out: Self::OUT];

        let winning_idx =
            port_map[cond].as_bool().map(|c| if c { tru } else { fal });

        if winning_idx.is_some() && port_map[winning_idx.unwrap()].is_def() {
            Ok(port_map.insert_val(
                out,
                AssignedValue::cell_value(
                    port_map[winning_idx.unwrap()].val().unwrap().clone(),
                ),
            )?)
        } else {
            port_map.write_undef(out)?;
            Ok(UpdateStatus::Unchanged)
        }
    }

    fn has_stateful(&self) -> bool {
        false
    }
}

comb_primitive!(StdNot(input [0]) -> (out [1]) {
    all_defined!(input);
    Ok(Some(input.clone_bit_vec().not().into()))
});

comb_primitive!(StdWire(input [0] ) -> (out [1]) {
    Ok(input.val().cloned())
});

// ===================== Unsigned binary operations ======================
comb_primitive!(StdAdd(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left + right;
    Ok(Some(tr))
});
comb_primitive!(StdSub(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let result = left - right;
    Ok(Some(result))
});

comb_primitive!(StdFpAdd(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left + right;
    Ok(Some(tr))
});

comb_primitive!(StdFpSub(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let result = left - right;
    Ok(Some(result))
});

// ===================== Shift Operations ======================
comb_primitive!(StdLsh[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.shift_left(right);
    //sanity check the widths
    Ok(Some(tr))
});

comb_primitive!(StdRsh[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.shift_right(right);
    //sanity check the widths
    Ok(Some(tr))
});

// ===================== Signed Shift Operations ======================
comb_primitive!(StdSlsh(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.shift_left(right);
    //sanity check the widths
    Ok(Some(tr))

});
comb_primitive!(StdSrsh(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.arithmetic_shift_right(right);
    //sanity check the widths
    Ok(Some(tr))
});
// ===================== Logial Operations ======================
comb_primitive!(StdAnd(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    let result = left.and(right);
    Ok(Some(result))
});
comb_primitive!(StdOr(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    let result = left.or(right);
    Ok(Some(result))
});
comb_primitive!(StdXor(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    let result = left.xor(right);
    Ok(Some(result))
});

// ===================== Comparison Operations ======================
comb_primitive!(StdGt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdLt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_less(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdGe(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater_or_equal(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdLe(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_less_or_equal(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdEq(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    Ok(Some(BitVecValue::from_bool(left.is_equal(right))))
});
comb_primitive!(StdNeq(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    Ok(Some(BitVecValue::from_bool(left.is_not_equal(right))))
});

// ===================== Signed Comparison Operations ======================
comb_primitive!(StdSgt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdSlt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_less_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdSge(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater_or_equal_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdSle(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_less_or_equal_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});
comb_primitive!(StdSeq(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    Ok(Some(BitVecValue::from_bool(left.is_equal(right))))
});
comb_primitive!(StdSneq(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    Ok(Some(BitVecValue::from_bool(left.is_not_equal(right))))
});

// ===================== Unsigned FP Comparison Operators ======================
comb_primitive!(StdFpGt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});

// ===================== Signed FP Comparison Operators ======================
comb_primitive!(StdFpSgt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_greater_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});

comb_primitive!(StdFpSlt(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);
    let tr = left.is_less_signed(right);
    Ok(Some(BitVecValue::from_bool(tr)))
});

// ===================== Resizing Operations ======================
comb_primitive!(StdSlice[OUT_WIDTH](input [0]) -> (out [1]) {
    all_defined!(input);

    Ok( Some(input.truncate(OUT_WIDTH as usize)))
});
comb_primitive!(StdPad[OUT_WIDTH](input [0]) -> (out [1]) {
    all_defined!(input);

    Ok( Some(input.ext(OUT_WIDTH as usize)))
});

comb_primitive!(StdCat(left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok(Some(Value::concat(left, right)))
});

// ===================== Unsynthesizeable Operations ======================
comb_primitive!(StdUnsynMult[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(Value::from(left.as_unsigned() * right.as_unsigned(), WIDTH)))
});

comb_primitive!(StdUnsynDiv[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(Value::from(left.as_unsigned() / right.as_unsigned(), WIDTH)))
});

comb_primitive!(StdUnsynSmult[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(Value::from(left.as_signed() * right.as_signed(), WIDTH)))
});

comb_primitive!(StdUnsynSdiv[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(Value::from(left.as_signed() / right.as_signed(), WIDTH)))
});

comb_primitive!(StdUnsynMod[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(Value::from(left.as_unsigned() % right.as_unsigned(), WIDTH)))
});

comb_primitive!(StdUnsynSmod[WIDTH](left [0], right [1]) -> (out [2]) {
    all_defined!(left, right);

    Ok( Some(BitVecValue::from_big_int(left.to_big_int() - right.to_big_int() * floored_division(
            &left.to_big_int(),
            &right.to_big_int()), WIDTH)))
});
