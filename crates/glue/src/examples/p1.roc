platform "example"
    requires {} { main : MyTag }
    exposes []
    packages {}
    imports []
    provides [mainForHost]

MyTag : [
    Ja (Result Str I64),
    Nein (Result I64 Str),
    Vielleicht (List Dec),
    Other { field1 : Str, field2 : U64 },
]

mainForHost : MyTag
mainForHost = main
