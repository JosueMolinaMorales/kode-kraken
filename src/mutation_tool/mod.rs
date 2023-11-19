mod builder;
mod mutation;
mod operators;
mod tool;

pub use builder::*;
pub use mutation::*;
pub use operators::*;
pub use tool::*;

#[cfg(test)]
pub mod test_util {
    pub const KOTLIN_TEST_CODE: &str = r#"
fun main() {
    // Arithmetic expressions
    val a = 10
    val b = 3
    val c = a + b
    val d = a - b
    val e = a * b
    val f = a / b
    val g = a % b
}
"#;

    pub const KOTLIN_RELATIONAL_TEST_CODE: &str = r#"
fun main() {
    // Relational expressions
    val a = 10
    val b = 3
    val c = a > b
    val d = a < b
    val e = a >= b
    val f = a <= b
    val g = a == b
    val h = a != b
}
"#;

    pub const KOTLIN_LOGICAL_TEST_CODE: &str = r#"
fun main() {
    // Logical expressions
    val a = true
    val b = false
    val c = a && b
    val d = a || b
}
"#;

    pub const KOTLIN_UNARY_TEST_CODE: &str = r#"
var h = 5
h++
h--
++h
--h
"#;

    pub const KOTLIN_UNARY_REMOVAL_TEST_CODE: &str = r#"
var h = 5
h++
h--
++h
--h
val a = !h
val b = -h
val c = +h
"#;

    pub const KOTLIN_ASSIGNMENT_TEST_CODE: &str = r#"
var h = 5
h += 3
h -= 1
h *= 2
h /= 4
h %= 2
"#;

    pub const KOTLIN_ELVIS_TEST_CODE: &str = r#"
fun main() {
    val a = 10
    val b = 3
    val c = a ?: b
}
"#;

    pub const KOTLIN_ELVIS_LITERAL_TEST_CODE: &str = r#"
fun main() {
    val a: String? = null
    val b = a ?: "b"
    val c: Int? = null
    val d = c ?: 1
    val e = c ?: -10
    val f: Boolean? = null
    val g = e ?: true
    val h: Double? = null
    val i = h ?: 2.0
    val j = h ?: -3.0
    val k: Float? = null
    val l = k ?: 4.0f
    val m = k ?: -5.0f
    val n: Long? = null
    val o = n ?: 6L
    val p = n ?: -7L
    val q: Char? = null
    val r = q ?: 'a'
    val s = q ?: 'b'
}
"#;
}
