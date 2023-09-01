hljs.registerLanguage("nano", function () {
  return {
    unicodeRegex: true,
    case_insensitive: false,
    keywords: {
      $pattern: /[\w_]+/,
      keyword: `
      if unless then elif elun else loop for while until do select count reduce 
      when match default 
      slot signal state struct class trait entity object
      print prompt exit err 
      in of and or is 
      typeof as keyof 
      let with shared import use has await 
      fn err_handler get set construct 
      return err continue break discard 
    `,
      type: `byte bool bitfield uint int uvec2 ivec2 uvec3 ivec3 uvec4 ivec4 float fvec2 fvec3 fvec4 obj string array list state_list  
      i8 i32 i64 f32 f64 u8 u32 u64 option Expectation ErrorHandler ParseError 
      `,
      literal: `false true yes no PI TAU self selffn super superfn`,
      punctuation: `( ) [ ] { } < >`,
      operator: `+ - // / * >> << = == != | &`,
    },
    contains: [
      { scope: "variable.constant", match: "\\b[A-Z_]+\\b" },
      { scope: "number", match: "\\b[\\d_]+(\\.[\\d+_])?f?\\b" },
      {
        scope: "string",
        begin: '"',
        end: '"',
        contains: [
          { scope: "char.escape", match: "\\\\\\w" },
          { scope: "subst", begin: "\\{", end: "\\}" },
        ],
      },
      {
        scope: "string",
        begin: "'",
        end: "'",
        contains: [
          { scope: "char.escape", match: "\\\\\\w" },
          { scope: "subst", begin: "\\{", end: "\\}" },
        ],
      },
      {
        scope: "regexp",
        begin: "r/",
        end: "/",
      },
      {
        scope: "regexp",
        begin: "o/",
        end: "/",
      },
      hljs.COMMENT(
        "###", // begin
        "###" // end
      ),
      hljs.COMMENT(
        "#", // begin
        "\n" // end
      ),
    ],
  };
});
