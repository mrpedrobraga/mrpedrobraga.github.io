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
  { scope: "meta", match: "@[\\w\\.]+" },
  ID_DQ_MODE,
  STRING_DQ_MODE,
  STRING_SQ_MODE,
  {
    scope: "regexp",
    begin: "r/",
    end: "/",
  },
  {
    scope: "macro",
    begin: "#@",
    end: "\n",
  },
  {
    scope: "macro",
    begin: "##@",
    end: "@##",
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
        if unless then elif elun else loop for while meanwhile until do exists some every  
        where match default 
        primitive type variants flags slot view signal state struct trait object 
        print push_error warn write prompt exit err sleep 
        in of xis is not and but or xor 
        constraint infer 
        typeof as keyof addrof 
        on buffer stack queue let alias with shared global import export from use has async
        fn impl 
        return err continue break discard yield 
        test assert 
      `,
      type: `any unknown byte bool bitfield uint int bvec2 bvec3 bvec4 uvec2 ivec2 uvec3 ivec3 uvec4 ivec4 float fvec2 fvec3 fvec4 obj 
        string array list list_view list_state addr nothing 
        i8 i16 i32 i64 f32 f64 u8 u16 u32 u64 option Expectation ParseError 
        `,
      literal: `false true yes no PI TAU self selffn super superfn`,
      punctuation: `( ) [ ] { } < >`,
      operator: `+ - // / * >> << = == != | &`,
    },
    contains: EXPRESSION_MODE,
  };
});
