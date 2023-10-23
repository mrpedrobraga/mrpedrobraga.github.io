const ID_DQ_MODE = {
  scope: "variable.name",
  begin: '\\$"',
  end: '"',
  contains: [
    { scope: "char.escape", match: "\\\\." },
  ],
};

const STRING_DQ_MODE = {
    scope: "string",
    begin: '"',
    end: '"',
    contains: [
      { scope: "char.escape", match: "\\\\." },
      { scope: "subst", begin: "\\{", end: "\\}" },
    ],
  };
  
  const STRING_SQ_MODE = {
    scope: "string",
    begin: "'",
    end: "'",
    contains: [
      { scope: "char.escape", match: "\\\\." },
      { scope: "subst", begin: "\\{", end: "\\}" },
    ],
  };
  
  const STRING_GROUP = [ID_DQ_MODE, STRING_DQ_MODE, STRING_SQ_MODE];
  
  const EXPRESSION_MODE = [
    { scope: "variable.constant", match: "\\b[A-Z_][A-Z_]+\\b" },
    { scope: "number", match: "\\b[\\d_]+(\\.[\\d+_])?f?\\b" },
    { scope: "number", match: "\\b0x[\\da-zA-Z_]+\\b" },
    { scope: "meta", match: "%%[\\w\\.]+" },
    { scope: "meta", match: "#%[\\w\\.]+" },
    ID_DQ_MODE,
    STRING_DQ_MODE,
    STRING_SQ_MODE,
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
  ];
  
  hljs.registerLanguage("nano", function () {
    return {
      unicodeRegex: true,
      case_insensitive: false,
      keywords: {
        $pattern: /[\w_]+/,
        keyword: `
        if unless then elif elun else loop for while meanwhile until do select some every count across 
        when match default 
        type enum flags slot view signal state struct class trait entity object
        print prompt exit err 
        in of xis is not and but or xor implies 
        constraint implies
        typeof as keyof addrof 
        let alias with shared global import export from use has await 
        fn err_handler get set construct destruct on
        return err continue break discard yield
        test assert 
      `,
        type: `byte bool bitfield uint int bvec2 bvec3 bvec4 uvec2 ivec2 uvec3 ivec3 uvec4 ivec4 float fvec2 fvec3 fvec4 obj 
        string array list list_view list_state addr nothing 
        i8 i16 i32 i64 f32 f64 u8 u16 u32 u64 option Expectation ErrorHandler ParseError 
        `,
        literal: `false true yes no PI TAU self selffn super superfn`,
        punctuation: `( ) [ ] { } < >`,
        operator: `+ - // / * >> << = == != | &`,
      },
      contains: EXPRESSION_MODE,
    };
  });
  