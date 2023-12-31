module.exports = grammar({
  name: "titan",
  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice(
      $.variable_definition,
      $.print_statement,
    ),

    print_statement: ($) => seq("print", "(", field("identifier", $.alpha_identifier), ")"),

    variable_definition: ($) =>
      seq(
        $._variable_start,
        field("pattern", $.alpha_identifier),
        optional(seq(":", field("type", $._type_annotation))),
        "=",
        field("value", $._non_null_literal)
      ),

    _type_annotation: ($) => choice(
      "bool",
      "string",
      "int"
    ),

    // TODO: Declare without assign
    variable_declaration: ($) => seq(),

    alpha_identifier: () =>
      /[A-Za-z\$_][A-Z\$_a-z0-9]*(_[\-!#%&*+\/\\:<=>?@\u005e\u007c~]+)?/,

    // TODO: Add annotations and modifiers and stuff
    _variable_start: ($) => seq("let"),

    // TODO: Add other types, maybe function/lambda
    _type: ($) => choice($.literal_type),

    // TODO: Add other types, string, float, etc
    _non_null_literal: ($) => choice($.string, $.integer_literal, $.boolean_literal),

    // TODO: Replace with choice(simple string and multi line string)
    string: ($) => seq('"', choice($.alpha_identifier, $.integer_literal), '"'),

    boolean_literal: ($) => choice("true", "false"),

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
