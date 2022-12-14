use crate::modules::protos::test_proto2::NestedProto2;
use crate::modules::protos::test_proto2::Test;

use crate::scanner::ScanContext;
use yara_macros::module_main;

//#[member_of(Submessage)]
pub(crate) fn sum(a: i64, b: i64) -> i64 {
    a + b
}

#[module_main]
fn main(_ctx: &ScanContext) -> Test {
    let mut test = Test::new();

    test.set_int32_zero(0);
    test.set_int64_zero(0);
    test.set_sint32_zero(0);
    test.set_sint64_zero(0);
    test.set_uint32_zero(0);
    test.set_uint64_zero(0);
    test.set_fixed32_zero(0);
    test.set_fixed64_zero(0);
    test.set_sfixed32_zero(0);
    test.set_sfixed64_zero(0);
    test.set_float_zero(0.0);

    test.set_int32_one(1);
    test.set_int64_one(1);
    test.set_sint32_one(1);
    test.set_sint64_one(1);
    test.set_uint32_one(1);
    test.set_uint64_one(1);
    test.set_fixed32_one(1);
    test.set_fixed64_one(1);
    test.set_sfixed32_one(1);
    test.set_sfixed64_one(1);
    test.set_float_one(1.0);

    test.set_string_foo("foo".to_string());
    test.set_string_bar("bar".to_string());

    test.set_bytes_foo("foo".as_bytes().to_vec());
    test.set_bytes_bar("bar".as_bytes().to_vec());

    test.array_int64.push(1);
    test.array_int64.push(10);
    test.array_int64.push(100);

    test.array_float.push(1.0);
    test.array_float.push(10.0);
    test.array_float.push(100.0);

    test.array_bool.push(false);
    test.array_bool.push(true);

    test.array_string.push("foo".to_string());
    test.array_string.push("bar".to_string());
    test.array_string.push("baz".to_string());

    let mut nested = NestedProto2::new();

    nested.set_nested_int32_zero(0);
    nested.set_nested_int64_zero(0);
    nested.set_nested_int32_one(1);
    nested.set_nested_int64_one(1);

    test.nested = Some(nested).into();

    let mut nested = NestedProto2::new();

    nested.set_nested_int32_zero(0);
    nested.set_nested_int64_zero(0);
    nested.set_nested_int32_one(1);
    nested.set_nested_int64_one(1);

    test.map_string_struct.insert("foo".to_string(), nested);
    test.map_string_int64.insert("one".to_string(), 1);

    test
}
