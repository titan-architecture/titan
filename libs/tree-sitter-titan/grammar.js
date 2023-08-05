module.exports = grammar({
  name: "titan",
  rules: {
    source_file: $ => repeat($._statement),

    _statement: ($) => choice($.variable_definition),
    variable_definition: ($) =>
      seq(
        $._variable_start,
        field("pattern", $.alpha_identifier),
        optional(seq(":", field("type", $._type))),
        "=",
        field("value", $._non_null_literal)
      ),

    // TODO: Declare without assign
    variable_declaration: ($) => seq(),

    alpha_identifier: () =>
      /[A-Za-z\$_][A-Z\$_a-z0-9]*(_[\-!#%&*+\/\\:<=>?@\u005e\u007c~]+)?/,

    // TODO: Add annotations and modifiers and stuff
    _variable_start: ($) => seq("let"),

    // TODO: Add other types, maybe function/lambda
    _type: ($) => choice("string", "int", "bool"), // choice($.literal_type),

    // TODO: Add other types, string, float, etc
    _non_null_literal: ($) => choice($.string, $.integer_literal),

    // TODO: Replace with choice(simple string and multi line string)
    string: ($) => seq('"', $.alpha_identifier, '"'),

    integer_literal: ($) =>
      token(
        seq(
          optional(/[-]/), //negative
          choice(/[\d](_?\d)*/, /0[xX][\da-fA-F](_?[\da-fA-F])*/), // digit or 0xdigits
          optional(/[lL]/) // long
        )
      ),

    literal_type: ($) => $._non_null_literal,
  },
});
