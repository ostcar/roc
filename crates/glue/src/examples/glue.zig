// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

const glue = @import("glue");

pub fn RocResult(comptime T: type, comptime E: type) type {
    return extern struct {
        payload: RocResultPayload(T, E),
        tag: RocResultTag,
    };
}

pub fn RocResultPayload(comptime T: type, comptime E: type) type {
    return extern union {
        ok: T,
        err: E,
    };
}

const RocResultTag = enum(u8) {
    RocErr = 0,
    RocOk = 1,
};
pub const RocStr = glue.str.RocStr;

pub const R1 = extern struct {
    field1: *RocStr,
    field2: u64,
};

pub const RocDec = glue.dec.RocDec;

pub const MyTag = enum(c_int) {
    ja: RocResult(*RocStr, i64),
    nein: RocResult(i64, *RocStr),
    other: R1,
    vielleicht: []RocDec,
};
